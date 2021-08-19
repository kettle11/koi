#[cfg(all(target_os = "macos", not(feature = "SDL")))]
mod macos;
#[cfg(all(target_os = "macos", not(feature = "SDL")))]
pub use macos::*;
#[cfg(all(target_os = "macos", not(feature = "SDL")))]
#[macro_use]
extern crate objc;

#[cfg(all(target_arch = "wasm32", not(feature = "SDL")))]
mod web;
#[cfg(all(target_arch = "wasm32", not(feature = "SDL")))]
pub use web::*;
#[cfg(all(target_os = "windows", not(feature = "SDL")))]
mod windows;
#[cfg(all(target_os = "windows", not(feature = "SDL")))]
pub use windows::*;

#[cfg(feature = "SDL")]
mod sdl;
#[cfg(feature = "SDL")]
pub use sdl::*;
