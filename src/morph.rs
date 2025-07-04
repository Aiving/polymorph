use core::f32;

use crate::{
    Cubic, DoubleMapper, MeasuredPolygon, RoundedPolygon,
    geometry::ANGLE_EPSILON,
    measurer::LengthMeasurer,
    path::{PathBuilder, add_cubics},
    util::positive_modulo,
};

/// A structure designed to obtain transition cubics between the start and end
/// [`RoundedPolygon`]s.
#[derive(Debug, Clone, PartialEq)]
pub struct Morph {
    start: RoundedPolygon,
    end: RoundedPolygon,
    r#match: Vec<(Cubic, Cubic)>,
}

impl Morph {
    /// Matches the [`Cubic`]s of the start and end [`RoundedPolygon`]s, then
    /// returns an instance of [`Morph`].
    ///
    /// # Panics
    ///
    /// May panic if not all cubics of both polygons have been matched.
    pub fn new(start: RoundedPolygon, end: RoundedPolygon) -> Self {
        let r#match = Self::match_morph(&start, &end);

        Self { start, end, r#match }
    }

    /// Returns the transition state between the start and end polygons at a
    /// given `progress` value represented as a list of [`Cubic`]s.
    pub fn as_cubics(&self, progress: f32) -> Vec<Cubic> {
        let mut cubics = Vec::new();

        // The first/last mechanism here ensures that the final anchor point in the
        // shape exactly matches the first anchor point. There can be rendering
        // artifacts introduced by those points being slightly off, even by much
        // less than a pixel
        let mut first_cubic: Option<Cubic> = None;
        let mut last_cubic: Option<Cubic> = None;

        for i in 0..self.r#match.len() {
            let cubic = Cubic::from_fn(|it| self.r#match[i].0.points[it].lerp(self.r#match[i].1.points[it], progress));

            if first_cubic.is_none() {
                first_cubic.replace(cubic);
            }

            if let Some(cubic) = last_cubic {
                cubics.push(cubic);
            }

            last_cubic.replace(cubic);
        }

        #[allow(clippy::collapsible_if)] // For MSRV compability
        if let Some(last_cubic) = last_cubic {
            if let Some(first_cubic) = first_cubic {
                cubics.push(Cubic::new(
                    last_cubic.anchor0(),
                    last_cubic.control0(),
                    last_cubic.control1(),
                    first_cubic.anchor0(),
                ));
            }
        }

        cubics
    }

    /// Returns a path with a drawn transition state (based on the provided
    /// `progress`). Path is created using the provided `T`, which should
    /// implement `PathBuilder` and `Default` traits.
    pub fn as_path<T: PathBuilder + Default>(&self, progress: f32, repeat_path: bool, close_path: bool) -> T::Path {
        let mut path = T::default();

        self.add_to(progress, &mut path, repeat_path, close_path);

        path.build()
    }

    /// Adds a transition state (based on the provided `progress`) to the
    /// `builder`.
    pub fn add_to<T: PathBuilder>(&self, progress: f32, builder: &mut T, repeat_path: bool, close_path: bool) {
        let cubics = self.as_cubics(progress);

        add_cubics(builder, repeat_path, close_path, &cubics);
    }

    fn match_morph(p1: &RoundedPolygon, p2: &RoundedPolygon) -> Vec<(Cubic, Cubic)> {
        // Measure polygons, returns lists of measured cubics for each polygon, which
        // we then use to match start/end curves
        let measured_polygon1 = MeasuredPolygon::measure_polygon(LengthMeasurer, p1);
        let measured_polygon2 = MeasuredPolygon::measure_polygon(LengthMeasurer, p2);

        println!(
            "[{}]",
            measured_polygon1.features.iter().fold(String::new(), |mut data, feature| {
                if !data.is_empty() {
                    data.push_str(", ");
                }

                data.push_str(&feature.to_string());

                data
            })
        );

        // features1 and 2 will contain the list of corners (just the inner circular
        // curve) along with the progress at the middle of those corners. These
        // measurement values are then used to compare and match between the two
        // polygons
        let features1 = &measured_polygon1.features;
        let features2 = &measured_polygon2.features;

        let double_mapper = DoubleMapper::from_features(features1, features2);

        let polygon2_cut_point = double_mapper.map(0.0);

        let bs1 = measured_polygon1;
        let bs2 = measured_polygon2.cut_and_shift(polygon2_cut_point);

        // Match
        // Now we can compare the two lists of measured cubics and create a list of
        // pairs of cubics [ret], which are the start/end curves that represent
        // the Morph object and the start and end shapes, and which can be
        // interpolated to animate the between those shapes.
        let mut ret = <Vec<(Cubic, Cubic)>>::new();
        // i1/i2 are the indices of the current cubic on the start (1) and end (2)
        // shapes
        let mut i1 = 0;
        let mut i2 = 0;
        // b1, b2 are the current measured cubic for each polygon
        let mut b1 = bs1.cubics.get(i1).copied();

        i1 += 1;

        let mut b2 = bs2.cubics.get(i2).copied();

        i2 += 1;

        // Iterate until all curves are accounted for and matched
        while let (Some(bb1), Some(bb2)) = (b1.take(), b2.take()) {
            // Progresses are in shape1's perspective
            // b1a, b2a are ending progress values of current measured cubics in 0..=1 range
            let b1a = if i1 == bs1.cubics.len() { 1.0 } else { bb1.end_outline_progress };
            let b2a = if i2 == bs2.cubics.len() {
                1.0
            } else {
                positive_modulo(double_mapper.map_back(bb2.end_outline_progress + polygon2_cut_point), 1.0)
            };
            let minb = b1a.min(b2a);

            println!("{b1a} {b2a} | {minb}");

            // min b is the progress at which the curve that ends first ends.
            // If both curves ends roughly there, no cutting is needed, we have a match.
            // If one curve extends beyond, we need to cut it.
            let (seg1, newb1) = if b1a > minb + ANGLE_EPSILON {
                let (a, b) = bb1.cut_at_progress(&bs1.measurer, minb);

                (a, Some(b))
            } else {
                let value = bs1.cubics.get(i1).copied();

                i1 += 1;

                (bb1, value)
            };

            let (seg2, newb2) = if b2a > minb + ANGLE_EPSILON {
                let (a, b) = bb2.cut_at_progress(&bs2.measurer, positive_modulo(double_mapper.map(minb) - polygon2_cut_point, 1.0));

                (a, Some(b))
            } else {
                let value = bs2.cubics.get(i2).copied();

                i2 += 1;

                (bb2, value)
            };

            ret.push((seg1.cubic, seg2.cubic));

            b1 = newb1;
            b2 = newb2;
        }

        assert!(b1.is_none() && b2.is_none(), "Expected both Polygon's Cubic to be fully matched");

        println!(
            "[{}]",
            ret.iter().fold(String::new(), |mut data, cubic| {
                if !data.is_empty() {
                    data.push_str(", ");
                }

                data.push('(');
                data.push_str(&cubic.0.to_string());
                data.push_str(", ");
                data.push_str(&cubic.1.to_string());
                data.push(')');

                data
            })
        );

        ret
    }
}
