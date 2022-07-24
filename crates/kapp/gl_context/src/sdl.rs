use kapp_platform_common::WindowId;

use crate::common::*;

pub struct GLContext {
    set_to_hidden_window: bool,
    current_window: *mut fermium::video::SDL_Window,
    sdl_gl_context: fermium::video::SDL_GLContext,
}

impl GLContext {
    pub fn builder() -> GLContextBuilder {
        GLContextBuilder {
            gl_attributes: GLContextAttributes {
                major_version: 4,
                minor_version: 1,
                msaa_samples: 1,
                color_bits: 24,
                alpha_bits: 8,
                depth_bits: 24,
                stencil_bits: 8,
                webgl_version: WebGLVersion::One,
                high_resolution_framebuffer: false,
                srgb: true,
            },
        }
    }
}
impl GLContextBuilder {
    pub fn build(&self) -> Result<GLContext, ()> {
        unsafe {
            fermium::video::SDL_GL_SetAttribute(
                fermium::video::SDL_GL_CONTEXT_MAJOR_VERSION,
                self.gl_attributes.major_version as i32,
            );
            fermium::video::SDL_GL_SetAttribute(
                fermium::video::SDL_GL_CONTEXT_MINOR_VERSION,
                self.gl_attributes.minor_version as i32,
            );
            fermium::video::SDL_GL_SetAttribute(
                fermium::video::SDL_GL_RED_SIZE,
                (self.gl_attributes.color_bits / 3) as i32,
            );
            fermium::video::SDL_GL_SetAttribute(
                fermium::video::SDL_GL_GREEN_SIZE,
                (self.gl_attributes.color_bits / 3) as i32,
            );
            fermium::video::SDL_GL_SetAttribute(
                fermium::video::SDL_GL_BLUE_SIZE,
                (self.gl_attributes.color_bits / 3) as i32,
            );
            fermium::video::SDL_GL_SetAttribute(
                fermium::video::SDL_GL_ALPHA_SIZE,
                self.gl_attributes.alpha_bits as i32,
            );
            fermium::video::SDL_GL_SetAttribute(
                fermium::video::SDL_GL_DEPTH_SIZE,
                self.gl_attributes.depth_bits as i32,
            );
            fermium::video::SDL_GL_SetAttribute(
                fermium::video::SDL_GL_STENCIL_SIZE,
                self.gl_attributes.stencil_bits as i32,
            );
            if self.gl_attributes.msaa_samples > 0 {
                fermium::video::SDL_GL_SetAttribute(fermium::video::SDL_GL_MULTISAMPLEBUFFERS, 1);
                fermium::video::SDL_GL_SetAttribute(
                    fermium::video::SDL_GL_MULTISAMPLESAMPLES,
                    self.gl_attributes.msaa_samples as i32,
                );
            }
            fermium::video::SDL_GL_SetAttribute(
                fermium::video::SDL_GL_FRAMEBUFFER_SRGB_CAPABLE,
                if self.gl_attributes.srgb { 1 } else { 0 },
            );

            fermium::video::SDL_GL_SetAttribute(
                fermium::video::SDL_GL_CONTEXT_PROFILE_MASK,
                fermium::video::SDL_GL_CONTEXT_PROFILE_CORE.0 as i32,
            );

            // Create a hidden window to initialize SDL's GLContext.
            let current_window = fermium::video::SDL_CreateWindow(
                "".as_ptr() as *const i8,
                fermium::video::SDL_WINDOWPOS_CENTERED,
                fermium::video::SDL_WINDOWPOS_CENTERED,
                1,
                1,
                (fermium::video::SDL_WINDOW_HIDDEN | fermium::video::SDL_WINDOW_OPENGL).0,
            );
            let sdl_gl_context = fermium::video::SDL_GL_CreateContext(current_window);

            Ok(GLContext {
                set_to_hidden_window: true,
                current_window,
                sdl_gl_context,
            })
        }
    }
}

impl GLContextTrait for GLContext {
    fn get_attributes(&self) -> GLContextAttributes {
        unimplemented!()
    }

    fn set_vsync(&mut self, vsync: VSync) -> Result<(), std::io::Error> {
        unsafe {
            let result = fermium::video::SDL_GL_SetSwapInterval(match vsync {
                VSync::Off => 0,
                VSync::On => 1,
                VSync::Adaptive => -1,
                VSync::Other(_) => 1,
            });
            if result == 0 {
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "VSync setting is unsupported",
                ))
            }
        }
    }

    fn get_vsync(&self) -> VSync {
        unsafe {
            match fermium::video::SDL_GL_GetSwapInterval() {
                0 => VSync::Off,
                1 => VSync::On,
                -1 => VSync::Adaptive,
                _ => unreachable!(),
            }
        }
    }

    fn resize(&mut self) {
        // No need on the SDL backend?
    }

    fn get_proc_address(&self, address: &str) -> *const core::ffi::c_void {
        unsafe { fermium::video::SDL_GL_GetProcAddress(address.as_ptr() as *const i8) }
    }

    fn set_window(
        &mut self,
        _window: Option<&impl raw_window_handle::HasRawWindowHandle>,
    ) -> Result<(), SetWindowError> {
        panic!("`set_window` is unsupported on the SDL backend");
    }

    fn make_current(&mut self) -> Result<(), std::io::Error> {
        self.set_window_with_window_id(WindowId::new(self.current_window as *mut _))
    }

    fn swap_buffers(&mut self) {
        #[cfg(target_os = "macos")]
        {
            let mut info = fermium::syswm::SDL_SysWMinfo::default();
            unsafe { fermium::syswm::SDL_GetWindowWMInfo(self.current_window, &mut info) };
            crate::occluded_window_vsync_hack(
                self.get_vsync(),
                Some(unsafe { info.info.cocoa.window as *mut _ }),
            );
        }
        unsafe { fermium::video::SDL_GL_SwapWindow(self.current_window) }
    }
}

// This is inelegant but the `raw_window_handle` crate doesn't have a `custom` or `SDL` backend we
// can use to pass the SDL window. So for now we just make a custom SDL function for setting the window.
impl GLContext {
    pub fn set_window_with_window_id(&mut self, window_id: WindowId) -> Result<(), std::io::Error> {
        unsafe {
            let window = window_id.raw() as *mut fermium::video::SDL_Window;
            let result = fermium::video::SDL_GL_MakeCurrent(window, self.sdl_gl_context);
            if result == 0 {
                if self.set_to_hidden_window {
                    self.set_to_hidden_window = false;
                    fermium::video::SDL_DestroyWindow(self.current_window);
                }
                self.current_window = window;
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "SDL error"))
            }
        }
    }
}
