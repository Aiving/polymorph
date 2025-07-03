use crate::{
    Cubic, DoubleMapper, MeasuredPolygon, RoundedPolygon,
    geometry::{ANGLE_EPSILON, Angle, Matrix3},
    measurer::LengthMeasurer,
    path::PathBuilder,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Morph {
    start: RoundedPolygon,
    end: RoundedPolygon,
    r#match: Vec<(Cubic, Cubic)>,
}

impl Morph {
    pub fn new(start: RoundedPolygon, end: RoundedPolygon) -> Self {
        let r#match = Self::match_morph(&start, &end);

        Self { start, end, r#match }
    }

    pub fn as_cubics(&self, progress: f32) -> Vec<Cubic> {
        let mut cubics = Vec::new();

        // The first/last mechanism here ensures that the final anchor point in the
        // shape exactly matches the first anchor point. There can be rendering
        // artifacts introduced by those points being slightly off, even by much
        // less than a pixel
        let mut first_cubic: Option<Cubic> = None;
        let mut last_cubic: Option<Cubic> = None;

        for i in 0..self.r#match.len() {
            let cubic = Cubic::from_fn(|it| self.r#match[i].0.points[it].lerp(self.r#match[i].1.points[it], progress));

            if first_cubic.is_none() {
                first_cubic.replace(cubic);
            }

            if let Some(cubic) = last_cubic {
                cubics.push(cubic);
            }

            last_cubic.replace(cubic);
        }

        #[allow(clippy::collapsible_if)] // For MSRV compability
        if let Some(last_cubic) = last_cubic {
            if let Some(first_cubic) = first_cubic {
                cubics.push(Cubic::new(
                    last_cubic.anchor0(),
                    last_cubic.control0(),
                    last_cubic.control1(),
                    first_cubic.anchor0(),
                ));
            }
        }

        cubics
    }

    pub fn to_path<O, T: PathBuilder<O>>(
        &self,
        progress: f32,
        mut path: T,
        start_angle: u16,  // = 270,
        repeat_path: bool, // = false,
        close_path: bool,  // = true,
    ) -> impl PathBuilder<O> {
        let cubics = self.as_cubics(progress);
        let angle_to_first_cubic = (cubics[0].anchor0().y - self.start.center.y).atan2(cubics[0].anchor0().x - self.start.center.x);

        let mut first = true;
        let mut first_cubic: Option<Cubic> = None;

        path.rewind();

        for it in &cubics {
            if first {
                path.move_to(it.anchor0());

                if start_angle != 0 {
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

        if start_angle != 0 && first_cubic.is_some() {
            // Rotate the Path to to start from the given angle.
            path.transformed(Matrix3::rotation(0.0, 0.0, 1.0, Angle::degrees(-angle_to_first_cubic + f32::from(start_angle))))
        } else {
            path.transformed(Matrix3::identity())
        }
    }

    /// # Panics
    ///
    /// May panic if not all cubics of each polygon have been processed.
    pub fn match_morph(p1: &RoundedPolygon, p2: &RoundedPolygon) -> Vec<(Cubic, Cubic)> {
        // Measure polygons, returns lists of measured cubics for each polygon, which
        // we then use to match start/end curves
        let measured_polygon1 = MeasuredPolygon::measure_polygon(LengthMeasurer, p1);
        let measured_polygon2 = MeasuredPolygon::measure_polygon(LengthMeasurer, p2);

        // features1 and 2 will contain the list of corners (just the inner circular
        // curve) along with the progress at the middle of those corners. These
        // measurement values are then used to compare and match between the two
        // polygons
        let features1 = &measured_polygon1.features;
        let features2 = &measured_polygon2.features;

        let double_mapper = DoubleMapper::from_features(features1, features2);

        let polygon2_cut_point = double_mapper.map(0.0);

        let bs1 = measured_polygon1;
        let bs2 = measured_polygon2.cut_and_shift(polygon2_cut_point);

        // Match
        // Now we can compare the two lists of measured cubics and create a list of
        // pairs of cubics [ret], which are the start/end curves that represent
        // the Morph object and the start and end shapes, and which can be
        // interpolated to animate the between those shapes.
        let mut ret = <Vec<(Cubic, Cubic)>>::new();
        // i1/i2 are the indices of the current cubic on the start (1) and end (2)
        // shapes
        let mut i1 = 1;
        let mut i2 = 1;
        // b1, b2 are the current measured cubic for each polygon
        let mut b1 = bs1.cubics.get(i1).copied();
        let mut b2 = bs2.cubics.get(i2).copied();

        // Iterate until all curves are accounted for and matched
        while let (Some(bb1), Some(bb2)) = (b1.take(), b2.take()) {
            // Progresses are in shape1's perspective
            // b1a, b2a are ending progress values of current measured cubics in [0,1] range
            let b1a = if i1 == bs1.cubics.len() { 1.0 } else { bb1.end_outline_progress };
            let b2a = if i2 == bs2.cubics.len() {
                1.0
            } else {
                double_mapper.map_back((bb2.end_outline_progress + polygon2_cut_point).rem_euclid(1.0))
            };
            let minb = b1a.min(b2a);

            // min b is the progress at which the curve that ends first ends.
            // If both curves ends roughly there, no cutting is needed, we have a match.
            // If one curve extends beyond, we need to cut it.
            let (seg1, newb1) = if b1a > minb + ANGLE_EPSILON {
                let (a, b) = bb1.cut_at_progress(&bs1.measurer, minb);

                (a, Some(b))
            } else {
                (bb1, {
                    let value = bs1.cubics.get(i1);

                    i1 += 1;

                    value.copied()
                })
            };

            let (seg2, newb2) = if b2a > minb + ANGLE_EPSILON {
                let (a, b) = bb2.cut_at_progress(&bs2.measurer, (double_mapper.map(minb) - polygon2_cut_point).rem_euclid(1.0));

                (a, Some(b))
            } else {
                (bb2, {
                    let value = bs2.cubics.get(i2);

                    i2 += 1;

                    value.copied()
                })
            };

            ret.push((seg1.cubic, seg2.cubic));

            b1 = newb1;
            b2 = newb2;
        }

        assert!(b1.is_none() && b2.is_none(), "Expected both Polygon's Cubic to be fully matched");

        ret
    }
}
