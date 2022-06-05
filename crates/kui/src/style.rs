use crate::*;

pub struct StandardStyle {
    pub primary_text_color: Color,
    pub primary_text_size: f32,
    pub heading_text_size: f32,
    pub primary_font: Font,
    pub heading_font: Font,
    pub padding: f32,
    /// Used for things like buttons
    pub primary_color: Color,
    pub disabled_color: Color,
    /// Used for things like pressed buttons
    pub primary_variant_color: Color,
    /// How rounded things like buttons are
    pub rounding: f32,
    /// How much the UI is scaled from the native pixel size. Usually set higher for high-dpi devices.
    // Maybe this shouldn't be here
    pub ui_scale: f32,
    pub background_color: Color,
    pub animation_curve: fn(f32) -> f32,
    pub animation_time: f32,
}

impl Default for StandardStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl StandardStyle {
    pub fn new() -> Self {
        Self {
            primary_text_color: Color::new_from_bytes(40, 40, 40, 255),
            primary_text_size: 16.,
            heading_text_size: 32.,
            primary_font: Font::default(),
            heading_font: Font::default(),
            padding: 10.0,
            primary_color: Color::from_srgb_hex(0xC4C4C4, 1.0),
            primary_variant_color: Color::from_srgb_hex(0xF6F6F6, 1.0),
            rounding: 2.,
            ui_scale: 1.0,
            disabled_color: Color::from_srgb_hex(0x9B9B9B, 1.0),
            background_color: Color::WHITE,
            animation_curve: smooth_step,
            animation_time: 0.15,
        }
    }
}

impl GetStandardStyle for StandardStyle {
    fn standard_style(&self) -> &StandardStyle {
        self
    }
    fn standard_style_mut(&mut self) -> &mut StandardStyle {
        self
    }
}

fn smooth_step(amount: f32) -> f32 {
    amount * amount * (3.0 - 2.0 * amount)
}
