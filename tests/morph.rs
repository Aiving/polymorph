use polymorpher::{
    Morph, RoundedPolygon,
    geometry::{Point, Vector},
};

const EPSILON: f32 = 1e-4;

#[test]
fn cubics_test() {
    let poly1 = RoundedPolygon::from_vertices_count_at(3, 1.0, Point::new(0.5, 0.5), None, &[]);
    let cubics11 = Morph::new(poly1.clone(), poly1.clone()).as_cubics(0.0);

    assert!(!cubics11.is_empty());

    // The structure of a morph and its component shapes may not match exactly,
    // because morph calculations may optimize some of the zero-length curves
    // out. But in general, every curve in the morph *should* exist somewhere in
    // the shape it is based on, so we do an exhaustive search for such
    // existence. Note that this assertion only works because we constructed the
    // Morph from/to the same shape. A Morph between different shapes
    // may not have the curves replicated exactly.
    for morph_cubic in cubics11 {
        let mut matched = false;

        for p1_cubic in &poly1.cubics {
            if (morph_cubic.anchor0() - p1_cubic.anchor0())
                .abs()
                .lower_than(Vector::splat(EPSILON))
                .and((morph_cubic.anchor1() - p1_cubic.anchor1()).abs().lower_than(Vector::splat(EPSILON)))
                .and((morph_cubic.control0() - p1_cubic.control0()).abs().lower_than(Vector::splat(EPSILON)))
                .and((morph_cubic.control1() - p1_cubic.control1()).abs().lower_than(Vector::splat(EPSILON)))
                .all()
            {
                matched = true;

                break;
            }
        }

        assert!(matched);
    }
}
