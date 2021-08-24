pub use kolor::spaces as color_spaces;
pub use kolor::ColorSpace;
// For now we're just using an f32, but maybe in the future there'd be a reason to change this.
type FType = f32;

// Color conversions in this file are computed every time a conversion occurs.
// It may be faster to precalculate the conversions and store them in a matrix.

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Color {
    x: FType,
    y: FType,
    z: FType,
    pub alpha: FType,
}

impl kecs::ComponentTrait for Color {
    fn clone_components(
        _entity_migrator: &mut kecs::EntityMigrator,
        items: &[Self],
    ) -> Option<Vec<Self>> {
        Some(items.into())
    }
}

/// A [Color] with `red`, `green`, and `blue`, components but without a specified `ColorSpace`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RGBColor {
    pub red: FType,
    pub green: FType,
    pub blue: FType,
    pub alpha: FType,
}

impl Color {
    /// Create a new [Color] from sRGB red, green, blue, and alpha (transparency) values.
    pub fn new(red: FType, green: FType, blue: FType, alpha: FType) -> Self {
        let converter =
            kolor::ColorConversion::new(kolor::spaces::ENCODED_SRGB, kolor::spaces::CIE_XYZ);
        let result = converter.convert(kolor::Vec3::new(red, green, blue));
        Self {
            x: result.x,
            y: result.y,
            z: result.z,
            alpha,
        }
    }

    pub fn new_from_bytes(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::new(
            red as f32 / 255.0,
            green as f32 / 255.0,
            blue as f32 / 255.0,
            alpha as f32 / 255.0,
        )
    }

    /// Creates a new [Color] from `red`, `green`, and `blue` components and you can specify the
    /// [ColorSpace] of the components.
    pub fn new_with_colorspace(
        x: FType,
        y: FType,
        z: FType,
        alpha: FType,
        color_space: ColorSpace,
    ) -> Self {
        let converter = kolor::ColorConversion::new(color_space, kolor::spaces::CIE_XYZ);
        let kolor::Vec3 { x, y, z } = converter.convert(kolor::Vec3::new(x, y, z));
        Self { x, y, z, alpha }
    }

    /// Creates a new color from the hex values of a number.
    pub fn from_srgb_hex(hex: u32, alpha: FType) -> Color {
        let red = ((hex >> 16) & 0xFF) as FType / 255.0;
        let green = ((hex >> 8) & 0xFF) as FType / 255.0;
        let blue = ((hex) & 0xFF) as FType / 255.0;
        Self::new(red, green, blue, alpha)
    }

    /// Convert this color to a [RGBColor] in a specified [ColorSpace]
    /// The red, green, and blue components may not actually correspond to red, green, and blue depending on the color space.
    pub fn to_rgb_color(self, color_space: ColorSpace) -> RGBColor {
        let converter = kolor::ColorConversion::new(kolor::spaces::CIE_XYZ, color_space);
        let result = converter.convert(kolor::Vec3::new(self.x, self.y, self.z));
        RGBColor {
            red: result.x,
            green: result.y,
            blue: result.z,
            alpha: self.alpha,
        }
    }

    /// Interpolates (synonyms: blend, mix, lerp) between two [Color]s.
    /// Colors are interpolated in the [OKLAB] [ColorSpace] for better results.
    /// Use
    pub fn interpolate(a: Self, b: Self, amount: FType) -> Self {
        Self::interpolate_in_color_space(a, b, amount, kolor::spaces::OKLAB)
    }

    /// Interpolates between two [Color]
    pub fn interpolate_in_color_space(
        a: Self,
        b: Self,
        amount: FType,
        color_space: ColorSpace,
    ) -> Self {
        let alpha = (b.alpha - a.alpha) * amount + a.alpha;
        let to_space = kolor::ColorConversion::new(kolor::spaces::CIE_XYZ, color_space);
        let a = to_space.convert(kolor::Vec3::new(a.x, a.y, a.z));
        let b = to_space.convert(kolor::Vec3::new(b.x, b.y, b.z));
        let interpolated = (b - a) * amount + a;
        let from_space = kolor::ColorConversion::new(color_space, kolor::spaces::CIE_XYZ);
        let kolor::Vec3 { x, y, z } = from_space.convert(kolor::Vec3::new(
            interpolated.x,
            interpolated.y,
            interpolated.z,
        ));
        Self { x, y, z, alpha }
    }

    /// Sets the lightness in the [OKLCH] [ColorSpace]
    pub fn set_lightness(&mut self, lightness: f32) {
        let to_space = kolor::ColorConversion::new(kolor::spaces::CIE_XYZ, kolor::spaces::OKLCH);
        let mut in_space = to_space.convert(kolor::Vec3::new(self.x, self.y, self.z));
        in_space.x = lightness;
        let from_space = kolor::ColorConversion::new(kolor::spaces::OKLCH, kolor::spaces::CIE_XYZ);
        let kolor::Vec3 { x, y, z } = from_space.convert(in_space);
        *self = Self {
            x,
            y,
            z,
            alpha: self.alpha,
        }
    }

    /// Sets the chroma in the [OKLCH] [ColorSpace]
    /// `chroma` should be between 0.0 and 1.0
    pub fn set_chroma(&mut self, chroma: f32) {
        let to_space = kolor::ColorConversion::new(kolor::spaces::CIE_XYZ, kolor::spaces::OKLCH);
        let mut in_space = to_space.convert(kolor::Vec3::new(self.x, self.y, self.z));
        in_space.y = chroma * 0.5;
        let from_space = kolor::ColorConversion::new(kolor::spaces::OKLCH, kolor::spaces::CIE_XYZ);
        let kolor::Vec3 { x, y, z } = from_space.convert(in_space);
        *self = Self {
            x,
            y,
            z,
            alpha: self.alpha,
        }
    }

    /// Sets the hue in the [OKLCH] [ColorSpace]
    /// `hue` should be between 0.0 and 1.0
    pub fn set_hue(&mut self, hue: f32) {
        let to_space = kolor::ColorConversion::new(kolor::spaces::CIE_XYZ, kolor::spaces::OKLCH);
        let mut in_space = to_space.convert(kolor::Vec3::new(self.x, self.y, self.z));
        in_space.z = hue * std::f32::consts::TAU;
        let from_space = kolor::ColorConversion::new(kolor::spaces::OKLCH, kolor::spaces::CIE_XYZ);
        let kolor::Vec3 { x, y, z } = from_space.convert(in_space);
        *self = Self {
            x,
            y,
            z,
            alpha: self.alpha,
        }
    }

    pub fn set_alpha(&mut self, alpha: FType) {
        self.alpha = alpha;
    }

    /// White in sRGB
    pub const WHITE: Color = Color {
        x: 0.950470,
        y: 1.0000,
        z: 1.08883,
        alpha: 1.0,
    };

    /// Black in sRGB
    pub const BLACK: Color = Color {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        alpha: 1.0,
    };

    /// Red in sRGB
    pub const RED: Color = Color {
        x: 0.412456,
        y: 0.212673,
        z: 0.019334,
        alpha: 1.0,
    };

    /// Green in sRGB
    pub const GREEN: Color = Color {
        x: 0.357576,
        y: 0.715152,
        z: 0.119192,
        alpha: 1.0,
    };

    /// Blue in sRGB
    pub const BLUE: Color = Color {
        x: 0.180437,
        y: 0.072175,
        z: 0.950304,
        alpha: 1.0,
    };
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(color: (f32, f32, f32, f32)) -> Self {
        Color::new(color.0, color.1, color.2, color.3)
    }
}
