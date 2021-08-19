pub use crate::{FilterMode, PixelFormat, WrappingMode};
pub use chlorine::c_uint;

pub const ACTIVE_UNIFORMS: c_uint = 0x8B86;
pub const ACTIVE_ATTRIBUTES: c_uint = 0x8B89;

pub const INT: c_uint = 0x1404;
pub const FLOAT: c_uint = 0x1406;
pub const UNSIGNED_SHORT: c_uint = 0x1403;
pub const UNSIGNED_INT: c_uint = 0x1405;
pub const UNSIGNED_BYTE: c_uint = 0x1401;

pub const FLOAT_VEC2: c_uint = 0x8B50;
pub const FLOAT_VEC3: c_uint = 0x8B51;
pub const FLOAT_VEC4: c_uint = 0x8B52;
pub const FLOAT_MAT4: c_uint = 0x8B5C;
pub const SAMPLER_2D: c_uint = 0x8B5E;

pub const LESS: c_uint = 0x0201;
pub const EQUAL: c_uint = 0x0202;
pub const LEQUAL: c_uint = 0x0203;
pub const GREATER: c_uint = 0x0204;
pub const NOTEQUAL: c_uint = 0x0205;
pub const GEQUAL: c_uint = 0x0206;
pub const ALWAYS: c_uint = 0x0207;

pub const FRONT: c_uint = 0x0404;
pub const BACK: c_uint = 0x0405;
pub const FRONT_AND_BACK: c_uint = 0x0408;

pub const ONE_MINUS_SRC_ALPHA: c_uint = 0x0303;
pub const SRC_ALPHA: c_uint = 0x0302;

pub const DEPTH_COMPONENT16: c_uint = 0x81A5;
pub const DEPTH_COMPONENT24: c_uint = 0x81A6;
pub const DEPTH_COMPONENT32F: c_uint = 0x8CAC;

pub const NEAREST: c_uint = 0x2600;
pub const LINEAR: c_uint = 0x2601;
pub const NEAREST_MIPMAP_NEAREST: c_uint = 0x2700;
pub const LINEAR_MIPMAP_NEAREST: c_uint = 0x2701;
pub const NEAREST_MIPMAP_LINEAR: c_uint = 0x2702;
pub const LINEAR_MIPMAP_LINEAR: c_uint = 0x2703;

pub const CLAMP_TO_EDGE: c_uint = 0x812F;
pub const MIRRORED_REPEAT: c_uint = 0x8370;
pub const REPEAT: c_uint = 0x2901;

pub const DEPTH_COMPONENT: c_uint = 0x1902;
pub const RED: c_uint = 0x1903;
pub const RG: c_uint = 0x8227;
pub const RGB: c_uint = 0x1907;
pub const RGBA: c_uint = 0x1908;

pub const R8: c_uint = 0x8229;
pub const RG8: c_uint = 0x822B;
pub const RGB8: c_uint = 0x8051;
pub const RGBA8: c_uint = 0x8058;
pub const SRGB8: c_uint = 0x8C41;
pub const SRGB8_ALPHA8: c_uint = 0x8C43;

pub const TEXTURE0: c_uint = 0x84C0;

#[inline]
fn srgb_to_linear(byte: u8) -> u8 {
    let u = byte as f64 / 255.0;
    if u <= 0.04045 {
        u / 12.92
    } else {
        f64::powf((u + 0.055) / 1.055, 2.4)
    };
    (u * 255.0) as u8
}

pub fn convert_srgb_data_to_linear_srgb(new_data: &mut Vec<u8>, data: &[u8], alpha: bool) {
    if alpha {
        for chunk in data.chunks(4) {
            new_data.push(srgb_to_linear(data[0]));
            new_data.push(srgb_to_linear(data[1]));
            new_data.push(srgb_to_linear(data[2]));
            new_data.push(data[3]);
        }
    } else {
        for d in data {
            new_data.push(srgb_to_linear(*d));
        }
    }
}

// Useful reference: https://webgl2fundamentals.org/webgl/lessons/webgl-data-textures.html
pub fn pixel_format_to_gl_format_and_inner_format_and_type(
    pixel_format: PixelFormat,
    srgb: bool,
) -> (c_uint, c_uint, c_uint) {
    let format = match pixel_format {
        PixelFormat::R8Unorm => RED,
        PixelFormat::RG8Unorm => RG,
        PixelFormat::RGB8Unorm => RGB,
        PixelFormat::RGBA8Unorm => RGBA,
        PixelFormat::Depth16 | PixelFormat::Depth24 | PixelFormat::Depth32F => DEPTH_COMPONENT,
    };

    let mut inner_format = match pixel_format {
        PixelFormat::Depth16 => DEPTH_COMPONENT16,
        PixelFormat::Depth24 => DEPTH_COMPONENT24,
        PixelFormat::Depth32F => DEPTH_COMPONENT32F,
        PixelFormat::R8Unorm => R8,
        PixelFormat::RG8Unorm => RG8,
        PixelFormat::RGB8Unorm => RGB8,
        PixelFormat::RGBA8Unorm => RGBA8,
    };

    // TODO: Maybe there should just be an additional set of sRGB formats like seems to have already
    // been started with [PixelFormat::RGBA8UnormSrgb].
    // Arguably the sRGB conversion is a "feature" so maybe it's not necessary.
    /*
    if srgb {
        inner_format = match inner_format {
            R8 | RG8 | RGB8 | RGBA8 => {
                SRGB8_ALPHA8
            },
            _ => inner_format,
        };
    }
    */

    let type_ = match pixel_format {
        PixelFormat::Depth16 => UNSIGNED_SHORT,
        PixelFormat::Depth24 => UNSIGNED_INT,
        PixelFormat::Depth32F => FLOAT,
        _ => UNSIGNED_BYTE,
    };

    (format, inner_format, type_)
}

pub fn minification_filter_to_gl_enum(
    minification_filter_mode: FilterMode,
    mipmap_filter_mode: FilterMode,
    has_mipmaps: bool,
) -> c_uint {
    if has_mipmaps {
        match (minification_filter_mode, mipmap_filter_mode) {
            (FilterMode::Nearest, FilterMode::Nearest) => NEAREST_MIPMAP_NEAREST,
            (FilterMode::Nearest, FilterMode::Linear) => NEAREST_MIPMAP_LINEAR,
            (FilterMode::Linear, FilterMode::Nearest) => LINEAR_MIPMAP_NEAREST,
            (FilterMode::Linear, FilterMode::Linear) => LINEAR_MIPMAP_LINEAR,
        }
    } else {
        match (minification_filter_mode) {
            FilterMode::Nearest => NEAREST,
            FilterMode::Linear => LINEAR,
        }
    }
}

pub fn magnification_filter_to_gl_enum(filter_mode: FilterMode) -> c_uint {
    match filter_mode {
        FilterMode::Nearest => NEAREST,
        FilterMode::Linear => LINEAR,
    }
}

pub fn wrapping_to_gl_enum(wrapping_mode: WrappingMode) -> c_uint {
    match wrapping_mode {
        WrappingMode::ClampToEdge => CLAMP_TO_EDGE,
        WrappingMode::MirrorRepeat => MIRRORED_REPEAT,
        WrappingMode::Repeat => REPEAT,
    }
}
