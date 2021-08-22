use super::{conversion::ColorConversion, transform::ColorTransform};
use crate::{FType, Vec3};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// A [TransformFn] identifies an invertible mapping of colors in a linear [ColorSpace].
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum TransformFn {
    NONE,
    /// The sRGB transfer functions (aka "gamma correction")
    sRGB,
    /// Oklab conversion from xyz
    Oklab,
    /// Oklch (Oklab's LCh variant) conversion from xyz
    Oklch,
    /// CIE xyY transform
    CIE_xyY,
    /// CIELAB transform
    CIELAB,
    /// CIELCh transform
    CIELCh,
    /// CIE 1960 UCS transform
    CIE_1960_UCS,
    /// CIE 1960 UCS transform in uvV form
    CIE_1960_UCS_uvV,
    /// CIE 1964 UVW transform
    CIE_1964_UVW,
    /// CIE 1976 Luv transform
    CIE_1976_Luv,
    /// (Hue, Saturation, Lightness),
    /// where L is defined as the average of the largest and smallest color components
    HSL,
    /// (Hue, Saturation, Value),
    /// where V is defined as the largest component of a color
    HSV,
    /// (Hue, Saturation, Intensity),
    /// where I is defined as the average of the three components
    HSI,
    /// BT.2100 ICtCp with PQ transfer function
    ICtCp_PQ,
    /// BT.2100 ICtCp with HLG transfer function
    ICtCp_HLG,
    /// The BT.601/BT.709/BT.2020 (they are equivalent) OETF and inverse.
    BT_601,
    /// SMPTE ST 2084:2014 aka "Perceptual Quantizer" transfer functions used in BT.2100
    /// for digitally created/distributed HDR content.
    PQ,
    // ACEScc is a logarithmic transform
    // ACES_CC,
    // ACEScct is a logarithmic transform with toe
    // ACES_CCT,
}
impl TransformFn {
    pub const ENUM_COUNT: TransformFn = TransformFn::ICtCp_HLG;
}
/// [RGBPrimaries] is a set of primary colors picked to define an RGB color space.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum RGBPrimaries {
    // Primaries
    NONE,
    /// BT.709 is the sRGB primaries.
    BT_709,
    // BT_2020 uses the same primaries as BT_2100
    BT_2020,
    AP0,
    AP1,
    /// P3 is the primaries for DCI-P3 and the variations with different white points
    P3,
    ADOBE_1998,
    ADOBE_WIDE,
    APPLE,
    PRO_PHOTO,
    CIE_RGB,
    /// The reference XYZ color space
    CIE_XYZ,
}
impl RGBPrimaries {
    pub const ENUM_COUNT: RGBPrimaries = RGBPrimaries::CIE_XYZ;
    pub const fn values(&self) -> &[[FType; 2]; 3] {
        match self {
            Self::NONE => &[[0.0; 2]; 3],
            Self::BT_709 => &[[0.64, 0.33], [0.30, 0.60], [0.15, 0.06]],
            Self::BT_2020 => &[[0.708, 0.292], [0.17, 0.797], [0.131, 0.046]],
            Self::AP0 => &[[0.7347, 0.2653], [0.0000, 1.0000], [0.0001, -0.0770]],
            Self::AP1 => &[[0.713, 0.293], [0.165, 0.830], [0.128, 0.044]],
            Self::ADOBE_1998 => &[[0.64, 0.33], [0.21, 0.71], [0.15, 0.06]],
            Self::ADOBE_WIDE => &[[0.735, 0.265], [0.115, 0.826], [0.157, 0.018]],
            Self::PRO_PHOTO => &[
                [0.734699, 0.265301],
                [0.159597, 0.840403],
                [0.036598, 0.000105],
            ],
            Self::APPLE => &[[0.625, 0.34], [0.28, 0.595], [0.155, 0.07]],
            Self::P3 => &[[0.680, 0.320], [0.265, 0.690], [0.150, 0.060]],
            Self::CIE_RGB => &[[0.7350, 0.2650], [0.2740, 0.7170], [0.1670, 0.0090]],
            Self::CIE_XYZ => &[[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]],
        }
    }
}

