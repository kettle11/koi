mod color_constants;
pub use color_constants::*;

pub use kolor::spaces as color_spaces;
pub use kolor::ColorSpace;
use kserde::SerializeDeserialize;
// For now we're just using an f32, but maybe in the future there'd be a reason to change this.
type FType = f32;

// Color conversions in this file are computed every time a conversion occurs.
// It may be faster to precalculate the conversions and store them in a matrix.

/// koi's color type. [Color] has various helper functions designed to make it easier
/// to work with color.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, SerializeDeserialize)]
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

impl Color {
    /// Create a new [Color] from sRGB red, green, blue, and alpha (transparency) values.
    pub fn new(red: FType, green: FType, blue: FType, alpha: FType) -> Self {
        // Internally colors are stored in CIE_XYZ color space.
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

    /// Creates a new [Color] from `red`, `green`, and `blue` components with a specified [ColorSpace].
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

    /// Outputs a [kmath::Vec4] with this [Color]'s values as *encoded* (non-linear) sRGB.
    pub fn to_srgb(self) -> kmath::Vec4 {
        self.to_rgb_color(color_spaces::ENCODED_SRGB)
    }

    /// Outputs a [kmath::Vec4] with this [Color]'s values as *non-encoded* (linear) sRGB.
    pub fn to_linear_srgb(self) -> kmath::Vec4 {
        self.to_rgb_color(color_spaces::LINEAR_SRGB)
    }

    pub fn from_linear_srgb(red: FType, green: FType, blue: FType, alpha: FType) -> Self {
        // Internally colors are stored in CIE_XYZ color space.
        let converter =
            kolor::ColorConversion::new(kolor::spaces::LINEAR_SRGB, kolor::spaces::CIE_XYZ);
        let result = converter.convert(kolor::Vec3::new(red, green, blue));
        Self {
            x: result.x,
            y: result.y,
            z: result.z,
            alpha,
        }
    }

    /// Convert this color to a [RGBColor] in a specified [ColorSpace]
    /// The red, green, and blue components may not actually correspond to red, green, and blue depending on the color space.
    pub fn to_rgb_color(self, color_space: ColorSpace) -> kmath::Vec4 {
        let converter = kolor::ColorConversion::new(kolor::spaces::CIE_XYZ, color_space);
        let result = converter.convert(kolor::Vec3::new(self.x, self.y, self.z));
        kmath::Vec4::new(result.x, result.y, result.z, self.alpha)
    }

    /// Interpolates (synonyms: blend, mix, lerp) between two [Color]s.
    /// Colors are interpolated in the [OKLAB] [ColorSpace] for better results.
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
    pub fn with_lightness(self, lightness: f32) -> Self {
        let to_space = kolor::ColorConversion::new(kolor::spaces::CIE_XYZ, kolor::spaces::OKLCH);
        let mut in_space = to_space.convert(kolor::Vec3::new(self.x, self.y, self.z));
        in_space.x = lightness;
        let from_space = kolor::ColorConversion::new(kolor::spaces::OKLCH, kolor::spaces::CIE_XYZ);
        let kolor::Vec3 { x, y, z } = from_space.convert(in_space);
        Self {
            x,
            y,
            z,
            alpha: self.alpha,
        }
    }

    /// Sets the chroma in the [OKLCH] [ColorSpace]
    /// `chroma` should be between 0.0 and 1.0
    pub fn with_chroma(self, chroma: f32) -> Self {
        let to_space = kolor::ColorConversion::new(kolor::spaces::CIE_XYZ, kolor::spaces::OKLCH);
        let mut in_space = to_space.convert(kolor::Vec3::new(self.x, self.y, self.z));
        in_space.y = chroma * 0.5;
        let from_space = kolor::ColorConversion::new(kolor::spaces::OKLCH, kolor::spaces::CIE_XYZ);
        let kolor::Vec3 { x, y, z } = from_space.convert(in_space);
        Self {
            x,
            y,
            z,
            alpha: self.alpha,
        }
    }

    /// Sets the hue in the [OKLCH] [ColorSpace]
    /// `hue` should be between 0.0 and 1.0
    pub fn with_hue(self, hue: f32) -> Self {
        let to_space = kolor::ColorConversion::new(kolor::spaces::CIE_XYZ, kolor::spaces::OKLCH);
        let mut in_space = to_space.convert(kolor::Vec3::new(self.x, self.y, self.z));
        in_space.z = hue * std::f32::consts::TAU;
        let from_space = kolor::ColorConversion::new(kolor::spaces::OKLCH, kolor::spaces::CIE_XYZ);
        let kolor::Vec3 { x, y, z } = from_space.convert(in_space);
        Self {
            x,
            y,
            z,
            alpha: self.alpha,
        }
    }

    /// With a specific alpha (transparency).
    pub fn with_alpha(mut self, alpha: FType) -> Self {
        self.alpha = alpha;
        self
    }

    /// Returns this color's lightness, chroma, and hue as defiend in OKLCH color space.
    pub fn get_lightness_chroma_hue(self) -> (f32, f32, f32) {
        let to_space = kolor::ColorConversion::new(kolor::spaces::CIE_XYZ, kolor::spaces::OKLCH);
        let mut in_space = to_space.convert(kolor::Vec3::new(self.x, self.y, self.z));
        in_space.z /= std::f32::consts::TAU;
        if in_space.z < 0.0 {
            in_space.z += 1.0;
        }
        (in_space.x, in_space.y, in_space.z)
    }

    pub fn xyza(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.alpha)
    }

    pub fn from_xyza(x: f32, y: f32, z: f32, alpha: f32) -> Self {
        Self { x, y, z, alpha }
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(color: (f32, f32, f32, f32)) -> Self {
        Color::new(color.0, color.1, color.2, color.3)
    }
}
