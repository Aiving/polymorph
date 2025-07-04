use core::f32;

use crate::{
    CornerRounding, RoundedPolygon,
    geometry::{Point, Size, Vector},
    util::radial_to_cartesian,
};

pub trait HaveRounding {}

pub trait HaveSize {
    fn size(&mut self) -> &mut Size;
}

pub trait HaveRadius {
    fn radius(&mut self) -> &mut f32;
}

pub struct Rectangle {
    pub(crate) size: Size,
}

impl HaveSize for Rectangle {
    fn size(&mut self) -> &mut Size {
        &mut self.size
    }
}

pub struct Circle {
    pub(crate) vertices: usize,
    pub(crate) radius: f32,
}

impl HaveRadius for Circle {
    fn radius(&mut self) -> &mut f32 {
        &mut self.radius
    }
}

pub struct Star {
    pub(crate) vertices_per_radius: usize,
    pub(crate) radius: f32,
    pub(crate) inner_radius: f32,
    pub(crate) inner_rounding: Option<CornerRounding>,
}

impl HaveRadius for Star {
    fn radius(&mut self) -> &mut f32 {
        &mut self.radius
    }
}

impl HaveRounding for Star {}

pub struct Pill {
    pub(crate) size: Size,
    pub(crate) smoothing: f32,
}

impl HaveSize for Pill {
    fn size(&mut self) -> &mut Size {
        &mut self.size
    }
}

pub struct PillStar {
    pub(crate) size: Size,
    pub(crate) vertices_per_radius: usize,
    pub(crate) inner_radius_ratio: f32,
    pub(crate) inner_rounding: Option<CornerRounding>,
    pub(crate) vertex_spacing: f32,
    pub(crate) start_location: f32,
}

impl HaveSize for PillStar {
    fn size(&mut self) -> &mut Size {
        &mut self.size
    }
}

impl HaveRounding for PillStar {}

pub struct RoundedPolygonBuilder<T> {
    pub(crate) data: T,
    pub(crate) center: Point,
    pub(crate) rounding: CornerRounding,
    pub(crate) per_vertex_rounding: Vec<CornerRounding>,
}

impl<T> RoundedPolygonBuilder<T> {
    #[must_use]
    pub const fn with_center(mut self, center: Point) -> Self {
        self.center = center;

        self
    }
}

impl<T: HaveRounding> RoundedPolygonBuilder<T> {
    #[must_use]
    pub const fn with_rounding(mut self, rounding: CornerRounding) -> Self {
        self.rounding = rounding;

        self
    }

    #[must_use]
    pub fn with_rounding_per_vertex<I: IntoIterator<Item = CornerRounding>>(mut self, iter: I) -> Self {
        self.per_vertex_rounding = iter.into_iter().collect();

        self
    }
}

impl<T: HaveSize> RoundedPolygonBuilder<T> {
    #[must_use]
    pub fn with_size(mut self, size: Size) -> Self {
        *self.data.size() = size;

        self
    }

    #[must_use]
    pub fn with_width(mut self, width: f32) -> Self {
        self.data.size().width = width;

        self
    }

    #[must_use]
    pub fn with_height(mut self, height: f32) -> Self {
        self.data.size().height = height;

        self
    }
}

impl<T: HaveRadius> RoundedPolygonBuilder<T> {
    #[must_use]
    pub fn with_radius(mut self, radius: f32) -> Self {
        *self.data.radius() = radius;

        self
    }
}

impl RoundedPolygonBuilder<Circle> {
    #[must_use]
    pub const fn with_vertices(mut self, vertices: usize) -> Self {
        self.data.vertices = vertices;

        self
    }

    pub fn build(self) -> RoundedPolygon {
        let theta = f32::consts::PI / self.data.vertices as f32;
        let polygon_radius = self.data.radius / theta.cos();

        RoundedPolygon::from_vertices_count_at(self.data.vertices, polygon_radius, self.center, Some(CornerRounding::new(self.data.radius)), &[
        ])
    }
}

