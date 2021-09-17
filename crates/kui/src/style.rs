use crate::*;

pub trait GetStandardStyleTrait {
    fn standard(&self) -> &StandardStyle;
}

pub struct StandardStyle {
    fonts: Vec<fontdue::Font>,
    pub primary_text_color: Color,
    pub primary_font: Font,
    pub heading_font: Font,
    pub padding: f32,
    /// Used for things like buttons
    pub primary_color: Color,
    /// Used for things like pressed buttons
    pub primary_variant_color: Color,
    /// How rounded things like buttons are
    pub rounding: f32,
}

impl GetStandardStyleTrait for StandardStyle {
    fn standard(&self) -> &StandardStyle {
        self
    }
}

impl StandardStyle {
    pub fn new() -> Self {
        Self {
            fonts: Vec::new(),
            primary_text_color: Color::WHITE,
            primary_font: Font::default(),
            heading_font: Font::default(),
            padding: 40.0,
            primary_color: Color::from_srgb_hex(0x1472FF, 1.0),
            primary_variant_color: Color::from_srgb_hex(0x085FE0, 1.0),
            rounding: 15.,
        }
    }

    pub fn new_font(&mut self, data: &[u8]) -> Result<Font, &'static str> {
        self.fonts.push(fontdue::Font::from_bytes(
            data,
            fontdue::FontSettings::default(),
        )?);
        Ok(Font(self.fonts.len() - 1))
    }

    pub fn fonts(&self) -> &[fontdue::Font] {
        &self.fonts
    }
}
