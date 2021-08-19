use crate::*;
use std::{
    ops::{Deref, DerefMut},
    usize,
};

pub type Vector<T, const N: usize> = Matrix<T, N, 1>;

impl<T, const N: usize> Vector<T, N> {
    pub const fn new_from_slice(values: [T; N]) -> Self {
        Self([values])
    }
}

impl<T> Vector<T, 2> {
    pub const fn new(x: T, y: T) -> Self {
        Self([[x, y]])
    }
}

impl<T> Vector<T, 3> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self([[x, y, z]])
    }
}

impl<T> Vector<T, 4> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self([[x, y, z, w]])
    }
}

impl<T> From<T> for Vector<T, 1> {
    fn from(value: T) -> Self {
        Self([[value]])
    }
}

impl<T> From<(T,)> for Vector<T, 1> {
    fn from(value: (T,)) -> Self {
        Self([[value.0]])
    }
}

impl<T> From<(T, T)> for Vector<T, 2> {
    fn from(value: (T, T)) -> Self {
        Self([[value.0, value.1]])
    }
}

impl<T: Copy> From<(T, T, T)> for Vector<T, 3> {
    fn from(value: (T, T, T)) -> Self {
        Self([[value.0, value.1, value.2]])
    }
}

impl<T: Copy> From<(T, T, T, T)> for Vector<T, 4> {
    fn from(value: (T, T, T, T)) -> Vector<T, 4> {
        Self([[value.0, value.1, value.2, value.3]])
    }
}

impl<T: Copy> From<Vector<T, 1>> for (T,) {
    fn from(value: Vector<T, 1>) -> (T,) {
        (value.0[0][0],)
    }
}

impl<T: Copy> From<Vector<T, 2>> for (T, T) {
    fn from(value: Vector<T, 2>) -> (T, T) {
        (value.0[0][0], value.0[0][1])
    }
}

impl<T: Copy> From<Vector<T, 3>> for (T, T, T) {
    fn from(value: Vector<T, 3>) -> (T, T, T) {
        (value.0[0][0], value.0[0][1], value.0[0][2])
    }
}

impl<T: Copy> From<Vector<T, 4>> for (T, T, T, T) {
    fn from(value: Vector<T, 4>) -> (T, T, T, T) {
        (value.0[0][0], value.0[0][1], value.0[0][2], value.0[0][3])
    }
}

impl<T: Numeric, const N: usize> Vector<T, N> {
    pub fn as_array(self) -> [T; N] {
        self.0[0]
    }

    /// Dot product of `self` and `other`
    #[inline]
    pub fn dot(self, other: Self) -> T {
        let mut total = T::ZERO;
        for i in 0..N {
            total = total + self.0[0][i] * other.0[0][i];
        }
        total
    }

    /// Returns a `Vector<T, 2>` with x and y components.
    /// If this `Vector` has fewer than 2 components then the extra
    /// components are set to 0.
    pub fn xy(&self) -> Vector<T, 2> {
        Vector::<T, 2>::new(self.0[0][0], if N > 1 { self.0[0][1] } else { T::ZERO })
    }

    /// Returns a `Vector<T, 3>` with x, y, and z components.
    /// If this `Vector` has fewer than 3 components then the extra
    /// components are set to 0.
    pub fn xyz(&self) -> Vector<T, 3> {
        Vector::<T, 3>::new(
            self.0[0][0],
            if N > 1 { self.0[0][1] } else { T::ZERO },
            if N > 2 { self.0[0][2] } else { T::ZERO },
        )
    }

    /// Returns a `Vector<T, 4>` with x, y, z, w components.
    /// If this `Vector` has fewer than 4 components then the extra
    /// components are set to 0.
    pub fn xyzw(&self) -> Vector<T, 4> {
        Vector::<T, 4>::new(
            self.0[0][0],
            if N > 1 { self.0[0][1] } else { T::ZERO },
            if N > 2 { self.0[0][2] } else { T::ZERO },
            if N > 3 { self.0[0][2] } else { T::ZERO },
        )
    }

    pub fn zxy(&self) -> Vector<T, 3> {
        Vector::<T, 3>::new(
            if N > 2 { self.0[0][2] } else { T::ZERO },
            self.0[0][0],
            if N > 1 { self.0[0][1] } else { T::ZERO },
        )
    }
}

pub trait Extend<T> {
    type ExtendTo;
    fn extend(self, value: T) -> Self::ExtendTo;
}

impl<T: Numeric> Extend<T> for Vector<T, 1> {
    type ExtendTo = Vector<T, 2>;
    fn extend(self, y: T) -> Self::ExtendTo {
        Vector::<T, 2>::new(self.0[0][0], y)
    }
}

impl<T: Numeric> Extend<T> for Vector<T, 2> {
    type ExtendTo = Vector<T, 3>;
    fn extend(self, z: T) -> Self::ExtendTo {
        Vector::<T, 3>::new(self.0[0][0], self.0[0][1], z)
    }
}

