use std::{
    array::TryFromSliceError,
    convert::TryFrom,
    convert::TryInto,
    default::Default,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
    usize,
};

use kserde::Serialize;

use crate::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct Matrix<T, const ROWS: usize, const COLUMNS: usize>(pub(crate) [[T; ROWS]; COLUMNS]);

// Manually tweaked serialization / deserialization implementations.
impl<KSer: kserde::Serializer, T: Serialize<KSer>, const ROWS: usize, const COLUMNS: usize>
    kserde::Serialize<KSer> for Matrix<T, ROWS, COLUMNS>
{
    fn serialize(&self, serializer: &mut KSer) {
        serializer.serialize(&self.0);
    }
}

impl<
        'kserde,
        KDes: kserde::Deserializer<'kserde>,
        T: kserde::Deserialize<'kserde, KDes>,
        const ROWS: usize,
        const COLUMNS: usize,
    > kserde::Deserialize<'kserde, KDes> for Matrix<T, ROWS, COLUMNS>
{
    fn deserialize(deserializer: &mut KDes) -> Option<Self> {
        Some(Self(<[[T; ROWS]; COLUMNS]>::deserialize(deserializer)?))
    }
}

impl<T: Numeric, const R: usize, const C: usize> Default for Matrix<T, R, C> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const R: usize, const C: usize> Default for Matrix<bool, R, C> {
    fn default() -> Self {
        Self::FALSE
    }
}

impl<T: Numeric, const R: usize, const C: usize> Add for Matrix<T, R, C> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut output = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = self.0[i][j] + other.0[i][j]
            }
        }
        output
    }
}

impl<T: Numeric, const R: usize, const C: usize> Sub for Matrix<T, R, C> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut output = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = self.0[i][j] - other.0[i][j]
            }
        }
        output
    }
}

impl<T: Numeric + NumericFloat, const R0: usize, const C0_R1: usize, const C1: usize>
    Mul<Matrix<T, C0_R1, C1>> for Matrix<T, R0, C0_R1>
{
    type Output = Matrix<T, R0, C1>;
    #[inline]
    fn mul(self, other: Matrix<T, C0_R1, C1>) -> Matrix<T, R0, C1> {
        // Godbolt comparing these two:
        // https://godbolt.org/z/9hvexjjdv
        // It'd be great to figure out a way to only use the general case.
        match (C0_R1, C1) {
            (4, 4) => unsafe {
                let s: Matrix<T, 4, 4> = std::mem::transmute_copy(&self);
                let other: Matrix<T, 4, 4> = std::mem::transmute_copy(&other);
                std::mem::transmute_copy(&s.faster_mul(&other))
            },
            _ => {
                let mut output = Matrix::<T, R0, C1>::ZERO;
                for j in 0..C1 {
                    for i in 0..R0 {
                        output.0[j][i] = self.row(i).dot(Matrix([other.0[j]]));
                    }
                }
                output
            }
        }
    }
}

impl<T: Numeric, const R: usize, const C: usize> Mul<T> for Matrix<T, R, C> {
    type Output = Self;

    #[inline]
    fn mul(self, other: T) -> Self {
        let mut output = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = self.0[i][j] * other
            }
        }
        output
    }
}

/*
impl<T: Numeric, const R: usize, const C: usize> Div for Matrix<T, R, C> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut output = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = self.0[i][j] / other.0[i][j]
            }
        }
        output
    }
}
*/

impl<T: Numeric, const R: usize, const C: usize> Div<T> for Matrix<T, R, C> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        let mut output = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = self.0[i][j] / other
            }
        }
        output
    }
}

impl<T: Numeric, const R: usize, const C: usize> AddAssign for Matrix<T, R, C> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: Numeric, const R: usize, const C: usize> SubAssign for Matrix<T, R, C> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

/*
impl<T: Numeric, const R: usize, const C: usize> DivAssign for Matrix<T, R, C> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}
*/

impl<T: Numeric, const R: usize, const C: usize> MulAssign<T> for Matrix<T, R, C> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

