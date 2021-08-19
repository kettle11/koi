use crate::common::*;
use objc::runtime::{Object, YES};
use objc::*;
use std::ffi::c_void;
use std::io::Error;

pub struct GLContext {
    gl_context: *mut Object,
    pixel_format: *mut Object,
    vsync: VSync,
    high_dpi_framebuffer: bool,
    srgb: bool,
    ns_window: Option<*mut Object>,
}

// This isn't really true because make_current must be called after GLContext is passed to another thread.
// A solution would be to wrap this is an object to send to another thread, and the unwrap
// calls make_current.
unsafe impl Send for GLContext {}

impl GLContextBuilder {
    pub fn build(&self) -> Result<GLContext, ()> {
        let profile_version = if self.gl_attributes.major_version > 4 {
            NSOpenGLProfileVersion4_1Core
        } else {
            NSOpenGLProfileVersion3_2Core
        };
        unsafe {
            let attrs = [
                NSOpenGLPFAOpenGLProfile as u32,
                profile_version as u32,
                // NSOpenGLPFAClosestPolicy as u32,
                NSOpenGLPFAAccelerated as u32,
                NSOpenGLPFAColorSize as u32,
                self.gl_attributes.color_bits as u32,
                NSOpenGLPFAAlphaSize as u32,
                self.gl_attributes.alpha_bits as u32,
                NSOpenGLPFADepthSize as u32,
                self.gl_attributes.depth_bits as u32,
                NSOpenGLPFAStencilSize as u32,
                self.gl_attributes.stencil_bits as u32,
                NSOpenGLPFADoubleBuffer as u32,
                NSOpenGLPFASampleBuffers as u32,
                1,
                NSOpenGLPFASamples as u32,
                self.gl_attributes.msaa_samples as u32,
                0,
            ];

            // This allocation is dropped when GLContext is dropped
            let pixel_format: *mut Object = msg_send![class!(NSOpenGLPixelFormat), alloc];
            let pixel_format: *mut Object = msg_send![pixel_format, initWithAttributes: &attrs];

            // This allocation is dropped when GLContext is dropped
            let gl_context: *mut Object = msg_send![class!(NSOpenGLContext), alloc];
            let gl_context: *mut Object =
                msg_send![gl_context, initWithFormat: pixel_format shareContext: nil];
            let () = msg_send![gl_context, makeCurrentContext];

            Ok(GLContext {
                gl_context,
                pixel_format,
                vsync: VSync::On, // Enable VSync for the next window bound
                // current_window: None,
                high_dpi_framebuffer: self.gl_attributes.high_resolution_framebuffer,
                srgb: self.gl_attributes.srgb,
                ns_window: None,
            })
        }
    }
}

impl GLContext {
    pub fn new() -> GLContextBuilder {
        GLContextBuilder {
            gl_attributes: GLContextAttributes {
                major_version: 3,
                minor_version: 3,
                msaa_samples: 1,
                color_bits: 24,
                alpha_bits: 8,
                depth_bits: 24,
                stencil_bits: 8,
                srgb: true,
                webgl_version: WebGLVersion::None,
                high_resolution_framebuffer: false,
            },
        }
    }
}

impl GLContextTrait for GLContext {
    fn set_window(
        &mut self,
        window: Option<&impl raw_window_handle::HasRawWindowHandle>,
    ) -> Result<(), SetWindowError> {
        // This does not currently handle the case where a window is closed
        // but this context remains.
        use raw_window_handle::*;

        let window_and_view = window.map(|w| match w.raw_window_handle() {
            RawWindowHandle::MacOS(handle) => (
                handle.ns_window as *mut Object,
                handle.ns_view as *mut Object,
            ),
            _ => unreachable!(),
        });

        if let Some((ns_window, _)) = window_and_view {
            // Lookup the window's view ourself in case it's not provided,
            // as is the case with SDL backend.
            let window_view: *mut Object = unsafe { msg_send![ns_window, contentView] };

            let () = unsafe {
                if self.srgb {
                    // There is a subtle bug here that will be left unaddressed for now.
                    // If a window is bound to this GL Context the window's color space will be set to sRGB.
                    // If the window is then bound to a non-sRGB context its color space will not reset.
                    // To set the color space properly in that scenario requires more research,
                    // and this code covers the vast majority of typical cases for now.
                    let srgb_color_space: *mut Object =
                        msg_send![class!(NSColorSpace), sRGBColorSpace];
                    let () = msg_send![ns_window, setColorSpace: srgb_color_space];
                }

                // Apparently this defaults to YES even without this call
                let () = msg_send![window_view, setWantsBestResolutionOpenGLSurface: self.high_dpi_framebuffer];
                msg_send![self.gl_context, performSelectorOnMainThread:sel!(setView:) withObject:window_view waitUntilDone:YES]
            };

            self.set_vsync(self.vsync).unwrap();
            self.ns_window = Some(ns_window);
        } else {
            let () = unsafe { msg_send![self.gl_context, clearDrawable] };
            self.ns_window = None;
        }

        Ok(())
    }

    fn get_attributes(&self) -> GLContextAttributes {
        todo!()
    }