/// A [WhitePoint] defines the color "white" in an RGB color system.
/// White points are derived from an "illuminant" which are defined
/// as some reference lighting condition based on a Spectral Power Distribution.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum WhitePoint {
    NONE,
    /// Incandescent/tungsten
    A,
    /// Old direct sunlight at noon
    B,
    /// Old daylight
    C,
    /// Equal energy
    E,
    /// ICC profile PCS
    D50,
    /// Mid-morning daylight
    D55,
    D60,
    /// Daylight, sRGB, Adobe-RGB
    D65,
    /// North sky daylight
    D75,
    /// P3-DCI white point, sort of greenish
    P3_DCI,
    /// Cool fluorescent
    F2,
    /// Daylight fluorescent, D65 simulator
    F7,
    /// Ultralume 40, Philips TL84
    F11,
}
impl WhitePoint {
    pub const ENUM_COUNT: WhitePoint = WhitePoint::F11;
    pub const fn values(&self) -> &'static [FType; 3] {
        match self {
            Self::NONE => &[0.0, 0.0, 0.0],
            Self::A => &[1.09850, 1.00000, 0.35585],
            Self::B => &[0.99072, 1.00000, 0.85223],
            Self::C => &[0.98074, 1.00000, 1.18232],
            Self::D50 => &[0.96422, 1.00000, 0.82521],
            Self::D55 => &[0.95682, 1.00000, 0.92149],
            Self::D60 => &[0.9523, 1.00000, 1.00859],
            Self::D65 => &[0.95047, 1.00000, 1.08883],
            Self::D75 => &[0.94972, 1.00000, 1.22638],
            Self::P3_DCI => &[0.89458689458, 1.00000, 0.95441595441],
            Self::E => &[1.00000, 1.00000, 1.00000],
            Self::F2 => &[0.99186, 1.00000, 0.67393],
            Self::F7 => &[0.95041, 1.00000, 1.08747],
            Self::F11 => &[1.00962, 1.00000, 0.64350],
        }
    }
}

/// A color space defined in data by its [Primaries][RGBPrimaries], [white point][WhitePoint], and an optional [invertible transform function][TransformFn].
///
/// See [spaces][crate::spaces] for defined color spaces.
///
/// [ColorSpace] assumes that a color space is one of
/// - the CIE XYZ color space
/// - an RGB color space
/// - a color space which may be defined as an invertible mapping from one the above ([TransformFn])
///
/// An example of a [TransformFn] is the sRGB "opto-eletronic transfer function", or
/// "gamma compensation".
///
/// `kolor` makes the distinction between "linear" and "non-linear" color spaces, where a linear
/// color space can be defined as a linear transformation from the CIE XYZ color space.
///
/// [ColorSpace] contains a reference [WhitePoint] to represent a color space's reference illuminant.
///
/// A linear RGB [ColorSpace] can be thought of as defining a relative coordinate system in the CIE XYZ
/// color coordinate space, where three RGB primaries each define an axis pointing from
/// the black point (0,0,0) in CIE XYZ.
///
/// Non-linear [ColorSpace]s - such as sRGB with gamma compensation applied - are defined as a non-linear mapping from a linear
/// [ColorSpace]'s coordinate system.
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct ColorSpace {
    primaries: RGBPrimaries,
    white_point: WhitePoint,
    transform_fn: TransformFn,
}
impl ColorSpace {
    pub const fn new(
        primaries: RGBPrimaries,
        white_point: WhitePoint,
        transform_fn: TransformFn,
    ) -> Self {
        Self {
            primaries,
            white_point,
            transform_fn,
        }
    }
    pub(crate) const fn linear(primaries: RGBPrimaries, white_point: WhitePoint) -> Self {
        Self {
            primaries,
            white_point,
            transform_fn: TransformFn::NONE,
        }
    }
    /// Whether the color space has a non-linear transform applied
    pub fn is_linear(&self) -> bool {
        self.transform_fn == TransformFn::NONE
    }
    pub fn as_linear(&self) -> Self {
        Self {
            primaries: self.primaries,
            white_point: self.white_point,
            transform_fn: TransformFn::NONE,
        }
    }
    pub fn primaries(&self) -> RGBPrimaries {
        self.primaries
    }
    pub fn white_point(&self) -> WhitePoint {
        self.white_point
    }
    pub fn transform_function(&self) -> TransformFn {
        self.transform_fn
    }

