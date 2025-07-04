use euclid::approxeq::ApproxEq;
use polymorpher::{
    Cubic, Feature,
    geometry::{DISTANCE_EPSILON, Point, Vector},
};

const EPSILON: Point = Point::new(1e-4, 1e-4);

fn assert_features(expected: &Feature, actual: &Feature) {
    assert_eq!(expected.cubics.len(), actual.cubics.len());

    for i in 0..expected.cubics.len() {
        assert!(expected.cubics[i].anchor0().approx_eq_eps(&actual.cubics[i].anchor0(), &EPSILON));
        assert!(expected.cubics[i].control0().approx_eq_eps(&actual.cubics[i].control0(), &EPSILON));
        assert!(expected.cubics[i].control1().approx_eq_eps(&actual.cubics[i].control1(), &EPSILON));
        assert!(expected.cubics[i].anchor1().approx_eq_eps(&actual.cubics[i].anchor1(), &EPSILON));
    }

    assert_eq!(expected.ty, actual.ty);

    if expected.is_corner() && actual.is_corner() {
        assert!(expected.is_corner_and(|a| actual.is_corner_and(|b| a == b)));
    }
}

fn validated(feature: Feature) -> Feature {
    assert!(!feature.cubics.is_empty(), "Features need at least one cubic.");

    assert!(
        is_continuous(&feature),
        "Feature must be continuous, with the anchor points of all cubics matching the anchor points of the preceding and succeeding cubics."
    );

    feature
}

fn is_continuous(feature: &Feature) -> bool {
    let mut prev_cubic = feature.cubics[0];

    for index in 1..feature.cubics.len() {
        let cubic = feature.cubics[index];

        if (cubic.anchor0() - prev_cubic.anchor1()).lower_than(Vector::splat(DISTANCE_EPSILON)).any() {
            return false;
        }

        prev_cubic = cubic;
    }

    true
}

#[test]
fn builds_concave_corner() {
    let cubic = Cubic::straight_line(Point::zero(), Point::new(1.0, 0.0));
    let actual = validated(Feature::corner(vec![cubic], false));
    let expected = Feature::corner(vec![cubic], false);

    assert_features(&expected, &actual);
}

#[test]
fn builds_convex_corner() {
    let cubic = Cubic::straight_line(Point::zero(), Point::new(1.0, 0.0));
    let actual = validated(Feature::corner(vec![cubic], true));
    let expected = Feature::corner(vec![cubic], true);

    assert_features(&expected, &actual);
}

#[test]
fn builds_ignorable_as_edge() {
    let cubic = Cubic::straight_line(Point::zero(), Point::new(1.0, 0.0));
    let actual = validated(Feature::edge(vec![cubic]));
    let expected = Feature::edge(vec![cubic]);

    assert_features(&expected, &actual);
}
