use crate::Cubic;

pub trait Measurer {
    /// Returns size of given cubic, according to however the implementation
    /// wants to measure the size (angle, length, etc). It has to be greater
    /// or equal to 0.
    fn measure_cubic(&self, c: &Cubic) -> f32;

    /// Given a cubic and a measure that should be between 0 and the value
    /// returned by measureCubic (If not, it will be capped), finds the
    /// parameter t of the cubic at which that measure is reached.
    fn find_cubic_cut_point(&self, c: &Cubic, m: f32) -> f32;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LengthMeasurer;

impl LengthMeasurer {
    const SEGMENTS: usize = 3;

    fn closest_progress_to(cubic: &Cubic, threshold: f32) -> (f32, f32) {
        let mut total = 0.0;
        let mut remainder = threshold;
        let mut prev = cubic.anchor0();

        for i in 1..=Self::SEGMENTS {
            let progress = i as f32 / Self::SEGMENTS as f32;

            let point = cubic.point_on_curve(progress);
            let segment = (point - prev).length();

            if segment >= remainder {
                return (progress - (1.0 - remainder / segment) / Self::SEGMENTS as f32, threshold);
            }

            remainder -= segment;
            total += segment;
            prev = point;
        }

        (1.0, total)
    }
}

impl Measurer for LengthMeasurer {
    fn measure_cubic(&self, c: &Cubic) -> f32 {
        Self::closest_progress_to(c, f32::INFINITY).1
    }

    fn find_cubic_cut_point(&self, c: &Cubic, m: f32) -> f32 {
        Self::closest_progress_to(c, m).0
    }
}