// MulAssign is only implemented for multiplying square matrices by
// a matrix of the same size.
impl<T: Numeric + NumericFloat, const N: usize> MulAssign<Matrix<T, N, N>> for Matrix<T, N, N> {
    #[inline]
    fn mul_assign(&mut self, other: Matrix<T, N, N>) {
        *self = *self * other
    }
}

impl<T: Numeric, const R: usize, const C: usize> DivAssign<T> for Matrix<T, R, C> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T: Numeric + Neg<Output = T>, const R: usize, const C: usize> Neg for Matrix<T, R, C> {
    type Output = Self;
    fn neg(self) -> Self {
        let mut output = self;
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = output.0[i][j].neg();
            }
        }
        output
    }
}

impl<T: Numeric, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const ZERO: Self = Self([[T::ZERO; R]; C]);
    pub const ONE: Self = Self([[T::ONE; R]; C]);
    pub const MAX: Self = Self([[T::MAX; R]; C]);
    pub const MIN: Self = Self([[T::MIN; R]; C]);

    /// Creates a new `Matrix` with `value` in each component.
    /// Also referred to as `broadcast` or `splat` in other libraries.
    pub fn fill(value: T) -> Self {
        Self([[value; R]; C])
    }

    pub fn min(self, other: Self) -> Self {
        let mut output = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = self.0[i][j].numeric_min(other.0[i][j])
            }
        }
        output
    }

    pub fn max(self, other: Self) -> Self {
        let mut output = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = self.0[i][j].numeric_max(other.0[i][j])
            }
        }
        output
    }

    /// Returns the value of the minimum component.
    pub fn min_component(self) -> T {
        let mut min = T::MAX;
        for i in 0..C {
            for j in 0..R {
                min = self.0[i][j].numeric_min(min)
            }
        }
        min
    }

    /// Returns the value of the maximum component.
    pub fn max_component(self) -> T {
        let mut max = T::MIN;
        for i in 0..C {
            for j in 0..R {
                max = self.0[i][j].numeric_max(max)
            }
        }
        max
    }

    /// Multiplies each component by the corresponding component from `other`.
    pub fn mul_by_component(self, other: Self) -> Self {
        let mut output = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = self.0[i][j] * other.0[i][j]
            }
        }
        output
    }

    /// Divides each component by the corresponding component from `other`.
    pub fn div_by_component(self, other: Self) -> Self {
        let mut output = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = self.0[i][j] / other.0[i][j]
            }
        }
        output
    }
}

impl<T: Numeric + NumericAbs, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Takes the absolute value of each component
    pub fn abs(self) -> Self {
        let mut v = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                v.0[i][j] = self.0[i][j].numeric_abs()
            }
        }
        v
    }
}

impl<T: Numeric + NumericSigNum, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Takes the absolute value of each component
    pub fn signum(self) -> Self {
        let mut v = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                v.0[i][j] = self.0[i][j].signum_numeric()
            }
        }
        v
    }
}

impl<T: Numeric, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn reciprocal(&self) -> Self
    where
        T: NumericFloat,
    {
        let mut v = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                v.0[i][j] = T::ONE / self.0[i][j];
            }
        }
        v
    }

    pub fn is_nan(&self) -> bool
    where
        T: NumericFloat,
    {
        for v in &self.0 {
            for j in v {
                if j.is_nan_numeric() {
                    return true;
                }
            }
        }
        return false;
    }

    // This should probably just return an array
    #[inline]
    pub fn row(&self, index: usize) -> Vector<T, C> {
        let mut v: Vector<T, C> = Vector::ZERO;
        for i in 0..C {
            v[i] = self.0[i][index]
        }
        v
    }

    // This should probably just return an array
    #[inline]
    pub fn column(&self, index: usize) -> Vector<T, R> {
        Vector::new_from_slice(self.0[index])
    }

    pub fn set_column(&mut self, index: usize, value: Vector<T, R>) {
        self.0[index] = value.0[0]
    }

    /// Get a direct mutable reference to the columns of the matrix.
    pub fn column_mut(&mut self, index: usize) -> &mut [T; R] {
        &mut self.0[index]
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        // This is safe because we are statically bounding our slices to the size of these
        // vectors
        unsafe { std::slice::from_raw_parts(std::mem::transmute(&self.0), R * C) }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        // This is safe because we are statically bounding our slices to the size of these
        // vectors
        unsafe { std::slice::from_raw_parts_mut(std::mem::transmute(&mut self.0), R * C) }
    }

    pub fn powf(&self, other: Self) -> Self
    where
        T: NumericFloat,
    {
        let mut v = Self::ZERO;
        for i in 0..C {
            for j in 0..R {
                v.0[i][j] = self.0[i][j].powf_numeric(other.0[i][j]);
            }
        }
        v
    }
}

