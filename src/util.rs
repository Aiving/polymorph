use crate::geometry::Vector;

pub fn progress_in_range(progress: f32, progress_from: f32, progress_to: f32) -> bool {
    if progress_to >= progress_from {
        (progress_from..=progress_to).contains(&progress)
    } else {
        progress >= progress_from || progress <= progress_to
    }
}

pub fn progress_distance(p1: f32, p2: f32) -> f32 {
    let value = (p1 - p2).abs();

    value.min(1.0 - value)
}

pub fn radial_to_cartesian(radius: f32, angle_radians: f32) -> Vector {
    Vector::new(angle_radians.cos(), angle_radians.sin()) * radius
}
