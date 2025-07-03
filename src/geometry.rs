use core::f32;

pub type Point = euclid::default::Point2D<f32>;
pub type Vector = euclid::default::Vector2D<f32>;
pub type Aabb = euclid::default::Box2D<f32>;
pub type Matrix3 = euclid::default::Transform3D<f32>;
pub type Angle = euclid::Angle<f32>;

pub const DISTANCE_EPSILON: f32 = 1e-4;
pub const ANGLE_EPSILON: f32 = 1e-6;

pub trait GeometryExt {
    #[must_use]
    fn rotated(self, angle: f32, center: Self) -> Self;
    #[must_use]
    fn rotate90(&self) -> Self;
    #[must_use]
    fn get_direction(&self) -> Self;
    fn is_convex(&self, current: Self, next: Self) -> bool;
}

impl GeometryExt for Point {
    fn rotated(self, a: f32, center: Self) -> Self {
        let a = a / 360.0 * 2.0 * f32::consts::PI;
        let off = self - center;

        Self::new(off.x.mul_add(a.cos(), -(off.y * a.sin())), off.x.mul_add(a.sin(), off.y * a.cos())) + center.to_vector()
    }

    fn rotate90(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    fn get_direction(&self) -> Self {
        let d = self.to_vector().length();

        assert!(d > 0.0, "Can't get the direction of a 0-length vector");

        *self / d
    }

    fn is_convex(&self, current: Self, next: Self) -> bool {
        let a = current - *self;
        let b = next - current;

        a.x.mul_add(b.y, -(a.y * b.x)) > 0.0
    }
}

impl GeometryExt for Vector {
    fn rotated(self, a: f32, center: Self) -> Self {
        let a = a / 360.0 * 2.0 * f32::consts::PI;
        let off = self - center;

        Self::new(off.x.mul_add(a.cos(), -(off.y * a.sin())), off.x.mul_add(a.sin(), off.y * a.cos())) + center
    }

    fn rotate90(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    fn get_direction(&self) -> Self {
        let d = self.length();

        assert!(d > 0.0, "Can't get the direction of a 0-length vector");

        *self / d
    }

    fn is_convex(&self, current: Self, next: Self) -> bool {
        let a = current - *self;
        let b = next - current;

        a.x.mul_add(b.y, -(a.y * b.x)) > 0.0
    }
}

pub trait PointTransformer {
    fn transform(&self, point: Point) -> Point;
}

impl<F: Fn(Point) -> Point> PointTransformer for F {
    fn transform(&self, point: Point) -> Point {
        self(point)
    }
}