    /// Creates a new color space with the primaries and white point from `this`,
    /// but with the provided [TransformFn].
    pub fn with_transform(&self, new_transform: TransformFn) -> Self {
        Self {
            primaries: self.primaries,
            white_point: self.white_point,
            transform_fn: new_transform,
        }
    }

    /// Creates a new color space with the transform function and white point from `this`,
    /// but with the provided [WhitePoint].
    pub fn with_whitepoint(&self, new_wp: WhitePoint) -> Self {
        Self {
            primaries: self.primaries,
            white_point: new_wp,
            transform_fn: self.transform_fn,
        }
    }

    /// Creates a new color space with the primaries and transform function from `this`,
    /// but with the provided [RGBPrimaries].
    pub fn with_primaries(&self, primaries: RGBPrimaries) -> Self {
        Self {
            primaries,
            white_point: self.white_point,
            transform_fn: self.transform_fn,
        }
    }

    /// Creates a CIE LAB color space using this space's white point.
    pub fn to_cielab(&self) -> Self {
        Self::new(RGBPrimaries::CIE_XYZ, self.white_point, TransformFn::CIELAB)
    }

    /// Creates a CIE uvV color space using this space's white point.
    #[allow(non_snake_case)]
    pub fn to_cie_xyY(&self) -> Self {
        Self::new(
            RGBPrimaries::CIE_XYZ,
            self.white_point,
            TransformFn::CIE_xyY,
        )
    }

    /// Creates a CIE LCh color space using this space's white point.
    pub fn to_cielch(&self) -> Self {
        Self::new(RGBPrimaries::CIE_XYZ, self.white_point, TransformFn::CIELCh)
    }
}
#[allow(non_upper_case_globals)]
pub mod color_spaces {
    use super::*;

    /// Linear sRGB is a linear encoding in [BT.709 primaries][RGBPrimaries::BT_709]
    /// with a [D65 whitepoint.][WhitePoint::D65]
    /// Linear sRGB is equivalent to [BT_709].
    pub const LINEAR_SRGB: ColorSpace = ColorSpace::linear(RGBPrimaries::BT_709, WhitePoint::D65);

    /// Encoded sRGB is [Linear sRGB][LINEAR_SRGB] with the [sRGB OETF](TransformFn::sRGB) applied (also called "gamma-compressed").
    pub const ENCODED_SRGB: ColorSpace =
        ColorSpace::new(RGBPrimaries::BT_709, WhitePoint::D65, TransformFn::sRGB);

    /// BT.709 is a linear encoding in [BT.709 primaries][RGBPrimaries::BT_709]
    /// with a [D65 whitepoint.][WhitePoint::D65]. It's equivalent to [Linear sRGB][LINEAR_SRGB]
    pub const BT_709: ColorSpace = ColorSpace::linear(RGBPrimaries::BT_709, WhitePoint::D65);

    /// Encoded BT.709 is [BT.709](BT_709) with the [BT.709 OETF](TransformFn::BT_601) applied.
    pub const ENCODED_BT_709: ColorSpace =
        ColorSpace::new(RGBPrimaries::BT_709, WhitePoint::D65, TransformFn::BT_601);

