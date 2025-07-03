use crate::{
    DoubleMapper, Feature,
    geometry::{DISTANCE_EPSILON, Point},
    measured_polygon::ProgressableFeature,
    util::{progress_distance, progress_in_range},
};

#[derive(Default)]
struct MappingHelper<'a> {
    mapping: Vec<(f32, f32)>,

    used_f1: Vec<&'a ProgressableFeature>,
    used_f2: Vec<&'a ProgressableFeature>,
}

impl<'a> MappingHelper<'a> {
    fn add_mapping(&mut self, f1: &'a ProgressableFeature, f2: &'a ProgressableFeature) {
        // We don't want to map the same feature twice.
        if self.used_f1.contains(&f1) || self.used_f2.contains(&f2) {
            return;
        }

        // Ret is sorted, find where we need to insert this new mapping.
        if let Err(insertion_index) = self.mapping.binary_search_by(|it| it.0.total_cmp(&f1.progress)) {
            let n = self.mapping.len();

            // We can always add the first 1 element
            if n >= 1 {
                let (before1, before2) = self.mapping[(insertion_index + n - 1) % n];
                let (after1, after2) = self.mapping[insertion_index % n];

                // We don't want features that are way too close to each other, that will make
                // the DoubleMapper unstable
                if progress_distance(f1.progress, before1) < DISTANCE_EPSILON
                    || progress_distance(f1.progress, after1) < DISTANCE_EPSILON
                    || progress_distance(f2.progress, before2) < DISTANCE_EPSILON
                    || progress_distance(f2.progress, after2) < DISTANCE_EPSILON
                {
                    return;
                }

                // When we have 2 or more elements, we need to ensure we are not adding extra
                // crossings.
                if n > 1 && !progress_in_range(f2.progress, before2, after2) {
                    return;
                }
            }

            // All good, we can add the mapping.
            self.mapping.insert(insertion_index, (f1.progress, f2.progress));
            self.used_f1.push(f1);
            self.used_f2.push(f2);
        } else {
            panic!("There can't be two features with the same progress");
        }
    }
}

impl DoubleMapper {
    pub fn from_features(features1: &[ProgressableFeature], features2: &[ProgressableFeature]) -> Self {
        // We only use corners for this mapping.

        let mut filtered_features1 = Vec::new();
        let mut filtered_features2 = Vec::new();

        for feature in features1 {
            if feature.feature.is_corner() {
                filtered_features1.push(feature);
            }
        }

        for feature in features2 {
            if feature.feature.is_corner() {
                filtered_features2.push(feature);
            }
        }

        let mut distance_vertex_list = Vec::new();

        for f1 in filtered_features1 {
            for &f2 in &filtered_features2 {
                let distance = feature_dist_squared(&f1.feature, &f2.feature);

                if distance != f32::MAX {
                    distance_vertex_list.push(DistanceVertex { distance, f1, f2 });
                }
            }
        }

        distance_vertex_list.sort_by(|a, b| a.distance.total_cmp(&b.distance));

        // Special cases.
        Self::new(match distance_vertex_list.len() {
            0 => vec![(0.0, 0.0), (0.5, 0.5)],
            1 => {
                let f1 = distance_vertex_list[0].f1.progress;
                let f2 = distance_vertex_list[0].f2.progress;

                vec![(f1, f2), ((f1 + 0.5) % 1.0, (f2 + 0.5) % 1.0)]
            }
            _ => {
                let mut helper = MappingHelper::default();

                for dv in distance_vertex_list {
                    helper.add_mapping(dv.f1, dv.f2);
                }

                helper.mapping
            }
        })
    }
}

struct DistanceVertex<'a> {
    distance: f32,
    f1: &'a ProgressableFeature,
    f2: &'a ProgressableFeature,
}

fn feature_representative_point(feature: &Feature) -> Point {
    (feature.cubics[0].anchor0() + feature.cubics[feature.cubics.len() - 1].anchor1().to_vector()) / 2.0
}

fn feature_dist_squared(f1: &Feature, f2: &Feature) -> f32 {
    if f1.is_corner_and(|f1_convex| f2.is_corner_and(|f2_convex| f1_convex != f2_convex)) {
        // Simple hack to force all features to map only to features of the same
        // concavity, by returning an infinitely large distance in that case
        f32::MAX
    } else {
        (feature_representative_point(f1) - feature_representative_point(f2)).square_length()
    }
}
