//! An implementation of std::time::Instant that also works on Wasm-web backends.

#[cfg(target_arch = "wasm32")]
pub use kwasm::libraries::Instant;
#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;
