/// This crate provides structures and traits shared between the platform backends.
/// Each platform must provide the following:
/// * Structures and implementations for the traits in platform_traits.rs
/// * Means to detect and trigger all events in events.rs
///   Each event has documented behavior that must be conformed to.
///   event_receiver should be used on platforms where calls to a platform
///   functions can trigger events.
mod cursors;
pub mod event_receiver;
mod events;
mod keys;
mod platform_traits;
pub mod redraw_manager;
mod screen_id;
mod window_id;
mod window_parameters;

pub use cursors::Cursor;
pub use events::{Event, PointerButton, PointerSource};
pub use keys::Key;
pub use platform_traits::{PlatformApplicationTrait, PlatformEventLoopTrait, PlatformUserEventSenderTrait};
pub use raw_window_handle;
pub use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
pub use screen_id::ScreenId;
pub use window_id::{RawWindowHandleTrait, WindowId};
pub use window_parameters::WindowParameters;
