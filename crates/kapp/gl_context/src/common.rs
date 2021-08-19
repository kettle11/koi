pub struct GLContextAttributes {
    pub major_version: u8,
    pub minor_version: u8,
    pub color_bits: u8,
    pub alpha_bits: u8,
    pub depth_bits: u8,
    pub stencil_bits: u8,
    pub srgb: bool,
    /// msaa_samples hould be a multiple of 2
    pub msaa_samples: u8,
    /// WebGL version is only relevant for web.
    pub webgl_version: WebGLVersion,
    /// Mac specific, should the framebuffer be allocated with a higher resolution.
    pub high_resolution_framebuffer: bool,
}

pub enum WebGLVersion {
    One,
    Two,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VSync {
    ///
    On,
    Off,
    Adaptive,
    /// Other will indicate how many frames to wait before displaying.
    /// For example, Other(2) would render at half the display framerate.
    Other(i32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SetWindowError {
    /// The pixel format of the window does not match the context's
    MismatchedPixelFormat,
}

pub trait GLContextTrait {
    /// Gets the pixel format and attributes of the context.
    fn get_attributes(&self) -> GLContextAttributes;

    /// Makes the GLContext current to the current thread
    fn make_current(&mut self) -> Result<(), std::io::Error>;

    /// Sets the Vsync for the window attached to this context.
    /// Returns a system error if not successful
    fn set_vsync(&mut self, vsync: VSync) -> Result<(), std::io::Error>;
    fn get_vsync(&self) -> VSync;

    /// Assigns a window to draw to
    fn set_window(
        &mut self,
        window: Option<&impl raw_window_handle::HasRawWindowHandle>,
    ) -> Result<(), SetWindowError>;

    /// Resizes the context to match the attached window
    fn resize(&mut self);

    /// Swaps the backbuffer and frontbuffer for the currently bound window.
    fn swap_buffers(&mut self);

    /// Gets the address of a GL process.
    /// Used by GL loaders
    fn get_proc_address(&self, address: &str) -> *const core::ffi::c_void;
}

pub struct GLContextBuilder {
    pub(crate) gl_attributes: GLContextAttributes,
}

impl GLContextBuilder {
    pub fn samples(&mut self, samples: u8) -> &mut Self {
        self.gl_attributes.msaa_samples = samples;
        self
    }

    /// Sets the major version.
    /// On MacOS only versions 4.1 and 3.2 are supported and the closest will be chosen.
    pub fn major_version(&mut self, version: u8) -> &mut Self {
        self.gl_attributes.major_version = version;
        self
    }

    /// Sets the minor version.
    /// This has no no effect on MacOS.
    pub fn minor_version(&mut self, version: u8) -> &mut Self {
        self.gl_attributes.minor_version = version;
        self
    }

    /// Sets if the context should use the sRGB color space.
    /// This has no effect on Web.
    pub fn srgb(&mut self, srgb: bool) -> &mut Self {
        self.gl_attributes.srgb = srgb;
        self
    }

    /// Sets if a high resolution window framebuffer should be requested (if possible).
    /// This is presently only relevant on retina Macs which can select a
    /// high resolution framebuffer or a smaller one for performance reasons.
    pub fn high_resolution_framebuffer(&mut self, value: bool) -> &mut Self {
        self.gl_attributes.high_resolution_framebuffer = value;
        self
    }
}
