use float_cmp::assert_approx_eq;
use polymorpher::{
    CornerRounding, Cubic, Feature, RoundedPolygon,
    geometry::{Point, Vector},
};

const ROUNDING: CornerRounding = CornerRounding::new(1.0);
const PER_VERTEX: [CornerRounding; 4] = [ROUNDING; 4];
const EPSILON: f32 = 1e-4;

fn assert_point_ge(expected: Point, actual: Point) {
    assert!(
        actual.x >= expected.x - EPSILON,
        "expected X to be greater than or equal {}, found: {}",
        expected.x,
        actual.x
    );
    assert!(
        actual.y >= expected.y - EPSILON,
        "expected Y to be greater than or equal {}, found: {}",
        expected.y,
        actual.y
    );
}

fn assert_point_le(expected: Point, actual: Point) {
    assert!(
        actual.x <= expected.x + EPSILON,
        "expected X to be lesser than or equal {}, found: {}",
        expected.x,
        actual.x
    );
    assert!(
        actual.y <= expected.y + EPSILON,
        "expected Y to be lesser than or equal {}, found: {}",
        expected.y,
        actual.y
    );
}

fn assert_is_inside(shape: &[Cubic], min: Point, max: Point) {
    for cubic in shape {
        assert_point_ge(min, cubic.anchor0());
        assert_point_le(max, cubic.anchor0());
        assert_point_ge(min, cubic.control0());
        assert_point_le(max, cubic.control0());
        assert_point_ge(min, cubic.control1());
        assert_point_le(max, cubic.control1());
        assert_point_ge(min, cubic.anchor1());
        assert_point_le(max, cubic.anchor1());
    }
}

fn assert_points(expected: Point, actual: Point) {
    assert_approx_eq!(f32, expected.x, actual.x, epsilon = EPSILON);
    assert_approx_eq!(f32, expected.y, actual.y, epsilon = EPSILON);
}

fn assert_cubics(expected: &Cubic, actual: &Cubic) {
    assert_points(expected.anchor0(), actual.anchor0());
    assert_points(expected.control0(), actual.control0());
    assert_points(expected.control1(), actual.control1());
    assert_points(expected.anchor1(), actual.anchor1());
}

fn assert_features(expected: &Feature, actual: &Feature) {
    assert_eq!(expected.cubics.len(), actual.cubics.len());

    for i in 0..expected.cubics.len() {
        assert_cubics(&expected.cubics[i], &actual.cubics[i]);
    }

    assert_eq!(expected.ty, actual.ty);

    if expected.is_corner() && actual.is_corner() {
        assert!(expected.is_corner_and(|a| actual.is_corner_and(|b| a == b)));
    }
}

fn assert_polygons(expected: &RoundedPolygon, actual: &RoundedPolygon) {
    assert_eq!(expected.cubics.len(), actual.cubics.len());

    for i in 0..expected.cubics.len() {
        assert_cubics(&expected.cubics[i], &actual.cubics[i]);
    }

    assert_eq!(expected.features.len(), actual.features.len());

    for i in 0..expected.features.len() {
        assert_features(&expected.features[i], &actual.features[i]);
    }
}

#[test]
fn from_vertices_count_test() {
    let square = RoundedPolygon::from_vertices_count(4, 1.0, None, &[]);

    assert_is_inside(&square.cubics, Point::splat(-1.0), Point::splat(1.0));

    let square = RoundedPolygon::from_vertices_count(4, 2.0, None, &[]);

    assert_is_inside(&square.cubics, Point::splat(-2.0), Point::splat(2.0));

    let square = RoundedPolygon::from_vertices_count(4, 1.0, Some(ROUNDING), &[]);

    assert_is_inside(&square.cubics, Point::splat(-1.0), Point::splat(1.0));

    let square = RoundedPolygon::from_vertices_count(4, 1.0, None, &PER_VERTEX);

    assert_is_inside(&square.cubics, Point::splat(-1.0), Point::splat(1.0));
}

#[test]
fn from_vertices_test() {
    let p0 = Point::new(1.0, 0.0);
    let p1 = Point::new(0.0, 1.0);
    let p2 = Point::new(-1.0, 0.0);
    let p3 = Point::new(0.0, -1.0);
    let verts = [p0, p1, p2, p3];

    let square = RoundedPolygon::from_vertices(&verts, CornerRounding::UNROUNDED, &[], Point::splat(f32::MIN));

    assert_is_inside(&square.cubics, Point::splat(-1.0), Point::splat(1.0));

    let offset = Vector::new(1.0, 2.0);
    let offset_verts = [p0 + offset, p1 + offset, p2 + offset, p3 + offset];

    let square = RoundedPolygon::from_vertices(&offset_verts, CornerRounding::UNROUNDED, &[], offset.to_point());

    assert_is_inside(&square.cubics, Point::new(0.0, 1.0), Point::new(2.0, 3.0));

    let square = RoundedPolygon::from_vertices(&verts, ROUNDING, &[], Point::splat(f32::MIN));

    assert_is_inside(&square.cubics, Point::splat(-1.0), Point::splat(1.0));

    let square = RoundedPolygon::from_vertices(&verts, CornerRounding::UNROUNDED, &PER_VERTEX, Point::splat(f32::MIN));

    assert_is_inside(&square.cubics, Point::splat(-1.0), Point::splat(1.0));
}