    /// ACEScg is a linear encoding in [AP1 primaries][RGBPrimaries::AP1]
    /// with a [D60 whitepoint][WhitePoint::D60].
    pub const ACES_CG: ColorSpace = ColorSpace::linear(RGBPrimaries::AP1, WhitePoint::D60);

    /// ACES2065-1 is a linear encoding in [AP0 primaries][RGBPrimaries::AP0] with a [D60 whitepoint][WhitePoint::D60].
    pub const ACES2065_1: ColorSpace = ColorSpace::linear(RGBPrimaries::AP0, WhitePoint::D60);

    /// CIE RGB is the original RGB space, defined in [CIE RGB primaries][RGBPrimaries::CIE_RGB]
    /// with white point [E][WhitePoint::E].
    pub const CIE_RGB: ColorSpace = ColorSpace::linear(RGBPrimaries::CIE_RGB, WhitePoint::E);

    /// CIE XYZ reference color space. Uses [CIE XYZ primaries][RGBPrimaries::CIE_XYZ]
    /// with white point [D65][WhitePoint::D65].
    pub const CIE_XYZ: ColorSpace = ColorSpace::linear(RGBPrimaries::CIE_XYZ, WhitePoint::D65);

    /// BT.2020 is a linear encoding in [BT.2020 primaries][RGBPrimaries::BT_2020]
    /// with a [D65 white point][WhitePoint::D65]
    /// BT.2100 has the same linear color space as BT.2020.
    pub const BT_2020: ColorSpace = ColorSpace::linear(RGBPrimaries::BT_2020, WhitePoint::D65);

    /// Encoded BT.2020 is [BT.2020](BT_2020) with the [BT.2020 OETF][TransformFn::BT_601] applied.
    pub const ENCODED_BT_2020: ColorSpace =
        ColorSpace::new(RGBPrimaries::BT_2020, WhitePoint::D65, TransformFn::BT_601);

    /// Encoded BT.2100 PQ is [BT.2020](BT_2020) (equivalent to the linear BT.2100 space) with
    /// the [Perceptual Quantizer inverse EOTF][TransformFn::PQ] applied.
    pub const ENCODED_BT_2100_PQ: ColorSpace =
        ColorSpace::new(RGBPrimaries::BT_2020, WhitePoint::D65, TransformFn::PQ);

    /// Oklab is a non-linear, perceptual encoding in [XYZ][RGBPrimaries::CIE_XYZ],
    /// with a [D65 whitepoint][WhitePoint::D65].
    ///
    /// Oklab's perceptual qualities make it a very attractive color space for performing
    /// blend operations between two colors which you want to be perceptually pleasing.
    /// See [this article](https://bottosson.github.io/posts/oklab/)
    /// for more on why you might want to use the Oklab colorspace.
    pub const OKLAB: ColorSpace =
        ColorSpace::new(RGBPrimaries::CIE_XYZ, WhitePoint::D65, TransformFn::Oklab);

    /// Oklch is a non-linear, perceptual encoding in [XYZ][RGBPrimaries::CIE_XYZ],
    /// with a [D65 whitepoint][WhitePoint::D65]. It is a variant of [Oklab][OKLAB]
    /// with LCh coordinates intead of Lab.
    ///
    /// Oklch's qualities make it a very attractive color space for performing
    /// computational modifications to a color. You can think of it as an improved
    /// version of an HSL/HSV-style color space. See [this article](https://bottosson.github.io/posts/oklab/)
    /// for more on why you might want to use the Oklch colorspace.
    pub const OKLCH: ColorSpace =
        ColorSpace::new(RGBPrimaries::CIE_XYZ, WhitePoint::D65, TransformFn::Oklch);

