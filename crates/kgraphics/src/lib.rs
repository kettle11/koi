#[cfg(feature = "metal")]
#[allow(non_upper_case_globals)]
mod metal;
#[cfg(feature = "metal")]
pub use metal::*;

#[cfg(all(not(target_arch = "wasm32"), feature = "gl"))]
mod gl;
#[cfg(all(not(target_arch = "wasm32"), feature = "gl"))]
pub use gl::*;

#[cfg(all(target_arch = "wasm32", feature = "gl"))]
mod webgl_backend;
#[cfg(all(target_arch = "wasm32", feature = "gl"))]
pub use webgl_backend::*;

#[cfg(feature = "gl")]
#[allow(unused)]
mod gl_shared;
#[cfg(feature = "gl")]
#[allow(unused)]
use gl_shared::*;

#[cfg(feature = "do_nothing_backend")]
mod do_nothing_backend;
#[cfg(feature = "do_nothing_backend")]
pub use do_nothing_backend::*;

mod graphics_backend_trait;
pub use graphics_backend_trait::*;

mod pipeline_builder;
pub use pipeline_builder::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PixelFormat {
    R8Unorm,
    RG8Unorm,
    RGB8Unorm,
    RGBA8Unorm,
    Depth16,
    Depth24,
    Depth32F,
}

#[derive(Clone, Copy, Debug)]
pub enum FacesToRender {
    Front,
    Back,
    FrontAndBack,
    None,
}

#[derive(Clone, Copy, Debug)]
/// Specifies if a pixel will be rendered based on the z-buffer value.
pub enum DepthTest {
    /// Effectively disables depth testing.
    AlwaysPass,
    /// Write the pixel if its z value is less than the current value.
    Less,
    /// Write the pixel if its z value is greater than the current value.
    Greater,
    /// Write the pixel if its z value is less than or equal to the current value.
    LessOrEqual,
    /// Write the pixel if its z value is greater than or equal to the current value.s
    GreaterOrEqual,
}

/// This should be expanded
#[derive(Clone, Copy, Debug)]
pub enum BlendFactor {
    /// source_pixel.alpha
    SourceAlpha,
    /// 1.0 - source_pixel.alpha
    OneMinusSourceAlpha,
}

pub struct GraphicsContextSettings {
    /// If possible, should a high resolution framebuffer be requested?
    pub high_resolution_framebuffer: bool,
    /// How many MSAA samples should be requested for the framebuffer?
    pub samples: u8,
}

impl Default for GraphicsContextSettings {
    fn default() -> Self {
        Self {
            high_resolution_framebuffer: true,
            samples: 2,
        }
    }
}

#[derive(Copy, Clone)]
pub enum FilterMode {
    Nearest,
    Linear,
}

#[derive(Copy, Clone)]
pub enum WrappingMode {
    ClampToEdge,
    Repeat,
    MirrorRepeat,
}

#[derive(Copy, Clone)]
pub struct TextureSettings {
    pub srgb: bool,
    pub minification_filter: FilterMode,
    pub magnification_filter: FilterMode,
    /// How this texture is sampled between mipmaps.
    /// Defaults fo [FilterMode::Linear]
    pub mipmap_filter: FilterMode,
    pub generate_mipmaps: bool,
    pub wrapping_horizontal: WrappingMode,
    pub wrapping_vertical: WrappingMode,
    pub border_color: (f32, f32, f32, f32),
}

impl Default for TextureSettings {
    fn default() -> Self {
        Self {
            srgb: true,
            minification_filter: FilterMode::Nearest,
            magnification_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            generate_mipmaps: true,
            wrapping_horizontal: WrappingMode::Repeat,
            wrapping_vertical: WrappingMode::Repeat,
            border_color: (0., 0., 0., 1.),
        }
    }
}
