mod matrix;
pub mod numeric_traits;
mod quaternion;
mod vector;

/// Commonly used projection matrices
pub mod projection_matrices;

use numeric_traits::*;

pub use matrix::*;
pub use quaternion::*;
pub use vector::*;
pub use vector::*;

mod default_types {
    use super::{Matrix, Quaternion, Vector};
    use crate::geometry::*;

    pub type Vec2 = Vector<f32, 2>;
    pub type Vec3 = Vector<f32, 3>;
    pub type Vec4 = Vector<f32, 4>;

    pub type Vec2i = Vector<i32, 2>;
    pub type Vec3i = Vector<i32, 3>;
    pub type Vec4i = Vector<i32, 4>;

    pub type Vec2u = Vector<usize, 2>;
    pub type Vec3u = Vector<usize, 3>;
    pub type Vec4u = Vector<usize, 4>;

    pub type Mat3 = Matrix<f32, 3, 3>;
    pub type Mat4 = Matrix<f32, 4, 4>;

    pub type Quat = Quaternion<f32>;

    pub type Box2 = BoundingBox<f32, 2>;
    pub type Box3 = BoundingBox<f32, 3>;

    pub type Ray2 = Ray<f32, 2>;
    pub type Ray3 = Ray<f32, 3>;

    pub type Plane2 = crate::geometry::Plane<f32, 2>;
    pub type Plane3 = crate::geometry::Plane<f32, 3>;

    pub type Line2 = Line<f32, 2>;
    pub type Line3 = Line<f32, 3>;
}
pub use default_types::*;

pub use geometry::{Frustum, Line, Plane};

// #[cfg(test)]
// mod tests;

pub mod geometry;
pub mod intersections;