impl<T: NumericFloat> Matrix<T, 4, 4> {
    pub fn as_array(&self) -> &[T; 16] {
        self.as_slice().try_into().unwrap()
    }

    pub fn from_translation(translation: Vector<T, 3>) -> Self {
        Self([
            [T::ONE, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ONE, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE, T::ZERO],
            translation.extend(T::ONE).0[0],
        ])
    }

    pub fn from_quaternion(quaternion: Quaternion<T>) -> Self
    where
        T: NumericFloat,
    {
        quaternion.into()
    }

    pub fn from_translation_rotation_scale(
        translation: Vector<T, 3>,
        rotation: Quaternion<T>,
        scale: Vector<T, 3>,
    ) -> Self
    where
        T: NumericFloat,
    {
        let mut m: Self = rotation.into();
        m.set_column(0, m.column(0) * scale[0]);
        m.set_column(1, m.column(1) * scale[1]);
        m.set_column(2, m.column(2) * scale[2]);
        m.set_column(3, translation.extend(T::ONE));
        m
    }

    pub fn to_translation_rotation_scale(&self) -> (Vector<T, 3>, Quaternion<T>, Vector<T, 3>) {
        (
            self.extract_translation(),
            self.extract_rotation(),
            self.extract_scale(),
        )
    }

    pub fn extract_translation(&self) -> Vector<T, 3> {
        self.column(3).xyz()
    }

    pub fn extract_scale(&self) -> Vector<T, 3>
    where
        T: NumericFloat,
    {
        [
            self.column(0).length(),
            self.column(1).length(),
            self.column(2).length(),
        ]
        .into()
    }

    fn extract_rotation_scale(&self) -> (Quaternion<T>, Vector<T, 3>) {
        let scale: Vector<T, 3> = [
            self.column(0).length(),
            self.column(1).length(),
            self.column(2).length(),
        ]
        .into();

        let rotation = Matrix([
            [
                self.0[0][0] / scale[0],
                self.0[0][1] / scale[0],
                self.0[0][2] / scale[0],
                T::ZERO,
            ],
            [
                self.0[1][0] / scale[1],
                self.0[1][1] / scale[1],
                self.0[1][2] / scale[1],
                T::ZERO,
            ],
            [
                self.0[2][0] / scale[2],
                self.0[2][1] / scale[2],
                self.0[2][2] / scale[2],
                T::ZERO,
            ],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ]);

        let m00 = rotation[(0, 0)];
        let m11 = rotation[(1, 1)];
        let m22 = rotation[(2, 2)];
        let m21 = rotation[(2, 1)];
        let m12 = rotation[(1, 2)];
        let m02 = rotation[(0, 2)];
        let m20 = rotation[(2, 0)];
        let m10 = rotation[(1, 0)];
        let m01 = rotation[(0, 1)];

        let w = T::ZERO.numeric_max(T::ONE + m00 + m11 + m22).numeric_sqrt() * T::HALF;
        let x = T::ZERO.numeric_max(T::ONE + m00 - m11 - m22).numeric_sqrt() * T::HALF;
        let y = T::ZERO.numeric_max(T::ONE - m00 + m11 - m22).numeric_sqrt() * T::HALF;
        let z = T::ZERO.numeric_max(T::ONE - m00 - m11 + m22).numeric_sqrt() * T::HALF;

        let x = x.copysign_numeric(m21 - m12);
        let y = y.copysign_numeric(m02 - m20);
        let z = z.copysign_numeric(m10 - m01);
        (Quaternion((x, y, z, w).into()), scale)
    }

