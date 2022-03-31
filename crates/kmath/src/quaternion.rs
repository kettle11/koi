use std::ops::{Add, Index, Mul, Neg};

use crate::*;

#[derive(Copy, Clone, Debug, Hash)]
pub struct Quaternion<T: NumericFloat>(pub(crate) Vector<T, 4>);

use kserde::*;
// Manually tweaked serialization / deserialization implementations.
impl<KSer: kserde::Serializer, T: Serialize<KSer> + NumericFloat> kserde::Serialize<KSer>
    for Quaternion<T>
{
    fn serialize(&self, serializer: &mut KSer) {
        serializer.serialize(&self.0);
    }
}

impl<
        'kserde,
        KDes: kserde::Deserializer<'kserde>,
        T: kserde::Deserialize<'kserde, KDes> + NumericFloat,
    > kserde::Deserialize<'kserde, KDes> for Quaternion<T>
{
    fn deserialize(deserializer: &mut KDes) -> Option<Self> {
        Some(Self(<Vector<T, 4>>::deserialize(deserializer)?))
    }
}

impl<T: NumericFloat + std::fmt::Debug> Quaternion<T> {
    pub const IDENTITY: Self = Quaternion(Vector::<T, 4>::new(T::ZERO, T::ZERO, T::ZERO, T::ONE));

    pub fn from_xyzw(x: T, y: T, z: T, w: T) -> Self {
        Self((x, y, z, w).into())
    }

    pub fn from_angle_axis(angle: T, axis: Vector<T, 3>) -> Self {
        let axis = axis.normalized();
        let (s, c) = (angle * T::HALF).sin_cos_numeric();
        let v = axis * s;
        Self(Vector::<T, 4>::new(v[0], v[1], v[2], c))
    }

    pub fn to_angle_axis(self) -> (T, Vector<T, 3>) {
        let v = Vector::<T, 3>::new(self.0[0], self.0[1], self.0[2]);
        let length = v.length();
        let axis = v / length;
        let angle = T::TWO * T::atan2(length, self.0[3]);
        (angle, axis)
    }

    pub fn as_array(self) -> [T; 4] {
        self.0 .0[0]
    }

    pub fn from_yaw_pitch_roll(yaw: T, pitch: T, roll: T) -> Self {
        Self::from_angle_axis(yaw, <Vector<T, 3>>::Y)
            * Self::from_angle_axis(pitch, <Vector<T, 3>>::X)
            * Self::from_angle_axis(roll, <Vector<T, 3>>::Z)
    }

    pub fn rotate_vector3(&self, v: Vector<T, 3>) -> Vector<T, 3> {
        self.mul(v)
    }

    pub fn normalized(self) -> Self {
        Self(self.0.normalized())
    }

    /// Forward must be normalized
    pub fn from_forward_up(forward: Vector<T, 3>, up: Vector<T, 3>) -> Self {
        // This could be made more efficient.
        let looking_at_matrix =
            <Matrix<T, 4, 4>>::looking_at(<Vector<T, 3>>::ZERO, forward, up).inversed();
        looking_at_matrix.extract_rotation()
    }

    pub fn lerp(self, other: Self, amount: T) -> Self {
        Self(self.0 + (other.0 - self.0) * amount).normalized()
    }

    /// Spherically interpolate quaternions.
    /// Not commutative, constant velocity, minimal torque.
    /// See this article for details:
    /// http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/
    pub fn slerp(self, mut other: Self, amount: T) -> Self {
        let mut dot = self.0.dot(other.0);
        if dot < T::ZERO {
            other = other * -T::ONE;
            dot = -dot;
        };
        let dot_threshold = T::from_f32(0.9995);
        if dot > dot_threshold {
            // If these Quaternions are too similar linear interpolate instead.
            self.lerp(other, amount)
        } else {
            // Clamp dot to the range of acos.
            let dot = dot.numeric_clamp(-T::ONE, T::ONE);
            let theta_0 = dot.acos();
            let theta = theta_0 * amount;
            let v2 = (other.0 - self.0 * dot).normalized();
            let (sin, cos) = theta.sin_cos_numeric();
            Self(self.0 * cos + v2 * sin)
        }
    }
}

impl<T: NumericFloat> Mul for Quaternion<T> {
    type Output = Self;
    fn mul(self, b: Self) -> Self::Output {
        let a = self.0;
        let b = b.0;
        Self(Vector::<T, 4>::new(
            a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
            a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
            a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
            a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
        ))
    }
}

impl<T: NumericFloat> Mul<Vector<T, 3>> for Quaternion<T> {
    type Output = Vector<T, 3>;
    fn mul(self, other: Vector<T, 3>) -> Self::Output {
        let w = self.0[3];
        let b = Vector::<T, 3>::new(self.0[0], self.0[1], self.0[2]);
        let b2 = b.dot(b);
        other * (w * w - b2) + b * (other.dot(b) * T::TWO) + b.cross(other) * (w * T::TWO)
    }
}

impl<T: NumericFloat> Index<usize> for Quaternion<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0 .0[0][index]
    }
}

impl<T: NumericFloat> From<Vector<T, 4>> for Quaternion<T> {
    fn from(value: Vector<T, 4>) -> Quaternion<T> {
        Self(value)
    }
}

impl<T: NumericFloat> From<(T, T, T, T)> for Quaternion<T> {
    fn from(value: (T, T, T, T)) -> Quaternion<T> {
        Self([[value.0, value.1, value.2, value.3]].into())
    }
}

impl<T: NumericFloat> From<[T; 4]> for Quaternion<T> {
    fn from(value: [T; 4]) -> Quaternion<T> {
        Self(value.into())
    }
}

impl<T: NumericFloat> From<Quaternion<T>> for [T; 4] {
    fn from(value: Quaternion<T>) -> [T; 4] {
        value.0.into()
    }
}

impl<T: NumericFloat> Add<Quaternion<T>> for Quaternion<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0.add(other.0))
    }
}

impl<T: NumericFloat> Mul<T> for Quaternion<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self(self.0 * other)
    }
}

impl<T: NumericFloat> Neg for Quaternion<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self(self.0 * -T::ONE)
    }
}
