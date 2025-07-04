use crate::{Cubic, Feature, Measurer, RoundedPolygon, geometry::DISTANCE_EPSILON, util::positive_modulo};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeasuredCubic {
    pub(crate) cubic: Cubic,
    pub(crate) start_outline_progress: f32,
    pub(crate) end_outline_progress: f32,
    pub(crate) measured_size: f32,
}

impl MeasuredCubic {
    fn new<T: Measurer>(measurer: &T, cubic: Cubic, start_outline_progress: f32, end_outline_progress: f32) -> Self {
        let size = measurer.measure_cubic(&cubic);

        Self {
            cubic,
            start_outline_progress,
            end_outline_progress,
            measured_size: size,
        }
    }

    pub(crate) fn cut_at_progress<T: Measurer>(self, measurer: &T, cut_outline_progress: f32) -> (Self, Self) {
        // Floating point errors further up can cause cut_outline_progress to land just
        // slightly outside of the start/end progress for this cubic, so we limit it
        // to those bounds to avoid further errors later
        let bounded_cut_outline_progress = cut_outline_progress.clamp(self.start_outline_progress, self.end_outline_progress);
        let outline_progress_size = self.end_outline_progress - self.start_outline_progress;
        let progress_from_start = bounded_cut_outline_progress - self.start_outline_progress;

        let relative_progress = progress_from_start / outline_progress_size;
        let t = measurer.find_cubic_cut_point(&self.cubic, relative_progress * self.measured_size);

        assert!((0.0..=1.0).contains(&t), "Cubic cut point is expected to be between 0 and 1");

        // c1/c2 are the two new cubics, then we return (MeasuredCubic, MeasuredCubic)
        // created from them
        let (c1, c2) = self.cubic.split(t);

        (
            Self::new(measurer, c1, self.start_outline_progress, bounded_cut_outline_progress),
            Self::new(measurer, c2, bounded_cut_outline_progress, self.end_outline_progress),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressableFeature {
    pub progress: f32,
    pub feature: Feature,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeasuredPolygon<T: Measurer> {
    pub(crate) measurer: T,
    pub(crate) cubics: Vec<MeasuredCubic>,
    pub(crate) features: Vec<ProgressableFeature>,
}

impl<T: Measurer> MeasuredPolygon<T> {
    pub fn new(measurer: T, features: Vec<ProgressableFeature>, cubics: &[Cubic], outline_progress: &[f32]) -> Self {
        let mut measured_cubics = <Vec<MeasuredCubic>>::new();
        let mut start_outline_progress = 0.0;

        for index in 0..cubics.len() {
            // Filter out "empty" cubics
            if (outline_progress[index + 1] - outline_progress[index]) > DISTANCE_EPSILON {
                measured_cubics.push(MeasuredCubic::new(
                    &measurer,
                    cubics[index],
                    start_outline_progress,
                    outline_progress[index + 1],
                ));

                // The next measured cubic will start exactly where this one ends.
                start_outline_progress = outline_progress[index + 1];
            }
        }

        // We could have removed empty cubics at the end. Ensure the last measured cubic
        // ends at 1f
        let i = measured_cubics.len() - 1;

        measured_cubics[i].end_outline_progress = 1.0;

        Self {
            measurer,
            cubics: measured_cubics,
            features,
        }
    }

    pub fn measure_polygon(measurer: T, polygon: &RoundedPolygon) -> Self {
        let mut cubics = <Vec<Cubic>>::new();
        let mut feature_to_cubic = <Vec<(&Feature, usize)>>::new();

        // Get the cubics from the polygon, at the same time, extract the features and
        // keep a reference to the representative cubic we will use.
        for feature_index in 0..polygon.features.len() {
            let feature = &polygon.features[feature_index];

            for cubic_index in 0..feature.cubics.len() {
                if feature.is_corner() && cubic_index == feature.cubics.len() / 2 {
                    feature_to_cubic.push((feature, cubics.len()));
                }

                cubics.push(feature.cubics[cubic_index]);
            }
        }

        let measure_results = cubics.iter().fold(vec![0.0], |mut measure, cubic| {
            measure.push(measure[measure.len() - 1] + measurer.measure_cubic(cubic));

            measure
        });

        let total_measure = measure_results[measure_results.len() - 1];

        let mut outline_progress = Vec::with_capacity(measure_results.len());

        for measure in measure_results {
            outline_progress.push(measure / total_measure);
        }

        let mut features = Vec::new();

        for (feature, ix) in feature_to_cubic {
            features.push(ProgressableFeature {
                progress: positive_modulo(outline_progress[ix].midpoint(outline_progress[ix + 1]), 1.0),
                feature: feature.clone(),
            });
        }

        Self::new(measurer, features, &cubics, &outline_progress)
    }

    /// # Panics
    ///
    /// May panic if `cutting_point` is outside of `0.0..=1.0`.
    #[must_use]
    pub fn cut_and_shift(self, cutting_point: f32) -> Self {
        assert!((0.0..=1.0).contains(&cutting_point), "Cutting point is expected to be between 0 and 1");

        if cutting_point < DISTANCE_EPSILON {
            return self;
        }

        // Find the index of cubic we want to cut
        let target_index = self
            .cubics
            .iter()
            .position(|it| (it.start_outline_progress..=it.end_outline_progress).contains(&cutting_point))
            .unwrap_or_default();

        // Cut the target cubic.
        // b1, b2 are two resulting cubics after cut
        let (b1, b2) = self.cubics[target_index].cut_at_progress(&self.measurer, cutting_point);

        // Construct the list of the cubics we need:
        // * The second part of the target cubic (after the cut)
        // * All cubics after the target, until the end + All cubics from the start,
        //   before the target cubic
        // * The first part of the target cubic (before the cut)
        let mut ret_cubics = vec![b2.cubic];

        for i in 1..self.cubics.len() {
            ret_cubics.push(self.cubics[(i + target_index) % self.cubics.len()].cubic);
        }

        ret_cubics.push(b1.cubic);

        // Construct the array of outline progress.
        // For example, if we have 3 cubics with outline progress [0 .. 0.3], [0.3 ..
        // 0.8] & [0.8 .. 1.0], and we cut + shift at 0.6:
        // 0. 0123456789 |--|--/-|-|
        // The outline progresses will start at 0 (the cutting point, that shifs to
        // 0.0), then 0.8 - 0.6 = 0.2, then 1 - 0.6 = 0.4, then 0.3 - 0.6 + 1 =
        // 0.7, then 1 (the cutting point again),
        // all together: (0.0, 0.2, 0.4, 0.7, 1.0)
        let mut ret_outline_progress = Vec::with_capacity(self.cubics.len() + 2);

        for index in 0..(self.cubics.len() + 2) {
            ret_outline_progress.push(match index {
                0 => 0.0,
                v if v == self.cubics.len() + 1 => 1.0,
                _ => {
                    let cubic_index = (target_index + index - 1) % self.cubics.len();

                    positive_modulo(self.cubics[cubic_index].end_outline_progress - cutting_point, 1.0)
                }
            });
        }

        // Shift the feature's outline progress too.
        let mut new_features = Vec::new();

        for i in 0..self.features.len() {
            new_features.push(ProgressableFeature {
                progress: positive_modulo(self.features[i].progress - cutting_point, 1.0),
                feature: self.features[i].feature.clone(),
            });
        }

        // Filter out all empty cubics (i.e. start and end anchor are (almost) the same
        // point.)
        Self::new(self.measurer, new_features, &ret_cubics, &ret_outline_progress)
    }
}