    pub fn extract_rotation(&self) -> Quaternion<T>
    where
        T: NumericFloat,
    {
        self.extract_rotation_scale().0
    }

    pub fn transform_point(&self, point: Vector<T, 3>) -> Vector<T, 3> {
        let point = point.extend(T::ONE);
        Vector::<T, 3>::new(
            self.row(0).dot(point),
            self.row(1).dot(point),
            self.row(2).dot(point),
        )
    }

    pub fn transform_vector(&self, vector: Vector<T, 3>) -> Vector<T, 3> {
        let vector = vector.extend(T::ZERO);
        Vector::<T, 3>::new(
            self.row(0).dot(vector),
            self.row(1).dot(vector),
            self.row(2).dot(vector),
        )
    }

    pub fn look_at(from: Vector<T, 3>, target: Vector<T, 3>, up: Vector<T, 3>) -> Self
    where
        T: NumericSqrt + Neg<Output = T>,
    {
        let f = (target - from).normalized();
        let r = f.cross(up).normalized();
        let u = r.cross(f);
        Self([
            [r[0], u[0], -f[0], T::ZERO],
            [r[1], u[1], -f[1], T::ZERO],
            [r[2], u[2], -f[2], T::ZERO],
            [-r.dot(from), -u.dot(from), f.dot(from), T::ONE],
        ])
    }

    /*
    // This should be implemented for other square matrices as well.
    // A match could branch on N internally to provide varying implementations.
    pub fn determinant(&self) -> T {
        let [m00, m01, m02, m03]: [T; 4] = self.column(0).into();
        let [m10, m11, m12, m13]: [T; 4] = self.column(1).into();
        let [m20, m21, m22, m23]: [T; 4] = self.column(3).into();
        let [m30, m31, m32, m33]: [T; 4] = self.column(3).into();

        let a2323 = (m22 * m33) - (m23 * m32);
        let a1323 = (m21 * m33) - (m23 * m31);
        let a1223 = (m21 * m32) - (m22 * m31);
        let a0323 = (m20 * m33) - (m23 * m30);
        let a0223 = (m20 * m32) - (m22 * m30);
        let a0123 = (m20 * m31) - (m21 * m30);

        m00 * (m11 * a2323 - m12 * a1323 + m13 * a1223)
            - m01 * (m10 * a2323 - m12 * a0323 + m13 * a0223)
            + m02 * (m10 * a1323 - m11 * a0323 + m13 * a0123)
            - m03 * (m10 * a1223 - m11 * a0223 + m12 * a0123)
    }
    */

