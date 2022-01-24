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
    pub column_spacing: f32,
    pub row_spacing: f32,
    pub background_color: Color,
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
            primary_text_size: 18.,
            heading_text_size: 32.,
            primary_font: Font::default(),
            heading_font: Font::default(),
            padding: 40.0,
            primary_color: Color::from_srgb_hex(0xB9B9B9, 1.0),
            primary_variant_color: Color::from_srgb_hex(0xF6F6F6, 1.0),
            rounding: 8.,
            ui_scale: 1.0,
            column_spacing: 30.,
            row_spacing: 30.,
            disabled_color: Color::from_srgb_hex(0x9B9B9B, 1.0),
            background_color: Color::WHITE,
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
