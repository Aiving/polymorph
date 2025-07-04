use polymorpher::DoubleMapper;

const EPSILON: f32 = 1e-4;

#[test]
fn identity_mapping_test() {
    validate_mapping(&DoubleMapper::identity(), |v| v);
}

#[test]
fn simple_mapping_test() {
    validate_mapping(
        // Map the first half of the start source to the first quarter of the target.
        &DoubleMapper::new([(0.0, 0.0), (0.5, 0.25)]),
        |x| if x < 0.5 { x / 2.0 } else { 3.0f32.mul_add(x, -1.0) / 2.0 },
    );
}

#[test]
fn target_wraps_test() {
    validate_mapping(
        // mapping applies a "+ 0.5f"
        &DoubleMapper::new([(0.0, 0.5), (0.1, 0.6)]),
        |x| (x + 0.5) % 1.0,
    );
}

#[test]
fn source_wraps_test() {
    validate_mapping(
        // Values on the source wrap (this is still the "+ 0.5f" function)
        &DoubleMapper::new([(0.5, 0.0), (0.1, 0.6)]),
        |x| (x + 0.5) % 1.0,
    );
}

#[test]
fn both_wrap_test() {
    validate_mapping(
        // Just the identity function
        &DoubleMapper::new([(0.5, 0.5), (0.75, 0.75), (0.1, 0.1), (0.49, 0.49)]),
        |v| v,
    );
}

#[test]
fn multiple_point_test() {
    validate_mapping(&DoubleMapper::new([(0.4, 0.2), (0.5, 0.22), (0.0, 0.8)]), |x| {
        if x < 0.4 {
            (0.8 + x) % 1.0
        } else if x < 0.5 {
            0.2 + (x - 0.4) / 5.0
        } else {
            // maps a change of 0.5 in the source to a change 0.58 in the target, hence the
            // 1.16
            (x - 0.5).mul_add(1.16, 0.22)
        }
    });
}

fn validate_mapping(mapper: &DoubleMapper, expected_function: fn(f32) -> f32) {
    for i in 0..9999u16 {
        let source = f32::from(i) / 10000.0;
        let target = expected_function(source);

        assert!((target - mapper.map(source)).abs() < EPSILON);
        assert!((source - mapper.map_back(target)).abs() < EPSILON);
    }
}