    // This should be implemented for other matrices as well.
    pub fn inversed(&self) -> Self {
        let (m00, m01, m02, m03) = self.column(0).into();
        let (m10, m11, m12, m13) = self.column(1).into();
        let (m20, m21, m22, m23) = self.column(2).into();
        let (m30, m31, m32, m33) = self.column(3).into();

        let coef00 = m22 * m33 - m32 * m23;
        let coef02 = m12 * m33 - m32 * m13;
        let coef03 = m12 * m23 - m22 * m13;

        let coef04 = m21 * m33 - m31 * m23;
        let coef06 = m11 * m33 - m31 * m13;
        let coef07 = m11 * m23 - m21 * m13;

        let coef08 = m21 * m32 - m31 * m22;
        let coef10 = m11 * m32 - m31 * m12;
        let coef11 = m11 * m22 - m21 * m12;

        let coef12 = m20 * m33 - m30 * m23;
        let coef14 = m10 * m33 - m30 * m13;
        let coef15 = m10 * m23 - m20 * m13;

        let coef16 = m20 * m32 - m30 * m22;
        let coef18 = m10 * m32 - m30 * m12;
        let coef19 = m10 * m22 - m20 * m12;

        let coef20 = m20 * m31 - m30 * m21;
        let coef22 = m10 * m31 - m30 * m11;
        let coef23 = m10 * m21 - m20 * m11;

        let fac0 = Vector::<T, 4>::new(coef00, coef00, coef02, coef03);
        let fac1 = Vector::<T, 4>::new(coef04, coef04, coef06, coef07);
        let fac2 = Vector::<T, 4>::new(coef08, coef08, coef10, coef11);
        let fac3 = Vector::<T, 4>::new(coef12, coef12, coef14, coef15);
        let fac4 = Vector::<T, 4>::new(coef16, coef16, coef18, coef19);
        let fac5 = Vector::<T, 4>::new(coef20, coef20, coef22, coef23);

        let vec1 = Vector::<T, 4>::new(m11, m01, m01, m01);
        let vec0 = Vector::<T, 4>::new(m10, m00, m00, m00);
        let vec2 = Vector::<T, 4>::new(m12, m02, m02, m02);
        let vec3 = Vector::<T, 4>::new(m13, m03, m03, m03);

        let inv0 =
            vec1.mul_by_component(fac0) - vec2.mul_by_component(fac1) + vec3.mul_by_component(fac2);
        let inv1 =
            vec0.mul_by_component(fac0) - vec2.mul_by_component(fac3) + vec3.mul_by_component(fac4);
        let inv2 =
            vec0.mul_by_component(fac1) - vec1.mul_by_component(fac3) + vec3.mul_by_component(fac5);
        let inv3 =
            vec0.mul_by_component(fac2) - vec1.mul_by_component(fac4) + vec2.mul_by_component(fac5);

        let sign_a = Vector::<T, 4>::new(T::ONE, -T::ONE, T::ONE, -T::ONE);
        let sign_b = Vector::<T, 4>::new(-T::ONE, T::ONE, -T::ONE, T::ONE);

        let inverse = Self([
            inv0.mul_by_component(sign_a).into(),
            inv1.mul_by_component(sign_b).into(),
            inv2.mul_by_component(sign_a).into(),
            inv3.mul_by_component(sign_b).into(),
        ]);

        let col0 = inverse.row(0);
        let dot = self.column(0).dot(col0);

        let rcp_det = T::ONE / dot;
        inverse * rcp_det
    }
}

// Square matrix implementations
impl<T: Numeric, const N: usize> Matrix<T, N, N> {
    /// An identity matrix.
    /// This is only defined for matrices where the number of columns is equal to the number of rows.
    pub const IDENTITY: Self = {
        let mut v = Self::ZERO;
        let mut i = 0;
        while i < N {
            v.0[i][i] = T::ONE;
            i += 1;
        }
        v
    };

    pub fn from_scale(scale: T) -> Self {
        let mut v = Self::ZERO;
        for i in 0..N - 1 {
            v.0[i][i] = scale;
        }
        v.0[N - 1][N - 1] = T::ONE;
        v
    }
}

impl<T: Numeric, const R: usize, const C: usize> Index<usize> for Matrix<T, R, C> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let row = index / R;
        let column = index % R;
        &self.0[row][column]
    }
}

impl<T: Numeric, const R: usize, const C: usize> IndexMut<usize> for Matrix<T, R, C> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let row = index / R;
        let column = index % R;
        &mut self.0[row][column]
    }
}

impl<T: Numeric, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
    type Output = T;
    /// Indexes by row then column.
    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}

impl<T: Numeric, const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<T, R, C> {
    /// Indexes by row then column.
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.1][index.0]
    }
}

impl<T, const R: usize, const C: usize> From<[[T; R]; C]> for Matrix<T, R, C> {
    fn from(value: [[T; R]; C]) -> Self {
        Self(value)
    }
}

impl<T, const R: usize> From<[T; R]> for Matrix<T, R, 1> {
    fn from(value: [T; R]) -> Self {
        Self([value])
    }
}

