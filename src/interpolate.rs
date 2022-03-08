pub trait InterpolateTrait {
    fn interpolate(&self, other: &Self, amount: f32) -> Self;
}

pub fn smooth_step(amount: f32) -> f32 {
    amount * amount * (3.0 - 2.0 * amount)
}
