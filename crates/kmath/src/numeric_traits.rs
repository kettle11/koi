use std::{
    default::Default,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub trait Numeric:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Copy
    + Default
    + Clone
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;

    const MAX: Self;
    const MIN: Self;

    fn numeric_min(self, other: Self) -> Self;
    fn numeric_max(self, other: Self) -> Self;
    fn numeric_clamp(self, min: Self, max: Self) -> Self;
}

pub trait NumericFloat:
    Numeric + NumericAbs + Neg<Output = Self> + std::cmp::PartialOrd + NumericSigNum + 'static
{
    const HALF: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const PI: Self;

    fn sin_cos_numeric(self) -> (Self, Self);
    fn tan_numeric(self) -> Self;
    fn is_nan_numeric(self) -> bool;
    fn copysign_numeric(self, sign: Self) -> Self;
    fn min_numeric(self, other: Self) -> Self;
    fn max_numeric(self, other: Self) -> Self;
    fn powf_numeric(self, other: Self) -> Self;
    fn floor_numeric(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn acos(self) -> Self;
    fn round_numeric(self) -> Self;
    fn from_f32(f: f32) -> Self;
    fn numeric_sqrt(self) -> Self;
    fn is_finite(self) -> bool;
}

impl NumericFloat for f32 {
    const HALF: Self = 0.5;
    const INFINITY: Self = Self::INFINITY;
    const NEG_INFINITY: Self = Self::NEG_INFINITY;
    const PI: Self = std::f32::consts::PI;

    fn sin_cos_numeric(self) -> (Self, Self) {
        self.sin_cos()
    }
    fn tan_numeric(self) -> Self {
        self.tan()
    }
    fn is_nan_numeric(self) -> bool {
        self.is_nan()
    }
    fn copysign_numeric(self, sign: Self) -> Self {
        self.copysign(sign)
    }
    fn min_numeric(self, other: Self) -> Self {
        self.min(other)
    }

    fn max_numeric(self, other: Self) -> Self {
        self.max(other)
    }
    fn powf_numeric(self, other: Self) -> Self {
        self.powf(other)
    }
    fn floor_numeric(self) -> Self {
        self.floor()
    }
    fn atan2(self, other: Self) -> Self {
        Self::atan2(self, other)
    }
    fn acos(self) -> Self {
        Self::acos(self)
    }
    fn round_numeric(self) -> Self {
        self.round()
    }
    fn from_f32(f: f32) -> Self {
        f
    }
    fn numeric_sqrt(self) -> Self {
        self.sqrt()
    }
    fn is_finite(self) -> bool {
        <core::primitive::f32>::is_finite(self)
    }
}

impl NumericFloat for f64 {
    const HALF: Self = 0.5;
    const INFINITY: Self = Self::INFINITY;
    const NEG_INFINITY: Self = Self::NEG_INFINITY;
    const PI: Self = std::f64::consts::PI;

    fn sin_cos_numeric(self) -> (Self, Self) {
        self.sin_cos()
    }
    fn tan_numeric(self) -> Self {
        self.tan()
    }
    fn is_nan_numeric(self) -> bool {
        self.is_nan()
    }
    fn copysign_numeric(self, sign: Self) -> Self {
        self.copysign(sign)
    }
    fn min_numeric(self, other: Self) -> Self {
        self.min(other)
    }
    fn max_numeric(self, other: Self) -> Self {
        self.max(other)
    }
    fn powf_numeric(self, other: Self) -> Self {
        self.powf(other)
    }
    fn floor_numeric(self) -> Self {
        self.floor()
    }
    fn atan2(self, other: Self) -> Self {
        Self::atan2(self, other)
    }
    fn acos(self) -> Self {
        Self::acos(self)
    }
    fn round_numeric(self) -> Self {
        self.round()
    }
    fn from_f32(f: f32) -> Self {
        f as f64
    }
    fn numeric_sqrt(self) -> Self {
        self.sqrt()
    }
    fn is_finite(self) -> bool {
        <core::primitive::f64>::is_finite(self)
    }
}

pub trait NumericSigNum {
    fn signum_numeric(self) -> Self;
}

impl NumericSigNum for f32 {
    fn signum_numeric(self) -> Self {
        self.signum()
    }
}

impl NumericSigNum for f64 {
    fn signum_numeric(self) -> Self {
        self.signum()
    }
}

pub trait NumericAbs {
    fn numeric_abs(self) -> Self;
}

impl Numeric for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl NumericAbs for f32 {
    fn numeric_abs(self) -> Self {
        self.abs()
    }
}

impl Numeric for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl NumericAbs for f64 {
    fn numeric_abs(self) -> Self {
        self.abs()
    }
}

impl Numeric for i8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl NumericAbs for i8 {
    fn numeric_abs(self) -> Self {
        self.abs()
    }
}

impl NumericSigNum for i8 {
    fn signum_numeric(self) -> Self {
        self.signum()
    }
}

impl Numeric for i16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl NumericAbs for i16 {
    fn numeric_abs(self) -> Self {
        self.abs()
    }
}

impl NumericSigNum for i16 {
    fn signum_numeric(self) -> Self {
        self.signum()
    }
}

impl Numeric for i32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl NumericAbs for i32 {
    fn numeric_abs(self) -> Self {
        self.abs()
    }
}

impl NumericSigNum for i32 {
    fn signum_numeric(self) -> Self {
        self.signum()
    }
}

impl Numeric for i64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl NumericAbs for i64 {
    fn numeric_abs(self) -> Self {
        self.abs()
    }
}

impl NumericSigNum for i64 {
    fn signum_numeric(self) -> Self {
        self.signum()
    }
}

impl Numeric for i128 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl NumericAbs for i128 {
    fn numeric_abs(self) -> Self {
        self.abs()
    }
}

impl NumericSigNum for i128 {
    fn signum_numeric(self) -> Self {
        self.signum()
    }
}

impl Numeric for u8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl Numeric for u16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl Numeric for u32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl Numeric for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl Numeric for u128 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl Numeric for usize {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    fn numeric_min(self, other: Self) -> Self {
        self.min(other)
    }
    fn numeric_max(self, other: Self) -> Self {
        self.max(other)
    }
    fn numeric_clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}
