use crate::numeric_traits::*;
use crate::*;

/// A circle in 2D, a sphere in 3D.
#[derive(Clone)]
pub struct Ball<T, const DIMENSIONS: usize> {
    pub center: Vector<T, DIMENSIONS>,
    pub radius: T,
}

impl<T, const DIMENSIONS: usize> Ball<T, DIMENSIONS> {
    pub fn new(center: Vector<T, DIMENSIONS>, radius: T) -> Self {
        Self { center, radius }
    }
}

pub struct Line<T, const DIMENSIONS: usize> {
    pub point: Vector<T, DIMENSIONS>,
    pub direction: Vector<T, DIMENSIONS>,
}

pub struct LineSegment<T, const DIMENSIONS: usize> {
    pub a: Vector<T, DIMENSIONS>,
    pub b: Vector<T, DIMENSIONS>,
}

// Returns magnitude of distance and the point
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

/// A rectangle in 2D, a rectangular prism in 3D.
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct BoundingBox<T, const DIMENSIONS: usize> {
    pub min: Vector<T, DIMENSIONS>,
    pub max: Vector<T, DIMENSIONS>,
}

impl<T: Numeric + PartialOrd + 'static, const DIMENSIONS: usize> BoundingBox<T, DIMENSIONS> {
    pub const ZERO: Self = Self {
        min: Vector::<T, DIMENSIONS>::ZERO,
        max: Vector::<T, DIMENSIONS>::ZERO,
    };

    pub fn new(min: Vector<T, DIMENSIONS>, max: Vector<T, DIMENSIONS>) -> Self {
        Self { min, max }
    }

    pub fn new_with_min_corner_and_size(
        min_corner: Vector<T, DIMENSIONS>,
        size: Vector<T, DIMENSIONS>,
    ) -> Self {
        Self {
            min: min_corner,
            max: min_corner + size,
        }
    }

    pub fn new_with_center_and_size(
        center: Vector<T, DIMENSIONS>,
        size: Vector<T, DIMENSIONS>,
    ) -> Self {
        let half_size = size / T::TWO;
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    pub fn size(self) -> Vector<T, DIMENSIONS> {
        self.max - self.min
    }

    pub fn from_points<'a>(points: impl IntoIterator<Item = &'a Vector<T, DIMENSIONS>>) -> Self {
        let (min, max) = points.into_iter().fold(
            (Vector::<T, DIMENSIONS>::MAX, Vector::<T, DIMENSIONS>::MIN),
            |(min, max), v| (min.min(*v), max.max(*v)),
        );
        BoundingBox { min, max }
    }

    pub fn contains_point(&self, point: Vector<T, DIMENSIONS>) -> bool {
        point.greater_than_per_component(self.min).all()
            && point.less_than_per_component(self.max).all()
    }

    pub fn contains_bounding_box(&self, bounding_box: Self) -> bool {
        let joined = self.join(bounding_box);
        joined == *self
    }

    /// Returns the area of a 2D `BoundingBox`, or the volume of a 3D `BoundingBox`
    pub fn area(&self) -> T {
        let size = self.max - self.min;
        let mut a = size[0];
        for v in &size.0[0][1..] {
            a = a * *v;
        }
        a
    }

    /// Creates a new `BoundingBox` that encompasses `self` and `other`
    /// Also referred to as a `union` operation
    pub fn join(self, other: Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Creates a new `BoundingBox` with only the part that is contained in both `BoundingBox`s
    /// Returns `None` otherwise.
    pub fn intersection(self, other: Self) -> Self {
        let mut new_bounds = Self {
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        };
        for i in 0..DIMENSIONS {
            if new_bounds.min[i] > new_bounds.max[i] {
                new_bounds.min[i] = new_bounds.max[i];
            }
        }
        new_bounds
    }

    pub fn center(self) -> Vector<T, DIMENSIONS> {
        (self.max - self.min) / T::TWO + self.min
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Ray<T: NumericFloat, const DIM: usize> {
    pub origin: Vector<T, DIM>,
    pub direction: Vector<T, DIM>,
}

impl<T: NumericFloat, const DIM: usize> Ray<T, DIM> {
    pub fn new(origin: Vector<T, DIM>, direction: Vector<T, DIM>) -> Self {
        let direction = direction.normalized();
        Self { origin, direction }
    }
    pub fn get_point(self, distance: T) -> Vector<T, DIM> {
        self.origin + self.direction * distance
    }
}

impl<T: NumericFloat> Matrix<T, 4, 4> {
    pub fn transform_ray(&self, ray: Ray<T, 3>) -> Ray<T, 3> {
        let direction = self.transform_vector(ray.direction);
        Ray {
            origin: self.transform_point(ray.origin),
            direction,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Plane<T: NumericFloat, const DIM: usize> {
    pub normal: Vector<T, DIM>,
    pub distance_along_normal: T,
}

impl<T: NumericFloat, const DIM: usize> Plane<T, DIM> {
    pub fn new(normal: Vector<T, DIM>, point_on_plane: Vector<T, DIM>) -> Self {
        let distance_along_normal = normal.dot(point_on_plane);
        Plane {
            normal,
            distance_along_normal,
        }
    }

    pub fn distance_to_point(&self, point: Vector<T, DIM>) -> T {
        self.normal.dot(point) - self.distance_along_normal
    }
}

// https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
/// Returns distance along the ray if it intersects.
/// The ray will not intersect if it points away from the plane or is parallel to the plane.
pub fn ray_with_plane<F: NumericFloat, const DIM: usize>(
    ray: Ray<F, DIM>,
    plane: Plane<F, DIM>,
) -> Option<F> {
    let bottom = ray.direction.dot(plane.normal);

    if bottom == F::ZERO {
        None // No intersection
    } else {
        let top = ((plane.normal * plane.distance_along_normal) - ray.origin).dot(plane.normal);

        if top == F::ZERO {
            None // Technically it intersects the entire plane, because the line is on the plane.
                 // However for now we're just saying it doesn't intersect.
        } else {
            let distance = top / bottom;
            Some(distance)
        }
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

/*
pub struct Frustum {
    pub left: Plane,
    pub right: Plane,
    pub top: Plane,
    pub bottom: Plane,
    pub near: Plane,
    pub far: Plane,
}

impl Frustum {
    pub fn from_matrix(matrix: &Mat4) -> Frustum {
        let row0 = matrix.row(0);
        let row1 = matrix.row(1);
        let row2 = matrix.row(2);
        let row3 = matrix.row(3);

        let left = (row3 + row0).normalized();
        let right = (row3 - row0).normalized();
        let top = (row3 - row1).normalized();
        let bottom = (row3 + row1).normalized();
        let near = (row3 + row2).normalized();
        let far = (row3 - row2).normalized();

        Frustum {
            left: Plane::new(left[3], left.xyz()),
            right: Plane::new(right[3], right.xyz()),
            top: Plane::new(top[3], top.xyz()),
            bottom: Plane::new(bottom[3], bottom.xyz()),
            near: Plane::new(near[3], near.xyz()),
            far: Plane::new(far[3], far.xyz()),
        }
    }
}
*/
