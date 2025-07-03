use crate::{
    CornerRounding, RoundedPoint, RoundedPolygon,
    geometry::{Angle, Matrix3, Point, Size},
};

const CORNER_ROUND15: CornerRounding = CornerRounding::new(0.15);
const CORNER_ROUND20: CornerRounding = CornerRounding::new(0.2);
const CORNER_ROUND30: CornerRounding = CornerRounding::new(0.3);
const CORNER_ROUND50: CornerRounding = CornerRounding::new(0.5);
const CORNER_ROUND100: CornerRounding = CornerRounding::new(1.0);

fn rotate_neg45() -> Matrix3 {
    Matrix3::rotation(0.0, 0.0, 1.0, -Angle::degrees(45.0))
}

fn rotate_neg90() -> Matrix3 {
    Matrix3::rotation(0.0, 0.0, 1.0, -Angle::degrees(90.0))
}

fn rotate_neg135() -> Matrix3 {
    Matrix3::rotation(0.0, 0.0, 1.0, -Angle::degrees(135.0))
}

pub fn circle(vertices: Option<usize>) -> RoundedPolygon {
    RoundedPolygon::circle(vertices.unwrap_or(10), 1.0).normalized()
}

pub fn square() -> RoundedPolygon {
    RoundedPolygon::rectangle(Size::splat(1.0), Some(CORNER_ROUND30), None).normalized()
}

pub fn slanted() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.926, 0.970), CornerRounding::smoothed(0.189, 0.811)),
            RoundedPoint::new(Point::new(-0.021, 0.967), CornerRounding::smoothed(0.187, 0.057)),
        ],
        2,
        false,
    )
    .normalized()
}

pub fn arch() -> RoundedPolygon {
    RoundedPolygon::from_vertices_count(4, 1.0, None, &[CORNER_ROUND100, CORNER_ROUND100, CORNER_ROUND20, CORNER_ROUND20])
        .transformed(rotate_neg135())
        .normalized()
}

pub fn fan() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(1.004, 1.000), CornerRounding::smoothed(0.148, 0.417)),
            RoundedPoint::new(Point::new(0.000, 1.000), CornerRounding::new(0.151)),
            RoundedPoint::new(Point::new(0.000, -0.003), CornerRounding::new(0.148)),
            RoundedPoint::new(Point::new(0.978, 0.020), CornerRounding::new(0.803)),
        ],
        1,
        false,
    )
    .normalized()
}

pub fn arrow() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.500, 0.892), CornerRounding::new(0.313)),
            RoundedPoint::new(Point::new(-0.216, 1.050), CornerRounding::new(0.207)),
            RoundedPoint::new(Point::new(0.499, -0.160), CornerRounding::smoothed(0.215, 1.000)),
            RoundedPoint::new(Point::new(1.225, 1.060), CornerRounding::new(0.211)),
        ],
        1,
        false,
    )
    .normalized()
}

pub fn semi_circle() -> RoundedPolygon {
    RoundedPolygon::rectangle(
        Size::new(1.6, 1.0),
        None,
        Some([CORNER_ROUND20, CORNER_ROUND20, CORNER_ROUND100, CORNER_ROUND100]),
    )
    .normalized()
}

pub fn oval() -> RoundedPolygon {
    let m = Matrix3::scale(1.0, 0.64, 1.0);

    RoundedPolygon::circle(8, 1.0).transformed(m).transformed(rotate_neg45()).normalized()
}

pub fn pill() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.961, 0.039), CornerRounding::new(0.426)),
            RoundedPoint::unrounded(Point::new(1.001, 0.428)),
            RoundedPoint::new(Point::new(1.000, 0.609), CornerRounding::new(1.000)),
        ],
        2,
        true,
    )
    .normalized()
}

pub fn triangle() -> RoundedPolygon {
    RoundedPolygon::from_vertices_count(3, 1.0, Some(CORNER_ROUND20), &[])
        .transformed(rotate_neg90())
        .normalized()
}

pub fn diamond() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.500, 1.096), CornerRounding::smoothed(0.151, 0.524)),
            RoundedPoint::new(Point::new(0.040, 0.500), CornerRounding::new(0.159)),
        ],
        2,
        false,
    )
    .normalized()
}

pub fn clam_shell() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.171, 0.841), CornerRounding::new(0.159)),
            RoundedPoint::new(Point::new(-0.020, 0.500), CornerRounding::new(0.140)),
            RoundedPoint::new(Point::new(0.170, 0.159), CornerRounding::new(0.159)),
        ],
        2,
        false,
    )
    .normalized()
}

