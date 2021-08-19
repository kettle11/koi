//! Cross platform windows, input, and GL context creation for Windows, Mac, and Web.
//!
//! # Hello Window
//! ```no_run
//! use kapp::*;
//!
//! fn main() {
//!     // Initialize the Application and EventLoop
//!     let (app, event_loop) = initialize();
//!
//!     // Open a window
//!     let _window = app.new_window().build().unwrap();
//!
//!     // Run forever receiving system events.
//!     event_loop.run( move |event| match event {
//!          Event::WindowCloseRequested { .. } => app.quit(),
//!          Event::Draw { .. } => {
//!            // Render something here.
//!          }
//!          _ => println!("Received event: {:?}", event),
//!     });
//! }
//! ```
//!
//! # User Input
//! Events are provided for user input:
//!
//! [KeyDown][Event::KeyDown], [KeyUp][Event::KeyUp], [PointerMoved][Event::PointerMoved],
//! [PointerDown][Event::PointerDown], [PointerUp][Event::PointerUp], [Scroll][Event::Scroll]
//!
//! If an event responds with coordinates the coordinates are in physical device space
//! (the actual pixels of the device without a scale factor applied).
//! The origin (0,0) is the upper left corner of the screen or window.
//! ```no_run
//! use kapp::*;
//!
//! fn main() {
//!     let (mut app, event_loop) = initialize();
//!     let _window = app.new_window().build().unwrap();
//!
//!     event_loop.run( move |event| match event {
//!         Event::KeyDown { key, .. } => println!("Key pressed: {:?}", key),
//!         Event::KeyUp { key, .. } => println!("Key up: {:?}", key),
//!         Event::PointerMoved { x, y, .. } => println!("Pointer moved: {:?},{:?}", x, y),
//!         _ => {},
//!     });
//! }
//! ```
//!
//! # GL Rendering
//! If the `gl_context` feature is enabled then a GLContext can be created for rendering with GL.
//! See the `simple_gl.rs` example.
mod application;
mod async_application;
mod state_tracker;
mod window;
mod window_builder;

use kapp_platforms::prelude as platform;

#[cfg(feature = "gl_context")]
pub use kapp_gl_context::prelude::*;

pub use platform::{Cursor, Event, Key, PointerButton, PointerSource, WindowId};

pub use application::{initialize, Application, EventLoop, UserEventSender};

pub use async_application::*;

pub use state_tracker::StateTracker;
pub use window::Window;
pub use window_builder::WindowBuilder;
