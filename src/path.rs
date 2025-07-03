#[cfg(feature = "lyon")]
use lyon_tessellation::path::{
    builder::NoAttributes,
    traits::{Build, PathBuilder as LyonPathBuilder},
};

use crate::{Cubic, geometry::Point};

pub trait PathBuilder {
    type Path;

    fn rewind(&mut self);
    fn move_to(&mut self, point: Point);
    fn line_to(&mut self, point: Point);
    fn cubic_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point);
    fn close(&mut self);

    fn build(self) -> Self::Path;
}

pub fn add_cubics<T: PathBuilder>(builder: &mut T, repeat_path: bool, close_path: bool, cubics: &[Cubic]) {
    let mut first = true;

    builder.rewind();

    for it in cubics {
        if first {
            builder.move_to(it.anchor0());

            first = false;
        }

        builder.cubic_to(it.control0(), it.control1(), it.anchor1());
    }

    if repeat_path {
        let mut first_in_repeat = true;

        for it in cubics {
            if first_in_repeat {
                builder.line_to(it.anchor0());

                first_in_repeat = false;
            }

            builder.cubic_to(it.control0(), it.control1(), it.anchor1());
        }
    }

    if close_path {
        builder.close();
    }
}

#[cfg(feature = "lyon")]
impl<T: LyonPathBuilder + Build> PathBuilder for NoAttributes<T> {
    type Path = T::PathType;

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

    fn build(self) -> Self::Path {
        Self::build(self)
    }
}

#[cfg(feature = "skia")]
impl PathBuilder for skia_safe::PathBuilder {
    type Path = skia_safe::Path;

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

    fn build(mut self) -> Self::Path {
        self.detach()
    }
}

#[cfg(feature = "skia")]
impl PathBuilder for skia_safe::Path {
    type Path = Self;

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

    fn build(self) -> Self {
        self
    }
}
