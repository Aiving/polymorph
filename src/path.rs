#[cfg(feature = "lyon")]
use lyon_tessellation::{
    geom::traits::Transformation,
    math::Vector,
    path::{
        builder::NoAttributes,
        traits::{Build, PathBuilder as LyonPathBuilder},
    },
};

use crate::{
    Cubic,
    geometry::{Angle, Matrix3, Point},
};

pub trait PathBuilder<P> {
    fn rewind(&mut self);
    fn move_to(&mut self, point: Point);
    fn line_to(&mut self, point: Point);
    fn cubic_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point);
    fn transformed(self, transform: Matrix3) -> impl PathBuilder<P>;
    fn close(&mut self);
    fn build(self) -> P;
}

pub fn path_from_cubics<P, T: PathBuilder<P>>(
    mut path: T,
    start_angle: f32,
    repeat_path: bool,
    close_path: bool,
    cubics: &[Cubic],
    rotation_pivot: Point,
) -> impl PathBuilder<P> {
    let mut first = true;
    let mut first_cubic: Option<Cubic> = None;

    path.rewind();

    for it in cubics {
        if first {
            path.move_to(it.anchor0());

            if start_angle != 0.0 {
                first_cubic.replace(*it);
            }

            first = false;
        }

        path.cubic_to(it.control0(), it.control1(), it.anchor1());
    }

    if repeat_path {
        let mut first_in_repeat = true;

        for it in cubics {
            if first_in_repeat {
                path.line_to(it.anchor0());

                first_in_repeat = false;
            }

            path.cubic_to(it.control0(), it.control1(), it.anchor1());
        }
    }

    if close_path {
        path.close();
    }

    if start_angle != 0.0 && first_cubic.is_some() {
        let angle_to_first_cubic = (cubics[0].anchor0() - rotation_pivot).angle_from_x_axis();

        // Rotate the Path to to start from the given angle.
        path.transformed(Matrix3::rotation(0.0, 0.0, 1.0, -angle_to_first_cubic + Angle::radians(start_angle)))
    } else {
        path.transformed(Matrix3::identity())
    }
}

#[cfg(feature = "lyon")]
impl<T: LyonPathBuilder + Build> PathBuilder<T::PathType> for NoAttributes<T> {
    fn rewind(&mut self) {}

    fn move_to(&mut self, point: Point) {
        self.begin(Point::new(point.x, point.y));
    }

    fn line_to(&mut self, point: Point) {
        self.line_to(Point::new(point.x, point.y));
    }

    fn cubic_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point) {
        self.cubic_bezier_to(Point::new(ctrl1.x, ctrl1.y), Point::new(ctrl2.x, ctrl2.y), Point::new(to.x, to.y));
    }

    fn close(&mut self) {
        self.end(true);
    }

    fn transformed(self, transform: Matrix3) -> impl PathBuilder<T::PathType> {
        Self::transformed(self, Transform(transform))
    }

    fn build(self) -> T::PathType {
        Self::build(self)
    }
}

#[cfg(feature = "lyon")]
pub struct Transform(Matrix3);

#[cfg(feature = "lyon")]
impl Transformation<f32> for Transform {
    fn transform_point(&self, p: Point) -> Point {
        self.0.transform_point2d(p).unwrap_or(p)
    }

    fn transform_vector(&self, v: Vector) -> Vector {
        self.0.transform_vector2d(v)
    }
}

#[cfg(feature = "skia")]
impl PathBuilder<skia_safe::Path> for skia_safe::PathBuilder {
    fn rewind(&mut self) {
        self.reset();
    }

    fn move_to(&mut self, point: Point) {
        self.move_to((point.x, point.y));
    }

    fn line_to(&mut self, point: Point) {
        self.line_to((point.x, point.y));
    }

    fn cubic_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point) {
        self.cubic_to((ctrl1.x, ctrl1.y), (ctrl2.x, ctrl2.y), (to.x, to.y));
    }

    fn close(&mut self) {
        self.close();
    }

    fn transformed(mut self, transform: Matrix3) -> impl PathBuilder<skia_safe::Path> {
        let mut matrix = skia_safe::Matrix::new_identity();

        let scale_x = transform.m11;
        let skew_y = transform.m12;
        let persp0 = transform.m14;
        let skew_x = transform.m21;
        let scale_y = transform.m22;
        let persp1 = transform.m24;

        let translate_x = transform.m41;
        let translate_y = transform.m42;
        let persp2 = transform.m44;

        matrix.set_all(scale_x, skew_x, translate_x, skew_y, scale_y, translate_y, persp0, persp1, persp2);

        self.transform(&matrix, None);

        self
    }

    fn build(mut self) -> skia_safe::Path {
        self.detach()
    }
}

#[cfg(feature = "skia")]
impl PathBuilder<Self> for skia_safe::Path {
    fn rewind(&mut self) {
        self.rewind();
    }

    fn move_to(&mut self, point: Point) {
        self.move_to((point.x, point.y));
    }

    fn line_to(&mut self, point: Point) {
        self.line_to((point.x, point.y));
    }

    fn cubic_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point) {
        self.cubic_to((ctrl1.x, ctrl1.y), (ctrl2.x, ctrl2.y), (to.x, to.y));
    }

    fn close(&mut self) {
        self.close();
    }

    fn transformed(mut self, transform: Matrix3) -> impl PathBuilder<Self> {
        let mut matrix = skia_safe::Matrix::new_identity();

        let scale_x = transform.m11;
        let skew_y = transform.m12;
        let persp0 = transform.m14;
        let skew_x = transform.m21;
        let scale_y = transform.m22;
        let persp1 = transform.m24;

        let translate_x = transform.m41;
        let translate_y = transform.m42;
        let persp2 = transform.m44;

        matrix.set_all(scale_x, skew_x, translate_x, skew_y, scale_y, translate_y, persp0, persp1, persp2);

        self.transform(&matrix);

        self
    }

    fn build(self) -> Self {
        self
    }
}
