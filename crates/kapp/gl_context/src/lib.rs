mod common;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::GLContext;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::GLContext;

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::GLContext;

pub mod prelude {
    pub use super::common::{GLContextBuilder, GLContextTrait, SetWindowError, VSync};
    pub use super::GLContext;
}
