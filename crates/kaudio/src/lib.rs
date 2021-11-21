#[cfg(target_os = "windows")]
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "macos")]
#[allow(non_upper_case_globals, non_snake_case)]
mod core_audio;
#[cfg(target_os = "macos")]
pub use core_audio::*;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::*;

mod sound;
pub use sound::*;

#[cfg(feature = "wav")]
mod wav;
#[cfg(feature = "wav")]
pub use wav::*;

pub struct StreamInfo {
    sample_rate: u32,
    channels: u32,
}

impl StreamInfo {
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channels(&self) -> u32 {
        self.channels
    }
}