impl<T, const R: usize, const C: usize> From<Matrix<T, R, C>> for [[T; R]; C] {
    fn from(value: Matrix<T, R, C>) -> Self {
        value.0
    }
}

impl<T: Copy, const R: usize> From<Matrix<T, R, 1>> for [T; R] {
    fn from(value: Matrix<T, R, 1>) -> Self {
        value.0[0]
    }
}

impl<T: NumericFloat> From<Quaternion<T>> for Matrix<T, 4, 4> {
    fn from(q: Quaternion<T>) -> Self {
        let x2 = q[0] + q[0];
        let y2 = q[1] + q[1];
        let z2 = q[2] + q[2];
        let xx = q[0] * x2;
        let xy = q[0] * y2;
        let xz = q[0] * z2;
        let yy = q[1] * y2;
        let yz = q[1] * z2;
        let zz = q[2] * z2;
        let wx = q[3] * x2;
        let wy = q[3] * y2;
        let wz = q[3] * z2;

        Self([
            [T::ONE - (yy + zz), xy + wz, xz - wy, T::ZERO],
            [xy - wz, T::ONE - (xx + zz), yz + wx, T::ZERO],
            [xz + wy, yz - wx, T::ONE - (xx + yy), T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }
}

impl<T: Numeric, const R: usize, const C: usize, const V: usize> TryFrom<&[T; V]>
    for Matrix<T, R, C>
{
    type Error = TryFromSliceError;
    fn try_from(value: &[T; V]) -> Result<Self, Self::Error> {
        let mut v = Self::ZERO;
        let mut offset = 0;
        for c in 0..C {
            let end = offset + R;
            v.0[c] = value[offset..end].try_into()?;
            offset = end;
        }
        Ok(v)
    }
}

impl<T: Numeric, const R: usize, const C: usize> TryFrom<&[T]>
    for Matrix<T, R, C>
{
    type Error = TryFromSliceError;
    fn try_from(value: &[T]) -> Result<Self, Self::Error> {
        let mut v = Self::ZERO;
        let mut offset = 0;
        for c in 0..C {
            let end = offset + R;
            v.0[c] = value[offset..end].try_into()?;
            offset = end;
        }
        Ok(v)
    }
}

impl<T: NumericFloat> Matrix<T, 4, 4> {
    #[inline]
    fn faster_mul(&self, other: &Self) -> Self {
        let sa = self.0[0];
        let sb = self.0[1];
        let sc = self.0[2];
        let sd = self.0[3];
        let oa = other.0[0];
        let ob = other.0[1];
        let oc = other.0[2];
        let od = other.0[3];
        Self([
            [
                (sa[0] * oa[0]) + (sb[0] * oa[1]) + (sc[0] * oa[2]) + (sd[0] * oa[3]),
                (sa[1] * oa[0]) + (sb[1] * oa[1]) + (sc[1] * oa[2]) + (sd[1] * oa[3]),
                (sa[2] * oa[0]) + (sb[2] * oa[1]) + (sc[2] * oa[2]) + (sd[2] * oa[3]),
                (sa[3] * oa[0]) + (sb[3] * oa[1]) + (sc[3] * oa[2]) + (sd[3] * oa[3]),
            ],
            [
                (sa[0] * ob[0]) + (sb[0] * ob[1]) + (sc[0] * ob[2]) + (sd[0] * ob[3]),
                (sa[1] * ob[0]) + (sb[1] * ob[1]) + (sc[1] * ob[2]) + (sd[1] * ob[3]),
                (sa[2] * ob[0]) + (sb[2] * ob[1]) + (sc[2] * ob[2]) + (sd[2] * ob[3]),
                (sa[3] * ob[0]) + (sb[3] * ob[1]) + (sc[3] * ob[2]) + (sd[3] * ob[3]),
            ],
            [
                (sa[0] * oc[0]) + (sb[0] * oc[1]) + (sc[0] * oc[2]) + (sd[0] * oc[3]),
                (sa[1] * oc[0]) + (sb[1] * oc[1]) + (sc[1] * oc[2]) + (sd[1] * oc[3]),
                (sa[2] * oc[0]) + (sb[2] * oc[1]) + (sc[2] * oc[2]) + (sd[2] * oc[3]),
                (sa[3] * oc[0]) + (sb[3] * oc[1]) + (sc[3] * oc[2]) + (sd[3] * oc[3]),
            ],
            [
                (sa[0] * od[0]) + (sb[0] * od[1]) + (sc[0] * od[2]) + (sd[0] * od[3]),
                (sa[1] * od[0]) + (sb[1] * od[1]) + (sc[1] * od[2]) + (sd[1] * od[3]),
                (sa[2] * od[0]) + (sb[2] * od[1]) + (sc[2] * od[2]) + (sd[2] * od[3]),
                (sa[3] * od[0]) + (sb[3] * od[1]) + (sc[3] * od[2]) + (sd[3] * od[3]),
            ],
        ])
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Matrix<bool, ROWS, COLUMNS> {
    pub const TRUE: Self = Self([[true; ROWS]; COLUMNS]);
    pub const FALSE: Self = Self([[false; ROWS]; COLUMNS]);

    pub fn all(self) -> bool {
        for c in &self.0 {
            for r in c {
                if !r {
                    return false;
                }
            }
        }
        true
    }

    pub fn any(self) -> bool {
        for c in &self.0 {
            for r in c {
                if *r {
                    return true;
                }
            }
        }
        false
    }
}

impl<T: PartialOrd, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS> {
    pub fn less_than_per_component(self, other: Self) -> Matrix<bool, ROWS, COLUMNS> {
        let mut m = Matrix::<bool, ROWS, COLUMNS>::default();
        for c in 0..COLUMNS {
            for r in 0..ROWS {
                m.0[c][r] = self.0[c][r] < other.0[c][r]
            }
        }
        m
    }

    pub fn less_than_or_equal_per_component(self, other: Self) -> Matrix<bool, ROWS, COLUMNS> {
        let mut m = Matrix::<bool, ROWS, COLUMNS>::default();
        for c in 0..COLUMNS {
            for r in 0..ROWS {
                m.0[c][r] = self.0[c][r] <= other.0[c][r]
            }
        }
        m
    }

    pub fn greater_than_per_component(self, other: Self) -> Matrix<bool, ROWS, COLUMNS> {
        let mut m = Matrix::<bool, ROWS, COLUMNS>::default();
        for c in 0..COLUMNS {
            for r in 0..ROWS {
                m.0[c][r] = self.0[c][r] > other.0[c][r]
            }
        }
        m
    }

    pub fn greater_than_or_equal_per_component(self, other: Self) -> Matrix<bool, ROWS, COLUMNS> {
        let mut m = Matrix::<bool, ROWS, COLUMNS>::default();
        for c in 0..COLUMNS {
            for r in 0..ROWS {
                m.0[c][r] = self.0[c][r] >= other.0[c][r]
            }
        }
        m
    }
}

impl<T: Eq, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS> {
    pub fn equal_per_component(self, other: Self) -> Matrix<bool, ROWS, COLUMNS> {
        let mut m = Matrix::<bool, ROWS, COLUMNS>::default();
        for c in 0..COLUMNS {
            for r in 0..ROWS {
                m.0[c][r] = self.0[c][r] == other.0[c][r]
            }
        }
        m
    }
}

// ------- More Mul / Div implementations --------
// I don't like these implementations but couldn't figure out a better way quickly.
// It'd be great to remove them and implement this generically.

impl<const R: usize, const C: usize> Mul<Matrix<f32, R, C>> for f32 {
    type Output = Matrix<f32, R, C>;

    #[inline]
    fn mul(self, other: Matrix<f32, R, C>) -> Self::Output {
        other * self
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<f64, R, C>> for f64 {
    type Output = Matrix<f64, R, C>;

    #[inline]
    fn mul(self, other: Matrix<f64, R, C>) -> Self::Output {
        other * self
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<i8, R, C>> for i8 {
    type Output = Matrix<i8, R, C>;

    #[inline]
    fn mul(self, other: Matrix<i8, R, C>) -> Self::Output {
        other * self
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<i16, R, C>> for i16 {
    type Output = Matrix<i16, R, C>;

    #[inline]
    fn mul(self, other: Matrix<i16, R, C>) -> Self::Output {
        other * self
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<i32, R, C>> for i32 {
    type Output = Matrix<i32, R, C>;

    #[inline]
    fn mul(self, other: Matrix<i32, R, C>) -> Self::Output {
        other * self
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<i64, R, C>> for i64 {
    type Output = Matrix<i64, R, C>;

    #[inline]
    fn mul(self, other: Matrix<i64, R, C>) -> Self::Output {
        other * self
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<i128, R, C>> for i128 {
    type Output = Matrix<i128, R, C>;

    #[inline]
    fn mul(self, other: Matrix<i128, R, C>) -> Self::Output {
        other * self
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<u8, R, C>> for u8 {
    type Output = Matrix<u8, R, C>;

    #[inline]
    fn mul(self, other: Matrix<u8, R, C>) -> Self::Output {
        other * self
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<u16, R, C>> for u16 {
    type Output = Matrix<u16, R, C>;

    #[inline]
    fn mul(self, other: Matrix<u16, R, C>) -> Self::Output {
        other * self
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<u32, R, C>> for u32 {
    type Output = Matrix<u32, R, C>;

    #[inline]
    fn mul(self, other: Matrix<u32, R, C>) -> Self::Output {
        other * self
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<u64, R, C>> for u64 {
    type Output = Matrix<u64, R, C>;

    #[inline]
    fn mul(self, other: Matrix<u64, R, C>) -> Self::Output {
        other * self
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<u128, R, C>> for u128 {
    type Output = Matrix<u128, R, C>;

    #[inline]
    fn mul(self, other: Matrix<u128, R, C>) -> Self::Output {
        other * self
    }
}

impl<const R: usize, const C: usize> Div<Matrix<f32, R, C>> for f32 {
    type Output = Matrix<f32, R, C>;

    #[inline]
    fn div(self, other: Matrix<f32, R, C>) -> Self::Output {
        other.reciprocal() * self
    }
}

impl<const R: usize, const C: usize> Div<Matrix<f64, R, C>> for f64 {
    type Output = Matrix<f64, R, C>;

    #[inline]
    fn div(self, other: Matrix<f64, R, C>) -> Self::Output {
        other.reciprocal() * self
    }
}

// These `as` casts be implemented generically
impl<const R: usize, const C: usize> Matrix<f32, R, C> {
    pub fn as_i32(&self) -> Matrix<i32, R, C> {
        let mut output = Matrix::<i32, R, C>::ZERO;
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = self.0[i][j] as i32;
            }
        }
        output
    }
}

impl<const R: usize, const C: usize> Matrix<i32, R, C> {
    pub fn as_f32(&self) -> Matrix<f32, R, C> {
        let mut output = Matrix::<f32, R, C>::ZERO;
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = self.0[i][j] as f32;
            }
        }
        output
    }
}

// TODO: Make this work for non-numeric types
impl<T: Numeric, const R: usize, const C: usize> Matrix<T, R, C> {
    /// For each element select either from `if_true` or `if_false`
    /// based on corresponding boolean in `mask`.
    pub fn select(mask: Matrix<bool, R, C>, if_true: Self, if_false: Self) -> Self {
        let mut output = Self::default();
        for i in 0..C {
            for j in 0..R {
                output.0[i][j] = if mask.0[i][j] {
                    if_true.0[i][j]
                } else {
                    if_false.0[i][j]
                };
            }
        }
        output
    }
}

impl<'a, T: Numeric, const R: usize, const C: usize> IntoIterator for &'a Matrix<T, R, C> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<'a, T: Numeric, const R: usize, const C: usize> IntoIterator for &'a mut Matrix<T, R, C> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice_mut().iter_mut()
    }
}
