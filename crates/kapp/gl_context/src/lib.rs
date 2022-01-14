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
    pub use super::common::{GLContextBuilder, GLContextTrait, SetWindowError, VSync};
    pub use super::GLContext;
}
