use crate::numeric_traits::*;
use crate::*;

/// A circle in 2D, a sphere in 3D.
#[derive(Clone, Copy)]
pub struct Ball<T, const DIMENSIONS: usize> {
    pub center: Vector<T, DIMENSIONS>,
    pub radius: T,
}

impl<T, const DIMENSIONS: usize> Ball<T, DIMENSIONS> {
    pub fn new(center: Vector<T, DIMENSIONS>, radius: T) -> Self {
        Self { center, radius }
    }
}

#[derive(Clone, Copy)]
pub struct Line<T: Numeric, const DIMENSIONS: usize> {
    pub point: Vector<T, DIMENSIONS>,
    pub direction: Vector<T, DIMENSIONS>,
}

impl<T: Numeric + NumericSqrt, const DIMENSIONS: usize> Line<T, DIMENSIONS> {
    pub fn new(a: Vector<T, DIMENSIONS>, b: Vector<T, DIMENSIONS>) -> Self {
        let direction = (b - a).normalized();
        Self {
            point: a,
            direction,
        }
    }

    pub fn get_point(self, distance: T) -> Vector<T, DIMENSIONS> {
        self.point + self.direction * distance
    }
}

#[derive(Clone, Copy)]
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

impl<T: Numeric> BoundingBox<T, 2> {
    pub fn corners(&self) -> [Vector<T, 2>; 4] {
        [
            Vector::<T, 2>::new(self.min.x, self.min.y),
            Vector::<T, 2>::new(self.max.x, self.min.y),
            Vector::<T, 2>::new(self.max.x, self.max.y),
            Vector::<T, 2>::new(self.min.x, self.max.y),
        ]
    }
}

impl<T: Numeric> BoundingBox<T, 3> {
    pub fn corners(&self) -> [Vector<T, 3>; 8] {
        [
            Vector::<T, 3>::new(self.min.x, self.min.y, self.min.z),
            Vector::<T, 3>::new(self.max.x, self.min.y, self.min.z),
            Vector::<T, 3>::new(self.max.x, self.min.y, self.max.z),
            Vector::<T, 3>::new(self.min.x, self.min.y, self.max.z),
            Vector::<T, 3>::new(self.min.x, self.max.y, self.min.z),
            Vector::<T, 3>::new(self.max.x, self.max.y, self.min.z),
            Vector::<T, 3>::new(self.max.x, self.max.y, self.max.z),
            Vector::<T, 3>::new(self.min.x, self.max.y, self.max.z),
        ]
    }
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

/// Returns distance along the ray if it intersects.
/// The ray will not intersect if it points away from the plane or is parallel to the plane.
/// Call [Ray::get_point] with the return value to get the intersection point.
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
            Some(distance)
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

/// A Frustum. Does not include a far plane
#[derive(Debug)]
pub struct Frustum {
    // Left, right, top, bottom, near, far
    pub planes: [Plane3; 5],
}

impl Frustum {
    /// Creates an `unnormalized` [Frustum]. For culling it is not necessary that the [Frustum] be normalized.
    pub fn from_matrix(matrix: Mat4) -> Frustum {
        // http://www.cs.otago.ac.nz/postgrads/alexis/planeExtraction.pdf

        let row0 = matrix.row(0);
        let row1 = matrix.row(1);
        let row2 = matrix.row(2);
        let row3 = matrix.row(3);

        let left = row3 + row0;
        let right = row3 - row0;
        let top = row3 - row1;
        let bottom = row3 + row1;
        let near = row3 + row2;
        //let far = row3 - row2;

        // I don't understand why `distance_along_normal` is negated here, but it was needed for
        // a correct frustum to be generated.
        Frustum {
            planes: [
                Plane3 {
                    normal: left.xyz(),
                    distance_along_normal: -left[3],
                },
                Plane3 {
                    normal: right.xyz(),
                    distance_along_normal: -right[3],
                },
                Plane3 {
                    normal: top.xyz(),
                    distance_along_normal: -top[3],
                },
                Plane3 {
                    normal: bottom.xyz(),
                    distance_along_normal: -bottom[3],
                },
                Plane3 {
                    normal: near.xyz(),
                    distance_along_normal: -near[3],
                },
            ],
        }
    }

    pub fn intersects_box(&self, box3: Box3) -> bool {
        // For each plane check if all corners of the box are outside the plane
        for plane in self.planes {
            let mut corners_outside_plane = 0;
            for corner in box3.corners() {
                if plane.distance_to_point(corner) < 0.0 {
                    corners_outside_plane += 1;
                }
            }
            if corners_outside_plane == 8 {
                return false;
            }
        }

        // Some corners are not outside the plane
        // Todo: Additional cases should be checked for here: https://www.iquilezles.org/www/articles/frustumcorrect/frustumcorrect.htm
        return true;
    }
}