    /// ICtCp_PQ is a non-linear encoding in [BT.2020 primaries][RGBPrimaries::BT_2020],
    /// with a [D65 whitepoint][WhitePoint::D65], using the PQ transfer function
    pub const ICtCp_PQ: ColorSpace = ColorSpace::new(
        RGBPrimaries::BT_2020,
        WhitePoint::D65,
        TransformFn::ICtCp_PQ,
    );
    /// ICtCp_HLG is a non-linear encoding in [BT.2020 primaries][RGBPrimaries::BT_2020],
    /// with a [D65 whitepoint][WhitePoint::D65], using the HLG transfer function
    pub const ICtCp_HLG: ColorSpace = ColorSpace::new(
        RGBPrimaries::BT_2020,
        WhitePoint::D65,
        TransformFn::ICtCp_HLG,
    );

    /// Encoded Display P3 is [Display P3][DISPLAY_P3] with the [sRGB OETF](TransformFn::sRGB) applied.
    pub const ENCODED_DISPLAY_P3: ColorSpace =
        ColorSpace::new(RGBPrimaries::P3, WhitePoint::D65, TransformFn::sRGB);

    /// Display P3 by Apple is a linear encoding in [P3 primaries][RGBPrimaries::P3]
    /// with a [D65 white point][WhitePoint::D65]
    pub const DISPLAY_P3: ColorSpace = ColorSpace::linear(RGBPrimaries::P3, WhitePoint::D65);

    /// P3-D60 (ACES Cinema) is a linear encoding in [P3 primaries][RGBPrimaries::P3]
    /// with a [D60 white point][WhitePoint::D60]
    pub const P3_D60: ColorSpace = ColorSpace::linear(RGBPrimaries::P3, WhitePoint::D60);

    /// P3-DCI (Theater) is a linear encoding in [P3 primaries][RGBPrimaries::P3]
    /// with a [P3-DCI white point][WhitePoint::P3_DCI]
    pub const P3_THEATER: ColorSpace = ColorSpace::linear(RGBPrimaries::P3, WhitePoint::P3_DCI);

    /// Adobe RGB (1998) is a linear encoding in [Adobe 1998 primaries][RGBPrimaries::ADOBE_1998]
    /// with a [D65 white point][WhitePoint::D65]
    pub const ADOBE_1998: ColorSpace =
        ColorSpace::linear(RGBPrimaries::ADOBE_1998, WhitePoint::D65);

    /// Adobe Wide Gamut RGB is a linear encoding in [Adobe Wide primaries][RGBPrimaries::ADOBE_WIDE]
    /// with a [D50 white point][WhitePoint::D50]
    pub const ADOBE_WIDE: ColorSpace =
        ColorSpace::linear(RGBPrimaries::ADOBE_WIDE, WhitePoint::D50);

    /// Pro Photo RGB is a linear encoding in [Pro Photo primaries][RGBPrimaries::PRO_PHOTO]
    /// with a [D50 white point][WhitePoint::D50]
    pub const PRO_PHOTO: ColorSpace = ColorSpace::linear(RGBPrimaries::PRO_PHOTO, WhitePoint::D50);

    /// Apple RGB is a linear encoding in [Apple primaries][RGBPrimaries::APPLE]
    /// with a [D65 white point][WhitePoint::D65]
    pub const APPLE: ColorSpace = ColorSpace::linear(RGBPrimaries::APPLE, WhitePoint::D65);

    /// Array containing all built-in color spaces.
    pub const ALL_COLOR_SPACES: [ColorSpace; 22] = [
        color_spaces::LINEAR_SRGB,
        color_spaces::ENCODED_SRGB,
        color_spaces::BT_709,
        color_spaces::ENCODED_BT_709,
        color_spaces::BT_2020,
        color_spaces::ENCODED_BT_2020,
        color_spaces::ENCODED_BT_2100_PQ,
        color_spaces::ACES_CG,
        color_spaces::ACES2065_1,
        color_spaces::CIE_RGB,
        color_spaces::CIE_XYZ,
        color_spaces::OKLAB,
        color_spaces::ICtCp_PQ,
        color_spaces::ICtCp_HLG,
        color_spaces::PRO_PHOTO,
        color_spaces::APPLE,
        color_spaces::P3_D60,
        color_spaces::P3_THEATER,
        color_spaces::DISPLAY_P3,
        color_spaces::ENCODED_DISPLAY_P3,
        color_spaces::ADOBE_1998,
        color_spaces::ADOBE_WIDE,
    ];
}

