use euclid::approxeq::ApproxEq;
use polymorpher::{
    Cubic,
    geometry::{Point, Vector},
};

// These points create a roughly circular arc in the upper-right quadrant around
// (0,0)
const EPSILON: Point = Point::new(1e-4, 1e-4);
const ZERO: Point = Point::new(0.0, 0.0);
const P0: Point = Point::new(1.0, 0.0);
const P1: Point = Point::new(1.0, 0.5);
const P2: Point = Point::new(0.5, 1.0);
const P3: Point = Point::new(0.0, 1.0);
const CUBIC: Cubic = Cubic::new(P0, P1, P2, P3);

#[test]
fn construction_test() {
    assert_eq!(P0, CUBIC.anchor0());
    assert_eq!(P1, CUBIC.control0());
    assert_eq!(P2, CUBIC.control1());
    assert_eq!(P3, CUBIC.anchor1());
}

#[test]
fn circular_arc_test() {
    let arc_cubic = Cubic::circular_arc(ZERO, P0, P3);

    assert_eq!(P0, arc_cubic.anchor0());
    assert_eq!(P3, arc_cubic.anchor1());
}

#[test]
fn div_test() {
    let mut cubic = CUBIC / 1.0;

    assert!(CUBIC.anchor0().approx_eq_eps(&cubic.anchor0(), &EPSILON));
    assert!(CUBIC.control0().approx_eq_eps(&cubic.control0(), &EPSILON));
    assert!(CUBIC.control1().approx_eq_eps(&cubic.control1(), &EPSILON));
    assert!(CUBIC.anchor1().approx_eq_eps(&cubic.anchor1(), &EPSILON));

    cubic = CUBIC / 1.0;

    assert!(CUBIC.anchor0().approx_eq_eps(&cubic.anchor0(), &EPSILON));
    assert!(CUBIC.control0().approx_eq_eps(&cubic.control0(), &EPSILON));
    assert!(CUBIC.control1().approx_eq_eps(&cubic.control1(), &EPSILON));
    assert!(CUBIC.anchor1().approx_eq_eps(&cubic.anchor1(), &EPSILON));

    cubic = CUBIC / 2.0;

    assert!((P0 / 2.0).approx_eq_eps(&cubic.anchor0(), &EPSILON));
    assert!((P1 / 2.0).approx_eq_eps(&cubic.control0(), &EPSILON));
    assert!((P2 / 2.0).approx_eq_eps(&cubic.control1(), &EPSILON));
    assert!((P3 / 2.0).approx_eq_eps(&cubic.anchor1(), &EPSILON));

    cubic = CUBIC / 2.0;

    assert!((P0 / 2.0).approx_eq_eps(&cubic.anchor0(), &EPSILON));
    assert!((P1 / 2.0).approx_eq_eps(&cubic.control0(), &EPSILON));
    assert!((P2 / 2.0).approx_eq_eps(&cubic.control1(), &EPSILON));
    assert!((P3 / 2.0).approx_eq_eps(&cubic.anchor1(), &EPSILON));
}

#[test]
fn mul_test() {
    let mut cubic = CUBIC * 1.0;

    assert_eq!(P0, cubic.anchor0());
    assert_eq!(P1, cubic.control0());
    assert_eq!(P2, cubic.control1());
    assert_eq!(P3, cubic.anchor1());

    cubic = CUBIC * 1.0;

    assert_eq!(P0, cubic.anchor0());
    assert_eq!(P1, cubic.control0());
    assert_eq!(P2, cubic.control1());
    assert_eq!(P3, cubic.anchor1());

    cubic = CUBIC * 2.0;

    assert!((P0 * 2.0).approx_eq_eps(&cubic.anchor0(), &EPSILON));
    assert!((P1 * 2.0).approx_eq_eps(&cubic.control0(), &EPSILON));
    assert!((P2 * 2.0).approx_eq_eps(&cubic.control1(), &EPSILON));
    assert!((P3 * 2.0).approx_eq_eps(&cubic.anchor1(), &EPSILON));

    cubic = CUBIC * 2.0;

    assert!((P0 * 2.0).approx_eq_eps(&cubic.anchor0(), &EPSILON));
    assert!((P1 * 2.0).approx_eq_eps(&cubic.control0(), &EPSILON));
    assert!((P2 * 2.0).approx_eq_eps(&cubic.control1(), &EPSILON));
    assert!((P3 * 2.0).approx_eq_eps(&cubic.anchor1(), &EPSILON));
}

