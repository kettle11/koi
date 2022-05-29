use crate::geometry::*;
use crate::*;

pub trait IntersectionEpsilon {
    const EPSILON: Self;
}

impl IntersectionEpsilon for f32 {
    const EPSILON: Self = 0.00001;
}

impl IntersectionEpsilon for f64 {
    const EPSILON: Self = 0.00001;
}

fn closest_parametric_values_between_lines<
    T: NumericFloat + IntersectionEpsilon,
    const DIMENSIONS: usize,
>(
    p0: Vector<T, DIMENSIONS>,
    v0: Vector<T, DIMENSIONS>,
    p1: Vector<T, DIMENSIONS>,
    v1: Vector<T, DIMENSIONS>,
) -> Option<(T, T)> {
    // Based on the implementation described here:
    // http://paulbourke.net/geometry/pointlineplane/
    let p13 = p0 - p1;
    let p21 = v0;
    let p43 = v1;

    let d1343 = p13.dot(p43);
    let d4321 = p43.dot(p21);
    let d1321 = p13.dot(p21);
    let d4343 = p43.dot(p43);
    let d2121 = p21.dot(p21);

    let denom = d2121 * d4343 - d4321 * d4321;
    if denom.numeric_abs() < T::EPSILON {
        None
    } else {
        let numer = d1343 * d4321 - d1321 * d4343;
        let s = numer / denom;
        let t = (d1343 + d4321 * s) / d4343;
        Some((s, t))
    }
}

#[derive(Debug)]
pub enum LineIntersectionResult<T, const DIMENSIONS: usize> {
    Parallel,
    Point(Vector<T, DIMENSIONS>),
    ClosestPoints(Vector<T, DIMENSIONS>, Vector<T, DIMENSIONS>),
}

/// Returns that the lines are parallel, intersect at a point, or returns the two closest points.
pub fn line_line<
    T: NumericFloat + IntersectionEpsilon + std::fmt::Debug,
    const DIMENSIONS: usize,
>(
    l0: Line<T, DIMENSIONS>,
    l1: Line<T, DIMENSIONS>,
) -> LineIntersectionResult<T, DIMENSIONS> {
    if let Some((s, t)) =
        closest_parametric_values_between_lines(l0.point, l0.direction, l1.point, l1.direction)
    {
        let p0 = l0.direction * s + l0.point;
        let p1 = l1.direction * t + l1.point;
        if (p0 - p1).length_squared() < T::EPSILON {
            LineIntersectionResult::Point(p0)
        } else {
            LineIntersectionResult::ClosestPoints(p0, p1)
        }
    } else {
        LineIntersectionResult::Parallel
    }
}

#[test]
fn line_intersection() {
    let p1 = Vec3::new(3., 0., 0.);
    let p2 = Vec3::new(3., 3., 0.);
    let p3 = Vec3::new(0., 5., 0.);
    let p4 = Vec3::new(4., 5., 0.);

    let r0 = Line::new(p1, p2);
    let l1 = Line::new(p3, p4);

    let r0 = Ray::new(p1, (p2 - p1).normalized());
    let l1 = LineSegment::new(p4, p3);

    println!("l1: {:?}", l1);

    let result = ray_line_segment(r0, l1);
    println!("RESULT: {:?}", result);
    match result {
        LineIntersectionResult::Point(p) => {
            assert_eq!(p, Vec3::new(3., 5., 0.));
        }
        _ => panic!(),
    }
}

/// Returns that the lines are parallel, intersect at a point, or returns the two closest points.
/// The closest points may be on the ends of the segments.
/// Todo: This could be improved by removing the 'normalize' in Line.
pub fn line_segment_line_segment<
    T: NumericFloat + IntersectionEpsilon + std::fmt::Debug,
    const DIMENSIONS: usize,
