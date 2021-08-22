pub(crate) trait Cuberoot {
    fn cbrt(&self) -> Self;
}

pub(crate) mod prelude {
    #[cfg(feature = "glam")]
    pub(crate) use glam::Vec3Swizzles;

    pub(crate) use super::Cuberoot;
}

pub use math::*;

#[cfg(feature = "glam")]
mod math {
    #[cfg(not(feature = "f64"))]
    pub use glam::f32::Mat3;
    #[cfg(not(feature = "f64"))]
    pub use glam::f32::Vec3;

    #[cfg(feature = "f64")]
    pub use glam::f64::DMat3 as Mat3;
    #[cfg(feature = "f64")]
    pub use glam::f64::DVec3 as Vec3;

    #[cfg(all(not(feature = "std"), feature = "libm"))]
    use num_traits::Float;

    impl super::Cuberoot for Vec3 {
        #[inline]
        fn cbrt(&self) -> Self {
            Self::new(self.x.cbrt(), self.y.cbrt(), self.z.cbrt())
        }
    }
}

#[cfg(not(feature = "glam"))]
mod math {
    use crate::FType;
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    use num_traits::Float;
    use std::ops::{Add, Div, Mul, MulAssign, Sub};

    #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Vec3 {
        pub x: FType,
        pub y: FType,
        pub z: FType,
    }

    pub struct BVec3 {
        pub x: bool,
        pub y: bool,
        pub z: bool,
    }

    impl Vec3 {
        pub const fn new(x: FType, y: FType, z: FType) -> Self {
            Self { x, y, z }
        }

        pub const fn splat(value: FType) -> Self {
            Self {
                x: value,
                y: value,
                z: value,
            }
        }

        pub const fn from_slice(slice: &[FType]) -> Self {
            Self {
                x: slice[0],
                y: slice[1],
                z: slice[2],
            }
        }

        pub fn powf(self, n: FType) -> Self {
            Self {
                x: self.x.powf(n),
                y: self.y.powf(n),
                z: self.z.powf(n),
            }
        }

        pub fn cmplt(self, other: Self) -> BVec3 {
            BVec3 {
                x: self.x < other.x,
                y: self.y < other.y,
                z: self.z < other.z,
            }
        }

        pub const fn select(mask: BVec3, if_true: Vec3, if_false: Vec3) -> Self {
            Self {
                x: if mask.x { if_true.x } else { if_false.x },
                y: if mask.y { if_true.y } else { if_false.y },
                z: if mask.z { if_true.z } else { if_false.z },
            }
        }

        pub const fn yz(self) -> Vec2 {
            Vec2 {
                x: self.y,
                y: self.z,
            }
        }

        pub fn dot(self, other: Self) -> FType {
            self.x * other.x + self.y * other.y + self.z * other.z
        }
    }

    impl super::Cuberoot for Vec3 {
        #[inline]
        fn cbrt(&self) -> Self {
            Self::new(self.x.cbrt(), self.y.cbrt(), self.z.cbrt())
        }
    }

    impl Add for Vec3 {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl Sub for Vec3 {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl Mul for Vec3 {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            Self {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }

    impl MulAssign<FType> for Vec3 {
        fn mul_assign(&mut self, other: FType) {
            *self = *self * other
        }
    }

    impl Mul<FType> for Vec3 {
        type Output = Self;

        fn mul(self, other: FType) -> Self {
            Self {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other,
            }
        }
    }

    impl Mul<Vec3> for FType {
        type Output = Vec3;

        fn mul(self, other: Vec3) -> Vec3 {
            other * self
        }
    }

    impl Div<FType> for Vec3 {
        type Output = Self;

        fn div(self, other: FType) -> Self {
            Self {
                x: self.x / other,
                y: self.y / other,
                z: self.z / other,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Vec2 {
        pub x: FType,
        pub y: FType,
    }

    impl Vec2 {
        pub fn length(self) -> FType {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }

    #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Mat3 {
        pub x_axis: Vec3,
        pub y_axis: Vec3,
        pub z_axis: Vec3,
    }

    impl Mat3 {
        pub const IDENTITY: Self = Self {
            x_axis: Vec3::new(1.0, 0.0, 0.0),
            y_axis: Vec3::new(0.0, 1.0, 0.0),
            z_axis: Vec3::new(0.0, 0.0, 1.0),
        };

        pub(crate) const fn from_cols_array_const(m: [FType; 9]) -> Self {
            Self {
                x_axis: Vec3::new(m[0], m[1], m[2]),
                y_axis: Vec3::new(m[3], m[4], m[5]),
                z_axis: Vec3::new(m[6], m[7], m[8]),
            }
        }

        pub fn from_cols_array(m: &[FType; 9]) -> Self {
            Self {
                x_axis: Vec3::new(m[0], m[1], m[2]),
                y_axis: Vec3::new(m[3], m[4], m[5]),
                z_axis: Vec3::new(m[6], m[7], m[8]),
            }
        }

        pub fn transpose(&self) -> Self {
            Self {
                x_axis: Vec3::new(self.x_axis.x, self.y_axis.x, self.z_axis.x),
                y_axis: Vec3::new(self.x_axis.y, self.y_axis.y, self.z_axis.y),
                z_axis: Vec3::new(self.x_axis.z, self.y_axis.z, self.z_axis.z),
            }
        }

        pub fn inverse(&self) -> Self {
            let m00 = self.x_axis.x;
            let m01 = self.y_axis.x;
            let m02 = self.z_axis.x;

            let m10 = self.x_axis.y;
            let m11 = self.y_axis.y;
            let m12 = self.z_axis.y;

            let m20 = self.x_axis.z;
            let m21 = self.y_axis.z;
            let m22 = self.z_axis.z;

            let det = m00 * (m11 * m22 - m21 * m12) - m01 * (m10 * m22 - m12 * m20)
                + m02 * (m10 * m21 - m11 * m20);

            let inv_det = 1.0 / det;

            let result = Self {
                x_axis: Vec3::new(
                    (m11 * m22 - m21 * m12) * inv_det,
                    (m12 * m20 - m10 * m22) * inv_det,
                    (m10 * m21 - m20 * m11) * inv_det,
                ),
                y_axis: Vec3::new(
                    (m02 * m21 - m01 * m22) * inv_det,
                    (m00 * m22 - m02 * m20) * inv_det,
                    (m20 * m01 - m00 * m21) * inv_det,
                ),
                z_axis: Vec3::new(
                    (m01 * m12 - m02 * m11) * inv_det,
                    (m10 * m02 - m00 * m12) * inv_det,
                    (m00 * m11 - m10 * m01) * inv_det,
                ),
            };

            result
        }
    }

    impl Mul<Vec3> for Mat3 {
        type Output = Vec3;
        #[inline]
        fn mul(self, other: Vec3) -> Vec3 {
            let r0 = Vec3::new(self.x_axis.x, self.y_axis.x, self.z_axis.x);
            let r1 = Vec3::new(self.x_axis.y, self.y_axis.y, self.z_axis.y);
            let r2 = Vec3::new(self.x_axis.z, self.y_axis.z, self.z_axis.z);

            Vec3::new(r0.dot(other), r1.dot(other), r2.dot(other))
        }
    }

    impl Mul<Mat3> for Mat3 {
        type Output = Mat3;
        #[inline]
        fn mul(self, other: Self) -> Self {
            let r0 = Vec3::new(self.x_axis.x, self.y_axis.x, self.z_axis.x);
            let r1 = Vec3::new(self.x_axis.y, self.y_axis.y, self.z_axis.y);
            let r2 = Vec3::new(self.x_axis.z, self.y_axis.z, self.z_axis.z);

            Self {
                x_axis: Vec3::new(
                    r0.dot(other.x_axis),
                    r1.dot(other.x_axis),
                    r2.dot(other.x_axis),
                ),
                y_axis: Vec3::new(
                    r0.dot(other.y_axis),
                    r1.dot(other.y_axis),
                    r2.dot(other.y_axis),
                ),
                z_axis: Vec3::new(
                    r0.dot(other.z_axis),
                    r1.dot(other.z_axis),
                    r2.dot(other.z_axis),
                ),
            }
        }
    }

    impl From<[FType; 3]> for Vec3 {
        fn from(values: [FType; 3]) -> Self {
            Self {
                x: values[0],
                y: values[1],
                z: values[2],
            }
        }
    }

    impl From<Vec3> for [FType; 3] {
        fn from(v: Vec3) -> Self {
            [v.x, v.y, v.z]
        }
    }
}