    fn set_vsync(&mut self, vsync: VSync) -> Result<(), std::io::Error> {
        let result = match vsync {
            VSync::On => {
                let () = unsafe {
                    msg_send![self.gl_context, setValues:&(1 as i32) forParameter:NSOpenGLCPSwapInterval]
                };
                Ok(())
            }
            VSync::Off => {
                let () = unsafe {
                    msg_send![self.gl_context, setValues:&(0 as i32) forParameter:NSOpenGLCPSwapInterval]
                };
                Ok(())
            }
            VSync::Adaptive => {
                Ok(()) // Unsupported, should throw an error
            }
            VSync::Other(..) => {
                Ok(()) // Unsupported, should throw an error
            }
        };

        if result.is_ok() {
            self.vsync = vsync;
        }
        result
    }

    fn get_vsync(&self) -> VSync {
        let mut i: i64 = 0;
        let () = unsafe {
            msg_send![self.gl_context, getValues:&mut i forParameter:NSOpenGLCPSwapInterval]
        };
        match i {
            0 => VSync::Off,
            1 => VSync::On,
            _ => VSync::Other(i as i32),
        }
    }

    fn make_current(&mut self) -> Result<(), Error> {
        unsafe {
            let () = msg_send![self.gl_context, makeCurrentContext];
        }
        Ok(())
    }

    fn resize(&mut self) {
        let update = sel!(update);
        unsafe {
            let () = msg_send![self.gl_context, performSelectorOnMainThread:update withObject:nil waitUntilDone:YES];
        }
    }

    // https://developer.apple.com/documentation/appkit/nsopenglcontext/1436211-flushbuffer?language=objc
    fn swap_buffers(&mut self) {
        unsafe {
            // Simulate VSync by sleeping for 16ms (60 fps) if a window is occluded and VSync is enabled.
            match self.vsync {
                VSync::On | VSync::Adaptive => {
                    if let Some(ns_window) = self.ns_window {
                        let occlusion_state: u64 = msg_send![ns_window, occlusionState];
                        if occlusion_state
                            & NSWindowOcclusionState::NSWindowOcclusionStateVisible as u64
                            == 0
                        {
                            std::thread::sleep(std::time::Duration::from_millis(16));
                        }
                    }
                }
                _ => {}
            }
            let () = msg_send![self.gl_context, flushBuffer];
        }
    }

    fn get_proc_address(&self, addr: &str) -> *const core::ffi::c_void {
        let symbol_name = NSString::new(addr);
        let framework_name = NSString::new("com.apple.opengl");
        let framework = unsafe { CFBundleGetBundleWithIdentifier(framework_name.raw) };
        let symbol = unsafe { CFBundleGetFunctionPointerForName(framework, symbol_name.raw) };
        symbol as *const _
    }
}

impl Drop for GLContext {
    fn drop(&mut self) {
        unsafe {
            let () = msg_send![self.gl_context, release];
            let () = msg_send![self.pixel_format, release];
        }
    }
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLContextParameter {
    NSOpenGLCPSwapInterval = 222,
}

#[repr(u64)]
pub enum NSWindowOcclusionState {
    NSWindowOcclusionStateVisible = 1 << 1,
}

use NSOpenGLContextParameter::*;

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPixelFormatAttribute {
    NSOpenGLPFADoubleBuffer = 5,
    NSOpenGLPFAColorSize = 8,

    NSOpenGLPFAAlphaSize = 11,
    NSOpenGLPFADepthSize = 12,
    NSOpenGLPFAStencilSize = 13,
    NSOpenGLPFASampleBuffers = 55,
    NSOpenGLPFASamples = 56,
    NSOpenGLPFAAccelerated = 73,
    NSOpenGLPFAOpenGLProfile = 99,
}
pub use NSOpenGLPixelFormatAttribute::*;

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPFAOpenGLProfiles {
    NSOpenGLProfileVersion3_2Core = 0x3200,
    NSOpenGLProfileVersion4_1Core = 0x4100,
}
pub use NSOpenGLPFAOpenGLProfiles::*;
pub struct NSString {
    pub raw: *mut Object,
}

impl NSString {
    pub fn new(string: &str) -> Self {
        unsafe {
            let raw: *mut Object = msg_send![class!(NSString), alloc];
            let raw: *mut Object = msg_send![
                raw,
                initWithBytes: string.as_ptr()
                length: string.len()
                encoding:UTF8_ENCODING as *mut Object
            ];

            Self { raw }
        }
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        unsafe {
            let () = msg_send![self.raw, release];
        }
    }
}

#[allow(non_upper_case_globals)]
pub const nil: *mut Object = 0 as *mut Object;

#[repr(C)]
pub struct __CFBundle(c_void);
pub type CFBundleRef = *mut __CFBundle;

extern "C" {
    pub fn CFBundleGetBundleWithIdentifier(bundleID: CFStringRef) -> CFBundleRef;
    pub fn CFBundleGetFunctionPointerForName(
        bundle: CFBundleRef,
        function_name: CFStringRef,
    ) -> *const c_void;
}

pub const UTF8_ENCODING: usize = 4;
pub type CFStringRef = *const Object; // CFString