>(
    s0: LineSegment<T, DIMENSIONS>,
    s1: LineSegment<T, DIMENSIONS>,
) -> LineIntersectionResult<T, DIMENSIONS> {
    let v0 = s0.b - s0.a;
    let v1 = s1.b - s1.a;
    if let Some((s, t)) = closest_parametric_values_between_lines(s0.a, v0, s1.a, v1) {
        let s = s.max_numeric(T::ZERO).min_numeric(T::ONE);
        let t = t.max_numeric(T::ZERO).min_numeric(T::ONE);

        let p0 = v0 * s + s0.a;
        let p1 = v1 * t + s1.a;
        if (p0 - p1).length_squared() < T::EPSILON {
            LineIntersectionResult::Point(p0)
        } else {
            LineIntersectionResult::ClosestPoints(p0, p1)
        }
    } else {
        LineIntersectionResult::Parallel
    }
}

/// Returns that the lines are parallel, intersect at a point, or returns the two closest points.
/// The closest points may be on the ends of the segment or the start of the ray.
pub fn ray_line_segment<T: NumericFloat + IntersectionEpsilon, const DIMENSIONS: usize>(
    ray: Ray<T, DIMENSIONS>,
    s1: LineSegment<T, DIMENSIONS>,
) -> LineIntersectionResult<T, DIMENSIONS> {
    let v1 = s1.b - s1.a;
    if let Some((s, t)) =
        closest_parametric_values_between_lines(ray.origin, ray.direction, s1.a, v1)
    {
        let s = s.max_numeric(T::ZERO);
        let t = t.max_numeric(T::ZERO).min_numeric(T::ONE);

        let p0 = ray.direction * s + ray.origin;
        let p1 = v1 * t + s1.a;
        if (p0 - p1).length_squared() < T::EPSILON {
            LineIntersectionResult::Point(p0)
        } else {
            LineIntersectionResult::ClosestPoints(p0, p1)
        }
    } else {
        LineIntersectionResult::Parallel
    }
}

/// Returns point on the line segment.
pub fn closest_point_on_line_segment<T: NumericFloat, const DIMENSIONS: usize>(
    point: Vector<T, DIMENSIONS>,
    line_segment: LineSegment<T, DIMENSIONS>,
) -> Vector<T, DIMENSIONS> {
    let ba = line_segment.b - line_segment.a;
    let pa = point - line_segment.a;
    let h = (ba.dot(pa) / ba.dot(ba))
        .numeric_max(T::ZERO)
        .numeric_min(T::ONE);
    line_segment.a + (ba * h)
}

/// Returns the point where a ray and plane intersect, if there is one.
/// The ray will not intersect if it points away from the plane or is parallel to the plane.
pub fn ray_with_plane_point<F: NumericFloat, const DIM: usize>(
    ray: Ray<F, DIM>,
    plane: Plane<F, DIM>,
) -> Option<Vector<F, DIM>> {
    ray_with_plane(ray, plane).map(|v| ray.get_point(v))
}

/// Returns distance along the ray if it intersects.
/// The ray will not intersect if it points away from the plane or is parallel to the plane.
/// Call [Ray::get_point] with the return value to get the intersection point or just use [ray_with_plane_point].
pub fn ray_with_plane<F: NumericFloat, const DIM: usize>(
    ray: Ray<F, DIM>,
    plane: Plane<F, DIM>,
) -> Option<F> {
    // https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection

    let bottom = ray.direction.dot(plane.normal);

    if bottom == F::ZERO {
        None // No intersection
    } else {
        let top = ray.origin.dot(plane.normal) - plane.distance_along_normal;
        if top == F::ZERO {
            None // Technically it intersects the entire plane, because the line is on the plane.
                 // However for now we're just saying it doesn't intersect.
        } else {
            let distance = top / bottom;
            Some(-distance)
        }
    }
}

/// Returns distance along the [Line] away from `line.point` if it intersects.
/// Call [Line::get_point] with the return value to get the intersection point.
/// If [None] is returned the entire [Line] is on the plane.
pub fn line_with_plane<F: NumericFloat, const DIM: usize>(
    line: Line<F, DIM>,
    plane: Plane<F, DIM>,
) -> Option<Vector<F, DIM>> {
    // https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
    let top = line.point.dot(plane.normal) - plane.distance_along_normal;
    let bottom = line.direction.dot(plane.normal);
    let d = top / bottom;
    if d == F::ZERO {
        None
    } else {
        Some(line.point - line.direction * (top / bottom))
    }
}

