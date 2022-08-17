mod common;

#[cfg(all(target_os = "macos", not(feature = "SDL")))]
mod macos;

#[cfg(all(target_os = "macos", not(feature = "SDL")))]
pub use macos::GLContext;

#[cfg(all(target_os = "windows", not(feature = "SDL")))]
mod windows;

#[cfg(all(target_os = "windows", not(feature = "SDL")))]
pub use windows::GLContext;

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::GLContext;

#[cfg(feature = "SDL")]
pub mod sdl;

#[cfg(feature = "SDL")]
pub use sdl::GLContext;

pub mod prelude {
    pub use super::common::{ColorSpace, GLContextBuilder, GLContextTrait, SetWindowError, VSync};
    pub use super::GLContext;
}

#[cfg(target_os = "macos")]
pub(crate) fn occluded_window_vsync_hack(
    vsync: common::VSync,
    ns_window: Option<*mut objc::runtime::Object>,
) {
    #[repr(u64)]
    enum NSWindowOcclusionState {
        NSWindowOcclusionStateVisible = 1 << 1,
    }
    use objc::*;

    // Simulate VSync by sleeping for 16ms (60 fps) if a window is occluded and VSync is enabled.
    match vsync {
        common::VSync::On | common::VSync::Adaptive => {
            if let Some(ns_window) = ns_window {
                let occlusion_state: u64 = unsafe { msg_send![ns_window, occlusionState] };
                if occlusion_state & NSWindowOcclusionState::NSWindowOcclusionStateVisible as u64
                    == 0
                {
                    std::thread::sleep(std::time::Duration::from_millis(16));
                }
            }
        }
        _ => {}
    }
}