#[test]
fn same_rectangle_from_features_test() {
    let base = RoundedPolygon::rectangle().build();
    let actual = RoundedPolygon::from_features(base.features.clone(), None);

    assert_polygons(&base, &actual);
}

#[test]
fn same_rounded_rectangle_from_features_test() {
    let base = RoundedPolygon::rectangle().with_rounding(CornerRounding::smoothed(0.5, 0.2)).build();
    let actual = RoundedPolygon::from_features(base.features.clone(), None);

    assert_polygons(&base, &actual);
}

#[test]
fn same_circles_from_features_test() {
    for i in 3..=20 {
        let base = RoundedPolygon::circle().with_vertices(i).build();
        let actual = RoundedPolygon::from_features(base.features.clone(), None);

        assert_polygons(&base, &actual);
    }
}

#[test]
fn same_stars_from_features_test() {
    for i in 3..=20 {
        let base = RoundedPolygon::star(i).build();
        let actual = RoundedPolygon::from_features(base.features.clone(), None);

        assert_polygons(&base, &actual);
    }
}

#[test]
fn same_rounded_stars_from_features_test() {
    for i in 3..=20 {
        let base = RoundedPolygon::star(i).with_rounding(CornerRounding::smoothed(0.5, 0.2)).build();
        let actual = RoundedPolygon::from_features(base.features.clone(), None);

        assert_polygons(&base, &actual);
    }
}

#[test]
fn same_pill_from_features_test() {
    let base = RoundedPolygon::pill().build();
    let actual = RoundedPolygon::from_features(base.features.clone(), None);

    assert_polygons(&base, &actual);
}

#[test]
fn same_pill_star_from_features_test() {
    let base = RoundedPolygon::pill_star().with_rounding(CornerRounding::smoothed(0.5, 0.2)).build();
    let actual = RoundedPolygon::from_features(base.features.clone(), None);

    assert_polygons(&base, &actual);
}

#[test]
fn calc_center_test() {
    let polygon = RoundedPolygon::from_vertices(
        &[Point::zero(), Point::new(1.0, 0.0), Point::new(0.0, 1.0), Point::splat(1.0)],
        CornerRounding::UNROUNDED,
        &[],
        Point::splat(f32::MIN),
    );

    assert_approx_eq!(f32, 0.5, polygon.center.x, epsilon = 1e-4);
    assert_approx_eq!(f32, 0.5, polygon.center.y, epsilon = 1e-4);
}

#[test]
fn rounding_space_usage_test() {
    let p0 = Point::new(0.0, 0.0);
    let p1 = Point::new(1.0, 0.0);
    let p2 = Point::new(0.5, 1.0);
    let pv_rounding = [
        CornerRounding::smoothed(1.0, 0.0),
        CornerRounding::smoothed(1.0, 1.0),
        CornerRounding::UNROUNDED,
    ];
    let polygon = RoundedPolygon::from_vertices(&[p0, p1, p2], CornerRounding::UNROUNDED, &pv_rounding, Point::splat(f32::MIN));

    // Since there is not enough room in the p0 -> p1 side even for the roundings,
    // we shouldn't take smoothing into account, so the corners should end in
    // the middle point.
    let lower_edge_feature = polygon.features.iter().find(|it| !it.is_corner()).unwrap();

    assert_eq!(1, lower_edge_feature.cubics.len());

    let lower_edge = lower_edge_feature.cubics.first().unwrap();

    assert_points(Point::new(0.5, 0.0), lower_edge.anchor0());
    assert_points(Point::new(0.5, 0.0), lower_edge.anchor1());
}

const POINTS: u16 = 20;

#[test]
fn uneven_smoothing_test() {
    // Vertex 3 has the default 0.5 radius, 0 smoothing.
    // Vertex 0 has 0.4 radius, and smoothing varying from 0 to 1.
    for it in 0..=POINTS {
        let smooth = f32::from(it) / f32::from(POINTS);

        do_uneven_smooth_test(
            CornerRounding::smoothed(0.4, smooth),
            0.4 * (1.0 + smooth),
            (0.4 * (1.0 + smooth)).min(0.5),
            0.5,
            None,
        );
    }
}

