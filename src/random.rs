use core::ops::Range;
use kmath::{Quat, Vector};
use std::usize;

use crate::*;

#[derive(ManualSerdeComponent, Clone)]
pub struct Random {
    random_number_generator: oorandom::Rand32,
}

impl<S: Serializer> kserde::Serialize<S> for Random {
    fn serialize(&self, serializer: &mut S) {
        let state = self.random_number_generator.state();
        serializer.begin_object();
        serializer.serialize(&state.0);
        serializer.serialize(&state.1);
        serializer.end_object();
    }
}

impl<'a, D: Deserializer<'a>> kserde::Deserialize<'a, D> for Random {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        // This may not deserialize correctly because `kserde` stores u64s as f64s.
        let i0 = u64::deserialize(deserializer)?;
        let i1 = u64::deserialize(deserializer)?;
        Some(Self {
            random_number_generator: oorandom::Rand32::from_state((i0, i1)),
        })
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

impl Random {
    /// Create a new `Random` with a seed based on the current system time.
    pub fn new() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let start = std::time::SystemTime::now();
            let since_the_epoch = start.duration_since(std::time::UNIX_EPOCH).unwrap();
            let seed = (since_the_epoch.as_nanos() % u64::MAX as u128) as u64;
            Self::new_with_seed(seed)
        }
        #[cfg(target_arch = "wasm32")]
        {
            // Use `date_now` instead of `now` because `now` is milliseconds since
            // program start and will often get similar seeds.
            Self::new_with_seed(kwasm::libraries::instant::date_now() as u64)
        }
    }

    pub fn new_with_seed(seed: u64) -> Self {
        let random_number_generator = oorandom::Rand32::new(seed);
        Self {
            random_number_generator,
        }
    }

    /// Resets the random number generator and sets its seed.
    /// Seeding a random number generator ensures that the same values are produced for each
    /// call.
    pub fn set_seed(&mut self, seed: u64) {
        self.random_number_generator = oorandom::Rand32::new(seed);
    }

    // TODO: It'd be better if more of these functions were generic based on the type.
    // But that would require some extra trait shenanigans or something like a custom
    // random implementation.

    /// Generates a random f32
    pub fn f32(&mut self) -> f32 {
        self.random_number_generator.rand_float()
    }

    /// Generates a random i32
    pub fn i32(&mut self) -> i32 {
        self.random_number_generator.rand_i32()
    }

    /// Generates a random u32
    pub fn u32(&mut self) -> u32 {
        self.random_number_generator.rand_u32()
    }

    /// Generates a random f32 within the range
    pub fn range_f32(&mut self, range: Range<f32>) -> f32 {
        let span = range.end - range.start;
        self.random_number_generator.rand_float() * span + range.start
    }

    /// Generates a random i32 within the range
    pub fn range_i32(&mut self, range: Range<i32>) -> i32 {
        let span = range.end - range.start;
        (self.random_number_generator.rand_float() * span as f32) as i32 + range.start
    }

    /// Generates a random u32 within the range
    pub fn range_u32(&mut self, range: Range<u32>) -> u32 {
        let span = range.end - range.start;
        (self.random_number_generator.rand_float() * span as f32) as u32 + range.start
    }

    /// Produces a Quaternion with a random orientation.
    /// TODO: Test and verify that this has a reasonable distribution.
    pub fn quaternion(&mut self) -> Quat {
        Quat::from_angle_axis(self.f32(), self.normalized_vec())
    }

    /// Returns a normalized vector pointing a random direction.
    /// TODO: Test and verify that this has a reasonable distribution.
    pub fn normalized_vec<const DIMENSIONS: usize>(&mut self) -> Vector<f32, DIMENSIONS> {
        let mut v = Vector::<f32, DIMENSIONS>::ZERO;
        for v in &mut v {
            *v = 1.0 - 2.0 * self.f32();
        }
        v.normalized()
    }

    /// Returns a random point within a circle (if `DIMENSIONS` == 2) or sphere (if `DIMENSIONS` == 3)
    pub fn point_in_unit_sphere<const DIMENSIONS: usize>(&mut self) -> Vector<f32, DIMENSIONS> {
        self.normalized_vec() * self.f32()
    }

    /// Selects a random item from a slice.
    pub fn select_from_slice<'a, T>(&mut self, options: &'a [T]) -> &'a T {
        &options[self.range_u32(0..options.len() as u32) as usize]
    }

    /// Returns a random [Color] within the sRGB gamut.
    pub fn color(&mut self) -> crate::Color {
        Color::new(self.f32(), self.f32(), self.f32(), 1.0)
    }
}