pub fn pentagon() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.500, -0.009), CornerRounding::new(0.172)),
            RoundedPoint::new(Point::new(1.030, 0.365), CornerRounding::new(0.164)),
            RoundedPoint::new(Point::new(0.828, 0.970), CornerRounding::new(0.169)),
        ],
        1,
        true,
    )
    .normalized()
}

pub fn gem() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.499, 1.023), CornerRounding::smoothed(0.241, 0.778)),
            RoundedPoint::new(Point::new(-0.005, 0.792), CornerRounding::new(0.208)),
            RoundedPoint::new(Point::new(0.073, 0.258), CornerRounding::new(0.228)),
            RoundedPoint::new(Point::new(0.433, -0.000), CornerRounding::new(0.491)),
        ],
        1,
        true,
    )
    .normalized()
}

pub fn sunny() -> RoundedPolygon {
    RoundedPolygon::star(8, 1.0, 0.8, Some(CORNER_ROUND15), None, &[]).normalized()
}

pub fn very_sunny() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.500, 1.080), CornerRounding::new(0.085)),
            RoundedPoint::new(Point::new(0.358, 0.843), CornerRounding::new(0.085)),
        ],
        8,
        false,
    )
    .normalized()
}

pub fn cookie4() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(1.237, 1.236), CornerRounding::new(0.258)),
            RoundedPoint::new(Point::new(0.500, 0.918), CornerRounding::new(0.233)),
        ],
        4,
        false,
    )
    .normalized()
}

pub fn cookie6() -> RoundedPolygon {
    // 6-point cookie
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.723, 0.884), CornerRounding::new(0.394)),
            RoundedPoint::new(Point::new(0.500, 1.099), CornerRounding::new(0.398)),
        ],
        6,
        false,
    )
    .normalized()
}

pub fn cookie7() -> RoundedPolygon {
    // 7-point cookie
    RoundedPolygon::star(7, 1.0, 0.75, Some(CORNER_ROUND50), None, &[])
        .transformed(rotate_neg90())
        .normalized()
}

pub fn cookie9() -> RoundedPolygon {
    RoundedPolygon::star(9, 1.0, 0.8, Some(CORNER_ROUND50), None, &[])
        .transformed(rotate_neg90())
        .normalized()
}

pub fn cookie12() -> RoundedPolygon {
    RoundedPolygon::star(12, 1.0, 0.8, Some(CORNER_ROUND50), None, &[])
        .transformed(rotate_neg90())
        .normalized()
}

pub fn ghostish() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.500, 0.0), CornerRounding::new(1.000)),
            RoundedPoint::new(Point::new(1.0, 0.0), CornerRounding::new(1.000)),
            RoundedPoint::new(Point::new(1.0, 1.140), CornerRounding::smoothed(0.254, 0.106)),
            RoundedPoint::new(Point::new(0.575, 0.906), CornerRounding::new(0.253)),
        ],
        1,
        true,
    )
    .normalized()
}

pub fn clover4() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::unrounded(Point::new(0.500, 0.074)),
            RoundedPoint::new(Point::new(0.725, -0.099), CornerRounding::new(0.476)),
        ],
        4,
        true,
    )
    .normalized()
}

pub fn clover8() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::unrounded(Point::new(0.500, 0.036)),
            RoundedPoint::new(Point::new(0.758, -0.101), CornerRounding::new(0.209)),
        ],
        8,
        false,
    )
    .normalized()
}

pub fn burst() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.500, -0.006), CornerRounding::new(0.006)),
            RoundedPoint::new(Point::new(0.592, 0.158), CornerRounding::new(0.006)),
        ],
        12,
        false,
    )
    .normalized()
}

pub fn soft_burst() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.193, 0.277), CornerRounding::new(0.053)),
            RoundedPoint::new(Point::new(0.176, 0.055), CornerRounding::new(0.053)),
        ],
        10,
        false,
    )
    .normalized()
}

pub fn boom() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.457, 0.296), CornerRounding::new(0.007)),
            RoundedPoint::new(Point::new(0.500, -0.051), CornerRounding::new(0.007)),
        ],
        15,
        false,
    )
    .normalized()
}

pub fn soft_boom() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::unrounded(Point::new(0.733, 0.454)),
            RoundedPoint::new(Point::new(0.839, 0.437), CornerRounding::new(0.532)),
            RoundedPoint::new(Point::new(0.949, 0.449), CornerRounding::smoothed(0.439, 1.000)),
            RoundedPoint::new(Point::new(0.998, 0.478), CornerRounding::new(0.174)),
        ],
        16,
        true,
    )
    .normalized()
}

