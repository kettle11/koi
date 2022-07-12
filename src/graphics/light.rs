use crate::*;

#[derive(Clone)]
pub enum LightMode {
    /// For large light sources that effect an entire environment, like the sun.
    Directional,
    /// For light sources that emit from a point, like a lamp.
    Point { radius: f32 },
}

#[derive(Component, Clone)]
pub struct Light {
    pub color: Color,
    pub intensity: f32,
    pub light_mode: LightMode,
    pub ambient_light_amount: f32,
}

impl Light {
    pub fn new(light_mode: LightMode, color: Color, intensity: f32) -> Self {
        Self {
            color,
            intensity,
            light_mode,
            ambient_light_amount: 0.0,
        }
    }
    pub fn with_ambient_light(mut self, amount: f32) -> Self {
        self.ambient_light_amount = amount;
        self
    }
}