impl<T: Numeric> Extend<T> for Vector<T, 3> {
    type ExtendTo = Vector<T, 4>;
    fn extend(self, w: T) -> Self::ExtendTo {
        Vector::<T, 4>::new(self.0[0][0], self.0[0][1], self.0[0][2], w)
    }
}

impl<T: Numeric> Vector<T, 1> {
    pub fn x(self) -> T {
        self.0[0][0]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.0[0][0]
    }
}

impl<T: Numeric> Vector<T, 2> {
    pub fn x(self) -> T {
        self.0[0][0]
    }

    pub fn y(self) -> T {
        self.0[0][1]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.0[0][0]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.0[0][1]
    }
}

impl<T: Numeric> Vector<T, 3> {
    pub fn x(self) -> T {
        self.0[0][0]
    }

    pub fn y(self) -> T {
        self.0[0][1]
    }

    pub fn z(self) -> T {
        self.0[0][2]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.0[0][0]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.0[0][1]
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self.0[0][2]
    }
}

impl<T: Numeric> Vector<T, 4> {
    pub fn x(self) -> T {
        self.0[0][0]
    }

    pub fn y(self) -> T {
        self.0[0][1]
    }

    pub fn z(self) -> T {
        self.0[0][2]
    }

    pub fn w(self) -> T {
        self.0[0][3]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.0[0][0]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.0[0][1]
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self.0[0][2]
    }

    pub fn w_mut(&mut self) -> &mut T {
        &mut self.0[0][3]
    }
}

impl<T: Numeric + NumericSqrt, const N: usize> Vector<T, N> {
    /// Calculates the length of this `Vector`
    pub fn length(self) -> T {
        self.dot(self).numeric_sqrt()
    }

    pub fn length_squared(self) -> T {
        self.dot(self)
    }

    /// Returns a new `Vector` with a length of 1.0
    pub fn normalized(self) -> Self {
        self / self.dot(self).numeric_sqrt()
    }
}

impl<T: Numeric> Vector<T, 1> {
    pub const X: Self = {
        let mut v = Self::ZERO;
        v.0[0][0] = T::ONE;
        v
    };
}

impl<T: Numeric> Vector<T, 2> {
    pub const X: Self = {
        let mut v = Self::ZERO;
        v.0[0][0] = T::ONE;
        v
    };
    pub const Y: Self = {
        let mut v = Self::ZERO;
        v.0[0][1] = T::ONE;
        v
    };
}

impl<T: Numeric> Vector<T, 3> {
    pub const X: Self = {
        let mut v = Self::ZERO;
        v.0[0][0] = T::ONE;
        v
    };
    pub const Y: Self = {
        let mut v = Self::ZERO;
        v.0[0][1] = T::ONE;
        v
    };
    pub const Z: Self = {
        let mut v = Self::ZERO;
        v.0[0][2] = T::ONE;
        v
    };
    pub const XY: Self = {
        let mut v = Self::ZERO;
        v.0[0][0] = T::ONE;
        v.0[0][1] = T::ONE;
        v
    };
    pub const XZ: Self = {
        let mut v = Self::ZERO;
        v.0[0][0] = T::ONE;
        v.0[0][2] = T::ONE;
        v
    };
    pub const YZ: Self = {
        let mut v = Self::ZERO;
        v.0[0][1] = T::ONE;
        v.0[0][2] = T::ONE;
        v
    };
}

impl<T: Numeric> Vector<T, 4> {
    pub const X: Self = {
        let mut v = Self::ZERO;
        v.0[0][0] = T::ONE;
        v
    };
    pub const Y: Self = {
        let mut v = Self::ZERO;
        v.0[0][1] = T::ONE;
        v
    };
    pub const Z: Self = {
        let mut v = Self::ZERO;
        v.0[0][2] = T::ONE;
        v
    };
    pub const W: Self = {
        let mut v = Self::ZERO;
        v.0[0][3] = T::ONE;
        v
    };
}

impl<T: Numeric> Vector<T, 3> {
    /// Produces a `Vector<T, 3>` perpendicular to `self` and `other`.
    /// Only applicable to 3-dimensional `Vector`s.
    pub fn cross(self, other: Self) -> Self {
        (self.zxy().mul_by_component(other) - self.mul_by_component(other.zxy())).zxy()
    }
}

#[repr(C)]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
pub struct XYZ<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
pub struct XYZW<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Deref for Vector<T, 2> {
    type Target = XY<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self.0.as_ptr()) }
    }
}

impl<T> DerefMut for Vector<T, 2> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self.0.as_mut_ptr()) }
    }
}

impl<T> Deref for Vector<T, 3> {
    type Target = XYZ<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self.0.as_ptr()) }
    }
}

impl<T> DerefMut for Vector<T, 3> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self.0.as_mut_ptr()) }
    }
}

impl<T> Deref for Vector<T, 4> {
    type Target = XYZW<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self.0.as_ptr()) }
    }
}

impl<T> DerefMut for Vector<T, 4> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self.0.as_mut_ptr()) }
    }
}