/// [Color] is a 3-component vector defined in a [ColorSpace].
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Color {
    pub value: Vec3,
    pub space: ColorSpace,
}
impl Color {
    pub fn new(x: FType, y: FType, z: FType, space: ColorSpace) -> Self {
        Self {
            value: Vec3::new(x, y, z),
            space,
        }
    }
    pub fn space(&self) -> ColorSpace {
        self.space
    }

    /// Equivalent to `Color::new(x, y, z, kolor::spaces::ENCODED_SRGB)`
    pub fn srgb(x: FType, y: FType, z: FType) -> Self {
        Self {
            value: Vec3::new(x, y, z),
            space: color_spaces::ENCODED_SRGB,
        }
    }

    /// Returns a [Color] with this color converted into the provided [ColorSpace].
    pub fn to(&self, space: ColorSpace) -> Color {
        let conversion = ColorConversion::new(self.space, space);
        let new_color = conversion.convert(self.value);
        Color {
            space,
            value: new_color,
        }
    }
    pub fn to_linear(&self) -> Color {
        if self.space.is_linear() {
            *self
        } else {
            let transform = ColorTransform::new(self.space.transform_function(), TransformFn::NONE)
                .unwrap_or_else(|| {
                    panic!(
                        "expected transform for {:?}",
                        self.space.transform_function()
                    )
                });
            let new_color_value = transform.apply(self.value, self.space().white_point);
            Self {
                value: new_color_value,
                space: self.space.as_linear(),
            }
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;
    use crate::details::conversion::LinearColorConversion;
    use crate::details::xyz::{rgb_to_xyz, xyz_to_rgb};
    use color_spaces as spaces;
    #[test]
    fn linear_srgb_to_aces_cg() {
        let conversion = LinearColorConversion::new(spaces::LINEAR_SRGB, spaces::ACES_CG);
        let result = conversion.convert(Vec3::new(0.35, 0.2, 0.8));
        assert_eq!(result, Vec3::new(0.32276854, 0.21838512, 0.72592676));
    }

    #[test]
    fn linear_srgb_to_cie_rgb() {
        let conversion = ColorConversion::new(spaces::LINEAR_SRGB, spaces::CIE_RGB);
        let result = conversion.convert(Vec3::new(0.35, 0.2, 0.8));
        assert_eq!(result, Vec3::new(0.3252983, 0.27015764, 0.73588717));
    }

    #[test]
    fn linear_srgb_to_aces_2065_1() {
        let conversion = ColorConversion::new(spaces::LINEAR_SRGB, spaces::ACES2065_1);
        let result = conversion.convert(Vec3::new(0.35, 0.2, 0.8));
        assert_eq!(result, Vec3::new(0.3741492, 0.27154857, 0.7261116));
    }

    #[test]
    fn linear_srgb_to_srgb() {
        let transform = ColorTransform::new(TransformFn::NONE, TransformFn::sRGB).unwrap();
        let test = Vec3::new(0.35, 0.1, 0.8);
        let result = transform.apply(test, WhitePoint::D65);
        assert_eq!(result, Vec3::new(0.6262097, 0.34919018, 0.9063317));
    }

    // #[test]
    // fn working_space_conversions() {
    //     // just make sure we aren't missing a conversion
    //     for src in &WORKING_SPACE_BY_WHITE_POINT {
    //         for dst in &WORKING_SPACE_BY_WHITE_POINT {
    //             let conversion = LinearColorConversion::new(*src, *dst);
    //             let mut result = Vec3::new(0.35, 0.2, 0.8);
    //             conversion.apply(&mut result);
    //         }
    //     }
    // }

    #[test]
    fn aces_cg_to_srgb() {
        let conversion = ColorConversion::new(spaces::ACES_CG, spaces::ENCODED_SRGB);
        let result = conversion.convert(Vec3::new(0.35, 0.1, 0.8));
        assert_eq!(result, Vec3::new(0.46201152, 0.06078783, 0.8996733));
    }

    #[test]
    fn aces2065_1_to_xyz_test() {
        let rgb_to_xyz = rgb_to_xyz(
            spaces::ACES2065_1.primaries().values(),
            spaces::ACES2065_1.white_point().values(),
        );

        let roundtrip = rgb_to_xyz.inverse() * rgb_to_xyz;
        println!("{:?}\n{:?}", rgb_to_xyz, roundtrip,);
        // println!(
        //     "{:?}",
        //     xyz_to_rgb(
        //         ColorSpace::ACES2065_1.primaries().values(),
        //         ColorSpace::ACES2065_1.white_point().values()
        //     )
        // );
    }

    #[test]
    fn rgb_to_xyz_test() {
        println!(
            "{:?}",
            rgb_to_xyz(
                spaces::LINEAR_SRGB.primaries().values(),
                spaces::LINEAR_SRGB.white_point().values()
            )
        );
        println!(
            "{:?}",
            xyz_to_rgb(
                spaces::LINEAR_SRGB.primaries().values(),
                spaces::LINEAR_SRGB.white_point().values()
            )
        );
    }

    #[test]
    fn cat_test() {
        println!(
            "{:?}",
            crate::details::cat::chromatic_adaptation_transform(
                Vec3::from_slice(WhitePoint::D65.values()),
                Vec3::from_slice(WhitePoint::E.values()),
                crate::details::cat::LMSConeSpace::VonKries,
            )
        );
    }

    #[test]
    fn oklab_test() {
        let xyz = Color::new(
            0.0,
            1.0,
            0.0,
            ColorSpace::new(RGBPrimaries::CIE_XYZ, WhitePoint::D65, TransformFn::NONE),
        );
        let oklab = xyz.to(spaces::OKLAB);
        println!(
            "conversion {:?}",
            ColorConversion::new(xyz.space(), oklab.space())
        );
        println!("xyz {:?}", xyz.value);
        println!("oklab {:?}", oklab.value);
    }

    #[test]
    fn cielab_test() {
        let srgb = Color::new(1.0, 0.5, 0.0, spaces::ENCODED_SRGB);
        let cielab = srgb.to(srgb.space.to_cielab());
        let cielab_inverse = cielab.to(srgb.space);
        let cielch = srgb.to(srgb.space.to_cielch());
        let cielch_inverse = cielch.to(srgb.space);
        let xyY = srgb.to(srgb.space.to_cie_xyY());
        let xyY_inverse = xyY.to(srgb.space);
        println!(
            "conversion {:?}",
            ColorConversion::new(srgb.space(), cielab.space())
        );
        println!("srgb {:?}", srgb.value);
        println!("cielab {:?}", cielab.value);
        println!("cielab_inverse {:?}", cielab_inverse.value);
        println!("cielch {:?}", cielch.value);
        println!("cielch_inverse {:?}", cielch_inverse.value);
        println!("xyY {:?}", xyY.value);
        println!("xyY_inverse {:?}", xyY_inverse.value);
        println!(
            " xyz {:?}",
            srgb.to(ColorSpace::new(
                RGBPrimaries::CIE_XYZ,
                WhitePoint::D65,
                TransformFn::NONE
            ))
        );
    }

    #[test]
    fn cie_uvV_test() {
        let srgb = Color::new(1.0, 0.5, 0.0, spaces::ENCODED_SRGB);
        let uvV = srgb.to(srgb.space.to_cielab());
        let uvV_inverse = uvV.to(srgb.space);
        println!("srgb {:?}", srgb.value);
        println!("uvV {:?}", uvV.value);
        println!("uvV_inverse {:?}", uvV_inverse.value);
    }
}