impl RoundedPolygonBuilder<Rectangle> {
    #[must_use]
    pub const fn with_rounding(mut self, rounding: CornerRounding) -> Self {
        self.rounding = rounding;

        self
    }

    #[must_use]
    pub fn with_rounding_per_vertex(mut self, corners: [CornerRounding; 4]) -> Self {
        self.per_vertex_rounding = corners.into();

        self
    }

    pub fn build(self) -> RoundedPolygon {
        let [left, top] = (self.center - self.data.size / 2.0).to_array();
        let [right, bottom] = (self.center + self.data.size / 2.0).to_array();

        let vertices = [
            Point::new(right, bottom),
            Point::new(left, bottom),
            Point::new(left, top),
            Point::new(right, top),
        ];

        RoundedPolygon::from_vertices(&vertices, self.rounding, &self.per_vertex_rounding, self.center)
    }
}

impl RoundedPolygonBuilder<Star> {
    #[must_use]
    pub const fn with_inner_radius(mut self, radius: f32) -> Self {
        self.data.inner_radius = radius;

        self
    }

    #[must_use]
    pub const fn with_inner_rounding(mut self, rounding: CornerRounding) -> Self {
        self.data.inner_rounding.replace(rounding);

        self
    }

    pub fn build(self) -> RoundedPolygon {
        let vertices = star_vertices_from_num_verts(self.data.vertices_per_radius, self.data.radius, self.data.inner_radius, self.center);

        // Star polygon is just a polygon with all vertices supplied (where we generate
        // those vertices to be on the inner/outer radii)
        if !self.per_vertex_rounding.is_empty() {
            RoundedPolygon::from_vertices(&vertices, self.rounding, &self.per_vertex_rounding, self.center)
        } else if let Some(inner_rounding) = self.data.inner_rounding {
            // If no per-vertex rounding supplied and caller asked for inner
            // rounding, create per-vertex rounding list based on
            // supplied outer/inner rounding parameters
            RoundedPolygon::from_vertices(
                &vertices,
                self.rounding,
                &(0..self.data.vertices_per_radius)
                    .flat_map(|_| [self.rounding, inner_rounding])
                    .collect::<Vec<_>>(),
                self.center,
            )
        } else {
            RoundedPolygon::from_vertices(&vertices, self.rounding, &[], self.center)
        }
    }
}

impl RoundedPolygonBuilder<Pill> {
    #[must_use]
    pub const fn with_smoothing(mut self, smoothing: f32) -> Self {
        self.data.smoothing = smoothing;

        self
    }

    pub fn build(self) -> RoundedPolygon {
        let half_size = self.data.size / 2.0;

        RoundedPolygon::from_vertices(
            &[
                self.center + half_size,
                self.center + Vector::new(-half_size.width, half_size.height),
                self.center - half_size,
                self.center + Vector::new(half_size.width, -half_size.height),
            ],
            CornerRounding::smoothed(half_size.width.min(half_size.height), self.data.smoothing),
            &[],
            self.center,
        )
    }
}

impl RoundedPolygonBuilder<PillStar> {
    #[must_use]
    pub const fn with_vertices_per_radius(mut self, count: usize) -> Self {
        self.data.vertices_per_radius = count;

        self
    }

    #[must_use]
    pub const fn with_vertex_spacing(mut self, spacing: f32) -> Self {
        self.data.vertex_spacing = spacing;

        self
    }

    #[must_use]
    pub const fn with_start_location(mut self, location: f32) -> Self {
        self.data.start_location = location;

        self
    }

    #[must_use]
    pub const fn with_inner_radius_ratio(mut self, radius: f32) -> Self {
        self.data.inner_radius_ratio = radius;

        self
    }

    #[must_use]
    pub const fn with_inner_rounding(mut self, rounding: CornerRounding) -> Self {
        self.data.inner_rounding.replace(rounding);

        self
    }