// Möller–Trumbore intersection algorithm
// https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
pub fn ray_with_tri(ray: Ray3, vertex0: Vec3, vertex1: Vec3, vertex2: Vec3) -> (bool, f32, Vec3) {
    const EPSILON: f32 = 0.000_000_1;
    let edge1 = vertex1 - vertex0;
    let edge2 = vertex2 - vertex0;
    let h = Vec3::cross(ray.direction, edge2);
    let a = Vec3::dot(edge1, h);

    if a > -EPSILON && a < EPSILON {
        return (false, 0., Vec3::ZERO);
    }

    let f = 1.0 / a;
    let s = ray.origin - vertex0;
    let u = f * Vec3::dot(s, h);

    if !(0.0..=1.0).contains(&u) {
        return (false, 0., Vec3::ZERO);
    }

    let q = Vec3::cross(s, edge1);
    let v = f * Vec3::dot(ray.direction, q);
    if v < 0.0 || u + v > 1.0 {
        return (false, 0., Vec3::ZERO);
    }

    // At this stage we can compute t to find out where the intersection point is on the line.
    let t = f * Vec3::dot(edge2, q);

    if t > EPSILON {
        let out_intersection_point = ray.origin + ray.direction * t;
        (true, t, out_intersection_point)
    } else {
        // This means that there is a line intersection but not a ray intersection.
        (false, t, Vec3::ZERO)
    }
}

// Brute force ray with mesh ray test.
pub fn ray_with_mesh(ray: Ray3, vertices: &[Vec3], indices: &[[u32; 3]]) -> Option<f32> {
    let mut nearest = std::f32::MAX;
    let mut intersects = false;
    for [i0, i1, i2] in indices.iter() {
        let result = ray_with_tri(
            ray,
            vertices[*i0 as usize],
            vertices[*i1 as usize],
            vertices[*i2 as usize],
        );

        if result.0 {
            let dis = result.1;

            if dis < nearest {
                nearest = dis;
                intersects = true;
            }
        }
    }

    if intersects {
        Some(nearest)
    } else {
        None
    }
}

// https://tavianator.com/fast-branchless-raybounding-box-intersections-part-2-nans/
pub fn ray_with_bounding_box<F: NumericFloat, const DIM: usize>(
    r: Ray<F, DIM>,
    b: geometry::BoundingBox<F, DIM>,
) -> Option<F> {
    // This could be cached for extra speed.
    let multiplicative_inverse = r.direction.reciprocal();

    let min_sub_origin_times_inverse = (b.min - r.origin).mul_by_component(multiplicative_inverse);
    let max_sub_origin_times_inverse = (b.max - r.origin).mul_by_component(multiplicative_inverse);

    let min = min_sub_origin_times_inverse.min(max_sub_origin_times_inverse);
    let max = min_sub_origin_times_inverse.max(max_sub_origin_times_inverse);

    let tmin = min.max_component();
    let tmax = max.min_component();

    let tmin = tmin.numeric_max(F::ZERO);
    if tmax >= tmin {
        Some(tmin)
    } else {
        None
    }
}

pub fn frustum_with_bounding_box(frustum: &Frustum, transform: Mat4, box3: Box3) -> bool {
    let corners = box3.corners();
    let corners = [
        transform.transform_point(corners[0]),
        transform.transform_point(corners[1]),
        transform.transform_point(corners[2]),
        transform.transform_point(corners[3]),
        transform.transform_point(corners[4]),
        transform.transform_point(corners[5]),
        transform.transform_point(corners[6]),
        transform.transform_point(corners[7]),
    ];
    // For each plane check if all corners of the box are outside the plane
    for plane in frustum.planes.iter() {
        let mut corners_outside_plane = 0;
        for corner in corners {
            if plane.signed_distance_to_point(corner) > 0.0 {
                corners_outside_plane += 1;
            }
        }
        if corners_outside_plane == 8 {
            return false;
        }
    }

    // Some corners are not outside the plane
    // Todo: Additional cases should be checked for here: https://www.iquilezles.org/www/articles/frustumcorrect/frustumcorrect.htm
    true
}
