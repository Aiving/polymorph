use crate::{
    geometry::DISTANCE_EPSILON,
    util::{positive_modulo, progress_distance, progress_in_range},
};

pub struct DoubleMapper {
    source_values: Vec<f32>,
    target_values: Vec<f32>,
}

fn validate_progress(p: &[f32]) {
    let mut prev = p.last().copied().unwrap_or_default();
    let mut wraps = 0;

    for i in 0..p.len() {
        let curr = p[i];

        assert!((0.0..1.0).contains(&curr), "FloatMapping - Progress outside of range: {p:?}");
        assert!(
            progress_distance(curr, prev) > DISTANCE_EPSILON,
            "FloatMapping - Progress repeats a value: {p:?}"
        );

        if curr < prev {
            wraps += 1;

            assert!(wraps <= 1, "FloatMapping - Progress wraps more than once: {p:?}");
        }

        prev = curr;
    }
}

impl DoubleMapper {
    pub fn identity() -> Self {
        Self::new([(0.0, 0.0), (0.5, 0.5)])
    }

    pub fn new<T: IntoIterator<Item = (f32, f32)>>(mappings: T) -> Self {
        let (source_values, target_values): (Vec<_>, Vec<_>) = mappings.into_iter().unzip();

        // Both source values and target values should be monotonically increasing, with
        // the exception of maybe one time (since progress wraps around).
        validate_progress(&source_values);
        validate_progress(&target_values);

        Self { source_values, target_values }
    }

    pub fn map(&self, x: f32) -> f32 {
        linear_map(&self.source_values, &self.target_values, x)
    }

    pub fn map_back(&self, x: f32) -> f32 {
        linear_map(&self.target_values, &self.source_values, x)
    }
}

fn linear_map(x_values: &[f32], y_values: &[f32], x: f32) -> f32 {
    assert!((0.0..=1.0).contains(&x), "Invalid progress: {x}");

    let segment_start_index = (0..x_values.len())
        .find(|&it| progress_in_range(x, x_values[it], x_values[(it + 1) % x_values.len()]))
        .unwrap_or_default();

    let segment_end_index = (segment_start_index + 1) % x_values.len();
    let segment_size_x = positive_modulo(x_values[segment_end_index] - x_values[segment_start_index], 1.0);
    let segment_size_y = positive_modulo(y_values[segment_end_index] - y_values[segment_start_index], 1.0);
    let position_in_segment = if segment_size_x < 0.001 {
        0.5
    } else {
        positive_modulo(x - x_values[segment_start_index], 1.0) / segment_size_x
    };

    positive_modulo(segment_size_y.mul_add(position_in_segment, y_values[segment_start_index]), 1.0)
}