#[test]
fn uneven_smoothing_test2() {
    // Vertex 3 has the default 0.5 radius, 0 smoothing.
    // Vertex 0 has 0.4 radius, and smoothing varying from 0 to 1.
    for it in 0..=POINTS {
        let smooth = f32::from(it) / f32::from(POINTS);

        let smooth_wanted_v0 = 0.4 * smooth;
        let smooth_wanted_v3 = 0.2;

        let factor = (0.4 / (smooth_wanted_v0 + smooth_wanted_v3)).min(1.0);

        do_uneven_smooth_test(
            CornerRounding::smoothed(0.4, smooth),
            0.4 * (1.0 + smooth),
            factor.mul_add(smooth_wanted_v0, 0.4),
            factor.mul_add(smooth_wanted_v3, 0.2),
            Some(CornerRounding::smoothed(0.2, 1.0)),
        );
    }
}

#[test]
fn uneven_smoothing_test3() {
    // Vertex 3 has the default 0.5 radius, 0 smoothing.
    // Vertex 0 has 0.4 radius, and smoothing varying from 0 to 1.
    for it in 0..=POINTS {
        let smooth = f32::from(it) / f32::from(POINTS);

        do_uneven_smooth_test(
            CornerRounding::smoothed(0.4, smooth),
            0.4 * (1.0 + smooth),
            0.4,
            0.6,
            Some(CornerRounding::new(0.6)),
        );
    }
}

#[test]
fn creating_full_size_test() {
    let radius = 400.0;
    let inner_radius_factor = 0.35;
    let inner_radius = radius * inner_radius_factor;
    let rounding_factor = 0.32;

    let full_size_shape = RoundedPolygon::star(4)
        .with_radius(radius)
        .with_inner_radius(inner_radius)
        .with_rounding(CornerRounding::new(radius * rounding_factor))
        .with_inner_rounding(CornerRounding::new(radius * rounding_factor))
        .with_center(Point::splat(radius))
        .build()
        .transformed(|point: Point| Point::new((point.x - radius) / radius, (point.y - radius) / radius));

    let canonical_shape = RoundedPolygon::star(4)
        .with_radius(1.0)
        .with_inner_radius(inner_radius_factor)
        .with_rounding(CornerRounding::new(rounding_factor))
        .with_inner_rounding(CornerRounding::new(rounding_factor))
        .build();

    let cubics = canonical_shape.cubics;
    let cubics1 = full_size_shape.cubics;

    assert_eq!(cubics.len(), cubics1.len());

    cubics.into_iter().zip(cubics1).for_each(|(cubic, cubic1)| {
        assert_approx_eq!(f32, cubic.anchor0().x, cubic1.anchor0().x, epsilon = EPSILON);
        assert_approx_eq!(f32, cubic.anchor0().y, cubic1.anchor0().y, epsilon = EPSILON);
        assert_approx_eq!(f32, cubic.anchor1().x, cubic1.anchor1().x, epsilon = EPSILON);
        assert_approx_eq!(f32, cubic.anchor1().y, cubic1.anchor1().y, epsilon = EPSILON);
        assert_approx_eq!(f32, cubic.control0().x, cubic1.control0().x, epsilon = EPSILON);
        assert_approx_eq!(f32, cubic.control0().y, cubic1.control0().y, epsilon = EPSILON);
        assert_approx_eq!(f32, cubic.control1().x, cubic1.control1().x, epsilon = EPSILON);
        assert_approx_eq!(f32, cubic.control1().y, cubic1.control1().y, epsilon = EPSILON);
    });
}

#[allow(clippy::similar_names)]
fn do_uneven_smooth_test(rounding0: CornerRounding, expected_v0_sx: f32, expected_v0_sy: f32, expected_v3_sy: f32, rounding3: Option<CornerRounding>) {
    let rounding3 = rounding3.unwrap_or_else(|| CornerRounding::new(0.5));

    let p0 = Point::new(0.0, 0.0);
    let p1 = Point::new(5.0, 0.0);
    let p2 = Point::new(5.0, 1.0);
    let p3 = Point::new(0.0, 1.0);

    let pv_rounding = [rounding0, CornerRounding::UNROUNDED, CornerRounding::UNROUNDED, rounding3];
    let polygon = RoundedPolygon::from_vertices(&[p0, p1, p2, p3], CornerRounding::UNROUNDED, &pv_rounding, Point::splat(f32::MIN));

    let features = polygon.features.iter().filter(|feature| !feature.is_corner()).collect::<Vec<_>>();
    let [e01, _, _, e30] = features.as_slice() else { unreachable!() };

    assert_approx_eq!(f32, expected_v0_sx, e01.cubics[0].anchor0().x, epsilon = EPSILON);
    assert_approx_eq!(f32, expected_v0_sy, e30.cubics[0].anchor1().y, epsilon = EPSILON);
    assert_approx_eq!(f32, expected_v3_sy, 1.0 - e30.cubics[0].anchor0().y, epsilon = EPSILON);
}