pub fn flower() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::unrounded(Point::new(0.370, 0.187)),
            RoundedPoint::new(Point::new(0.416, 0.049), CornerRounding::new(0.381)),
            RoundedPoint::new(Point::new(0.479, 0.001), CornerRounding::new(0.095)),
        ],
        8,
        true,
    )
    .normalized()
}

pub fn puffy() -> RoundedPolygon {
    let matrix = Matrix3::scale(1.0, 0.742, 1.0);

    RoundedPolygon::from_points(
        &[
            RoundedPoint::unrounded(Point::new(0.500, 0.053)),
            RoundedPoint::new(Point::new(0.545, -0.040), CornerRounding::new(0.405)),
            RoundedPoint::new(Point::new(0.670, -0.035), CornerRounding::new(0.426)),
            RoundedPoint::new(Point::new(0.717, 0.066), CornerRounding::new(0.574)),
            RoundedPoint::unrounded(Point::new(0.722, 0.128)),
            RoundedPoint::new(Point::new(0.777, 0.002), CornerRounding::new(0.360)),
            RoundedPoint::new(Point::new(0.914, 0.149), CornerRounding::new(0.660)),
            RoundedPoint::new(Point::new(0.926, 0.289), CornerRounding::new(0.660)),
            RoundedPoint::unrounded(Point::new(0.881, 0.346)),
            RoundedPoint::new(Point::new(0.940, 0.344), CornerRounding::new(0.126)),
            RoundedPoint::new(Point::new(1.003, 0.437), CornerRounding::new(0.255)),
        ],
        2,
        true,
    )
    .transformed(|point| matrix.transform_point2d(point).unwrap_or(point))
    .normalized()
}

pub fn puffy_diamond() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.870, 0.130), CornerRounding::new(0.146)),
            RoundedPoint::unrounded(Point::new(0.818, 0.357)),
            RoundedPoint::new(Point::new(1.000, 0.332), CornerRounding::new(0.853)),
        ],
        4,
        true,
    )
    .normalized()
}

pub fn pixel_circle() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::unrounded(Point::new(0.500, 0.000)),
            RoundedPoint::unrounded(Point::new(0.704, 0.000)),
            RoundedPoint::unrounded(Point::new(0.704, 0.065)),
            RoundedPoint::unrounded(Point::new(0.843, 0.065)),
            RoundedPoint::unrounded(Point::new(0.843, 0.148)),
            RoundedPoint::unrounded(Point::new(0.926, 0.148)),
            RoundedPoint::unrounded(Point::new(0.926, 0.296)),
            RoundedPoint::unrounded(Point::new(1.000, 0.296)),
        ],
        2,
        true,
    )
    .normalized()
}

pub fn pixel_triangle() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::unrounded(Point::new(0.110, 0.500)),
            RoundedPoint::unrounded(Point::new(0.113, 0.000)),
            RoundedPoint::unrounded(Point::new(0.287, 0.000)),
            RoundedPoint::unrounded(Point::new(0.287, 0.087)),
            RoundedPoint::unrounded(Point::new(0.421, 0.087)),
            RoundedPoint::unrounded(Point::new(0.421, 0.170)),
            RoundedPoint::unrounded(Point::new(0.560, 0.170)),
            RoundedPoint::unrounded(Point::new(0.560, 0.265)),
            RoundedPoint::unrounded(Point::new(0.674, 0.265)),
            RoundedPoint::unrounded(Point::new(0.675, 0.344)),
            RoundedPoint::unrounded(Point::new(0.789, 0.344)),
            RoundedPoint::unrounded(Point::new(0.789, 0.439)),
            RoundedPoint::unrounded(Point::new(0.888, 0.439)),
        ],
        1,
        true,
    )
    .normalized()
}

pub fn bun() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::unrounded(Point::new(0.796, 0.500)),
            RoundedPoint::new(Point::new(0.853, 0.518), CornerRounding::new(1.0)),
            RoundedPoint::new(Point::new(0.992, 0.631), CornerRounding::new(1.0)),
            RoundedPoint::new(Point::new(0.968, 1.000), CornerRounding::new(1.0)),
        ],
        2,
        true,
    )
    .normalized()
}

pub fn heart() -> RoundedPolygon {
    RoundedPolygon::from_points(
        &[
            RoundedPoint::new(Point::new(0.500, 0.268), CornerRounding::new(0.016)),
            RoundedPoint::new(Point::new(0.792, -0.066), CornerRounding::new(0.958)),
            RoundedPoint::new(Point::new(1.064, 0.276), CornerRounding::new(1.000)),
            RoundedPoint::new(Point::new(0.501, 0.946), CornerRounding::new(0.129)),
        ],
        1,
        true,
    )
    .normalized()
}
