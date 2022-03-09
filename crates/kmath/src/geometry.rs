use crate::numeric_traits::*;
use crate::*;

/// A circle in 2D, a sphere in 3D.
#[derive(Clone, Copy, Debug)]
pub struct Ball<T, const DIMENSIONS: usize> {
    pub center: Vector<T, DIMENSIONS>,
    pub radius: T,
}

impl<T, const DIMENSIONS: usize> Ball<T, DIMENSIONS> {
    pub fn new(center: Vector<T, DIMENSIONS>, radius: T) -> Self {
        Self { center, radius }
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub struct LineSegment<T, const DIMENSIONS: usize> {
    pub a: Vector<T, DIMENSIONS>,
    pub b: Vector<T, DIMENSIONS>,
}

impl<T, const DIMENSIONS: usize> LineSegment<T, DIMENSIONS> {
    pub fn new(a: Vector<T, DIMENSIONS>, b: Vector<T, DIMENSIONS>) -> Self {
        Self { a, b }
    }
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

    /// Returns a new bounding box inflated by `amounnt` on each side.
    pub fn inflated(self, amount: T) -> Self {
        let v = Vector::<T, DIMENSIONS>::fill(amount);
        Self {
            min: self.min - v,
            max: self.max + v,
        }
    }

    pub fn size(self) -> Vector<T, DIMENSIONS> {
        self.max - self.min
    }

    pub fn from_points<'a>(points: impl IntoIterator<Item = Vector<T, DIMENSIONS>>) -> Self {
        let (min, max) = points.into_iter().fold(
            (Vector::<T, DIMENSIONS>::MAX, Vector::<T, DIMENSIONS>::MIN),
            |(min, max), v| (min.min(v), max.max(v)),
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

    pub fn transform_plane(&self, plane: Plane<T, 3>) -> Plane<T, 3> {
        let normal = self.transform_vector(plane.normal).normalized();
        // This could probably be more efficient.
        let distance_along_normal =
            (self.transform_point(plane.normal * plane.distance_along_normal)).dot(normal);
        Plane {
            normal,
            distance_along_normal,
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

    pub fn signed_distance_to_point(&self, point: Vector<T, DIM>) -> T {
        self.normal.dot(point) - self.distance_along_normal
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

        Frustum {
            planes: [
                Plane3 {
                    normal: -left.xyz(),
                    distance_along_normal: left[3],
                },
                Plane3 {
                    normal: -right.xyz(),
                    distance_along_normal: right[3],
                },
                Plane3 {
                    normal: -top.xyz(),
                    distance_along_normal: top[3],
                },
                Plane3 {
                    normal: -bottom.xyz(),
                    distance_along_normal: bottom[3],
                },
                Plane3 {
                    normal: -near.xyz(),
                    distance_along_normal: near[3],
                },
            ],
        }
    }
}
