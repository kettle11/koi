use crate::geometry::*;
use crate::*;

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
) -> (bool, F) {
    // This could be cached for extra speed.
    let multiplicative_inverse = r.direction.reciprocal();

    let min_sub_origin_times_inverse = (b.min - r.origin).mul_by_component(multiplicative_inverse);
    let max_sub_origin_times_inverse = (b.max - r.origin).mul_by_component(multiplicative_inverse);

    let min = min_sub_origin_times_inverse.min(max_sub_origin_times_inverse);
    let max = min_sub_origin_times_inverse.max(max_sub_origin_times_inverse);

    let tmin = min.max_component();
    let tmax = max.min_component();

    let tmin = tmin.numeric_max(F::ZERO);
    (tmax >= tmin, tmin)
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
            if plane.distance_to_point(corner) > 0.0 {
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