#[test]
fn plus_test() {
    let offset_cubic = CUBIC * 2.0;
    let cubic = CUBIC + offset_cubic;

    assert!((P0 + offset_cubic.anchor0().to_vector()).approx_eq_eps(&cubic.anchor0(), &EPSILON));
    assert!((P1 + offset_cubic.control0().to_vector()).approx_eq_eps(&cubic.control0(), &EPSILON));
    assert!((P2 + offset_cubic.control1().to_vector()).approx_eq_eps(&cubic.control1(), &EPSILON));
    assert!((P3 + offset_cubic.anchor1().to_vector()).approx_eq_eps(&cubic.anchor1(), &EPSILON));
}

#[test]
fn reverse_test() {
    let cubic = CUBIC.reversed();

    assert_eq!(P3, cubic.anchor0());
    assert_eq!(P2, cubic.control0());
    assert_eq!(P1, cubic.control1());
    assert_eq!(P0, cubic.anchor1());
}

fn assert_between(end0: Point, end1: Point, actual: Point) {
    let min_x = end0.x.min(end1.x);
    let min_y = end0.y.min(end1.y);
    let max_x = end0.x.max(end1.x);
    let max_y = end0.y.max(end1.y);

    assert!(min_x <= actual.x);
    assert!(min_y <= actual.y);
    assert!(max_x >= actual.x);
    assert!(max_y >= actual.y);
}

#[test]
fn straight_line_test() {
    let line_cubic = Cubic::straight_line(P0, P3);

    assert_eq!(P0, line_cubic.anchor0());
    assert_eq!(P3, line_cubic.anchor1());

    assert_between(P0, P3, line_cubic.control0());
    assert_between(P0, P3, line_cubic.control1());
}

#[test]
fn split_test() {
    let (split0, split1) = CUBIC.split(0.5);

    assert_eq!(CUBIC.anchor0(), split0.anchor0());
    assert_eq!(CUBIC.anchor1(), split1.anchor1());

    assert_between(CUBIC.anchor0(), CUBIC.anchor1(), split0.anchor1());
    assert_between(CUBIC.anchor0(), CUBIC.anchor1(), split1.anchor0());
}

#[test]
fn point_on_curve_test() {
    let mut halfway = CUBIC.point_on_curve(0.5);

    assert_between(CUBIC.anchor0(), CUBIC.anchor1(), halfway);

    let cubic = Cubic::straight_line(P0, P3);

    halfway = cubic.point_on_curve(0.5);

    let computed_halfway = Point::new(0.5f32.mul_add(P3.x - P0.x, P0.x), 0.5f32.mul_add(P3.y - P0.y, P0.y));

    assert!(computed_halfway.approx_eq_eps(&halfway, &EPSILON));
}

#[test]
fn transform_test() {
    let mut cubic = CUBIC.transformed(&|point| point);

    assert!(CUBIC.anchor0().approx_eq_eps(&cubic.anchor0(), &EPSILON));
    assert!(CUBIC.control0().approx_eq_eps(&cubic.control0(), &EPSILON));
    assert!(CUBIC.control1().approx_eq_eps(&cubic.control1(), &EPSILON));
    assert!(CUBIC.anchor1().approx_eq_eps(&cubic.anchor1(), &EPSILON));

    cubic = CUBIC.transformed(&|point| point * 3.0);

    assert!((CUBIC * 3.0).anchor0().approx_eq_eps(&cubic.anchor0(), &EPSILON));
    assert!((CUBIC * 3.0).control0().approx_eq_eps(&cubic.control0(), &EPSILON));
    assert!((CUBIC * 3.0).control1().approx_eq_eps(&cubic.control1(), &EPSILON));
    assert!((CUBIC * 3.0).anchor1().approx_eq_eps(&cubic.anchor1(), &EPSILON));

    let translation_vector = Vector::new(200.0, 300.0);

    cubic = CUBIC.transformed(&|point| point + translation_vector);

    assert!((CUBIC.anchor0() + translation_vector).approx_eq_eps(&cubic.anchor0(), &EPSILON));
    assert!((CUBIC.control0() + translation_vector).approx_eq_eps(&cubic.control0(), &EPSILON));
    assert!((CUBIC.control1() + translation_vector).approx_eq_eps(&cubic.control1(), &EPSILON));
    assert!((CUBIC.anchor1() + translation_vector).approx_eq_eps(&cubic.anchor1(), &EPSILON));
}

#[test]
fn empty_cubic_has_zero_length() {
    assert!(Cubic::new(Point::splat(10.0), Point::splat(10.0), Point::splat(10.0), Point::splat(10.0)).zero_length());
}