    pub fn build(self) -> RoundedPolygon {
        let vertices = pill_star_vertices_from_num_verts(
            self.data.vertices_per_radius,
            self.data.size,
            self.data.inner_radius_ratio,
            self.data.vertex_spacing,
            self.data.start_location,
            self.center,
        );

        if !self.per_vertex_rounding.is_empty() {
            RoundedPolygon::from_vertices(&vertices, self.rounding, &self.per_vertex_rounding, self.center)
        } else if let Some(inner_rounding) = self.data.inner_rounding {
            // If no per-vertex rounding supplied and caller asked for inner
            // rounding, create per-vertex rounding list based on
            // supplied outer/inner rounding parameters
            RoundedPolygon::from_vertices(
                &vertices,
                self.rounding,
                &(0..self.data.vertices_per_radius)
                    .flat_map(|_| [self.rounding, inner_rounding])
                    .collect::<Vec<_>>(),
                self.center,
            )
        } else {
            RoundedPolygon::from_vertices(&vertices, self.rounding, &[], self.center)
        }
    }
}

fn pill_star_vertices_from_num_verts(
    num_vertices_per_radius: usize,
    size: Size,
    inner_radius: f32,
    vertex_spacing: f32,
    start_location: f32,
    center: Point,
) -> Vec<Point> {
    // The general approach here is to get the perimeter of the underlying pill
    // outline, then the t value for each vertex as we walk that perimeter. This
    // tells us where on the outline to place that vertex, then we figure out
    // where to place the vertex depending on which "section" it is in. The
    // possible sections are the vertical edges on the sides, the circular
    // sections on all four corners, or the horizontal edges on the top and
    // bottom. Note that either the vertical or horizontal edges will be
    // of length zero (whichever dimension is smaller gets only circular curvature
    // for the pill shape).
    let endcap_radius = size.width.min(size.height);
    let v_seg_len = (size.height - size.width).max(0.0);
    let h_seg_len = (size.width - size.height).max(0.0);
    let v_seg_half = v_seg_len / 2.0;
    let h_seg_half = h_seg_len / 2.0;
    // vertexSpacing is used to position the vertices on the end caps. The caller
    // has the choice of spacing the inner (0) or outer (1) vertices like those
    // along the edges, causing the other vertices to be either further apart
    // (0) or closer (1). The default is .5, which averages things. The
    // magnitude of the inner and rounding parameters may cause the caller
    // to want a different value.
    let circle_perimeter = f32::consts::PI * 2.0 * endcap_radius * inner_radius.mul_add(1.0 - vertex_spacing, 1.0 * vertex_spacing);
    // perimeter is circle perimeter plus horizontal and vertical sections of inner
    // rectangle, whether either (or even both) might be of length zero.
    let perimeter = 2.0f32.mul_add(h_seg_len, 2.0 * v_seg_len) + circle_perimeter;

    // The sections array holds the t start values of that part of the outline. We
    // use these to determine which section a given vertex lies in, based on its
    // t value, as well as where in that section it lies.
    let mut sections = [0.0; 11];

    sections[1] = v_seg_len / 2.0;
    sections[2] = sections[1] + circle_perimeter / 4.0;
    sections[3] = sections[2] + h_seg_len;
    sections[4] = sections[3] + circle_perimeter / 4.0;
    sections[5] = sections[4] + v_seg_len;
    sections[6] = sections[5] + circle_perimeter / 4.0;
    sections[7] = sections[6] + h_seg_len;
    sections[8] = sections[7] + circle_perimeter / 4.0;
    sections[9] = sections[8] + v_seg_len / 2.0;
    sections[10] = perimeter;

    // "t" is the length along the entire pill outline for a given vertex. With
    // vertices spaced evenly along this contour, we can determine for any
    // vertex where it should lie.
    let t_per_vertex = perimeter / (2 * num_vertices_per_radius) as f32;
    // separate iteration for inner vs outer, unlike the other shapes, because
    // the vertices can lie in different quadrants so each needs their own
    // calculation
    let mut inner = false;
    // Increment section index as we walk around the pill contour with our
    // increasing t values
    let mut curr_sec_index = 0;
    // secStart/End are used to determine how far along a given vertex is in the
    // section in which it lands
    let mut sec_start = 0.0;
    let mut sec_end = sections[1];
    // t value is used to place each vertex. 0 is on the positive x axis,
    // moving into section 0 to begin with. startLocation, a value from 0 to 1,
    // varies the location anywhere on the perimeter of the shape
    let mut t = start_location * perimeter;
    // The list of vertices to be returned
    let mut result = Vec::with_capacity(num_vertices_per_radius * 2);
    let rect_bottom_right = Point::new(h_seg_half, v_seg_half);
    let rect_bottom_left = Point::new(-h_seg_half, v_seg_half);
    let rect_top_left = Point::new(-h_seg_half, -v_seg_half);
    let rect_top_right = Point::new(h_seg_half, -v_seg_half);

    // Each iteration through this loop uses the next t value as we walk around the
    // shape
    for _ in 0..num_vertices_per_radius * 2 {
        // t could start (and end) after 0; extra boundedT logic makes sure it does the
        // right thing when crossing the boundar past 0 again
        let bounded_t = t % perimeter;

        if bounded_t < sec_start {
            curr_sec_index = 0;
        }

        #[allow(clippy::while_float)]
        while bounded_t >= sections[(curr_sec_index + 1) % sections.len()] {
            curr_sec_index = (curr_sec_index + 1) % sections.len();
            sec_start = sections[curr_sec_index];
            sec_end = sections[(curr_sec_index + 1) % sections.len()];
        }

        // find t in section and its proportion of that section's total length
        let t_in_section = bounded_t - sec_start;
        let t_proportion = t_in_section / (sec_end - sec_start);

        // The vertex placement in a section varies depending on whether it is on one of
        // the semicircle endcaps or along one of the straight edges. For the
        // endcaps, we use tProportion to get the angle along that circular cap
        // and add the starting angle for that section. For the edges we use a
        // straight linear calculation given tProportion and the start/end t
        // values for that edge.
        let curr_radius = if inner { endcap_radius * inner_radius } else { endcap_radius };
        let vertex = match curr_sec_index {
            0 => Point::new(curr_radius, t_proportion * v_seg_half),
            1 => rect_bottom_right + radial_to_cartesian(curr_radius, t_proportion * f32::consts::PI / 2.0),
            2 => Point::new(t_proportion.mul_add(-h_seg_len, h_seg_half), curr_radius),
            3 => rect_bottom_left + radial_to_cartesian(curr_radius, f32::consts::PI / 2.0 + (t_proportion * f32::consts::PI / 2.0)),
            4 => Point::new(-curr_radius, t_proportion.mul_add(-v_seg_len, v_seg_half)),
            5 => rect_top_left + radial_to_cartesian(curr_radius, f32::consts::PI + (t_proportion * f32::consts::PI / 2.0)),
            6 => Point::new(t_proportion.mul_add(h_seg_len, -h_seg_half), -curr_radius),
            7 => rect_top_right + radial_to_cartesian(curr_radius, f32::consts::PI.mul_add(1.5, t_proportion * f32::consts::PI / 2.0)),
            // 8
            _ => Point::new(curr_radius, t_proportion.mul_add(v_seg_half, -v_seg_half)),
        };

        result.push(vertex + center.to_vector());

        t += t_per_vertex;

        inner = !inner;
    }

    result
}

fn star_vertices_from_num_verts(num_vertices_per_radius: usize, radius: f32, inner_radius: f32, center: Point) -> Vec<Point> {
    (0..num_vertices_per_radius * 2)
        .map(|i| {
            center
                + radial_to_cartesian(
                    if i % 2 == 0 { radius } else { inner_radius },
                    f32::consts::PI / num_vertices_per_radius as f32 * i as f32,
                )
        })
        .collect()
}
