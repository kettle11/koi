use crate::platform::*;
use std::cell::RefCell;
use std::rc::Rc;

/// A handle used to control a Window.
/// The window is closed when the Window instance is dropped.
#[derive(Clone)]
pub struct Window(Rc<WindowInner>);

struct WindowInner {
    id: WindowId,
    platform_application: Rc<RefCell<PlatformApplication>>,
}

impl Window {
    pub(crate) fn new(
        id: WindowId,
        platform_application: Rc<RefCell<PlatformApplication>>,
    ) -> Self {
        Self(Rc::new(WindowInner {
            id,
            platform_application,
        }))
    }

    pub fn minimize(&self) {
        self.0
            .platform_application
            .borrow_mut()
            .minimize_window(self.0.id);
    }

    pub fn maximize(&self) {
        self.0
            .platform_application
            .borrow_mut()
            .maximize_window(self.0.id);
    }

    /// Returns the window from a minimized, maximized, or fullscreened state.
    pub fn restore(&self) {
        self.0
            .platform_application
            .borrow_mut()
            .restore_window(self.0.id);
    }

    /// On Web this must be done in response to a user event.
    pub fn fullscreen(&self) {
        self.0
            .platform_application
            .borrow_mut()
            .fullscreen_window(self.0.id);
    }

    /// Sets the title displayed at the top of the window
    pub fn set_title(&mut self, title: &str) {
        self.0
            .platform_application
            .borrow_mut()
            .set_window_title(self.0.id, title);
    }

    /// Set the lower left corner of the window.
    pub fn set_position(&mut self, x: u32, y: u32) {
        self.0
            .platform_application
            .borrow_mut()
            .set_window_position(self.0.id, x, y);
    }

    /// Set the window's width and height, excluding the titlebar
    /// Width and height are specified with physical coordinates.
    pub fn set_size(&self, width: u32, height: u32) {
        self.0
            .platform_application
            .borrow_mut()
            .set_window_size(self.0.id, width, height);
    }

    /// Lets the OS know where it should place text input related popups like
    /// accent character selection.
    /// Position is specified relative to the window's upper left corner.
    /// ONLY SUPPORTED ON MAC (for now)
    pub fn set_text_input_rectangle(&self, x: f64, y: f64, width: f64, height: f64) {
        self.0
            .platform_application
            .borrow_mut()
            .set_text_input_rectangle(self.0.id, x, y, width, height)
    }

    /// Get the window's width and height excluding the titlebar.
    /// Unimplemented on Web.
    pub fn size(&self) -> (u32, u32) {
        self.0
            .platform_application
            .borrow_mut()
            .get_window_size(self.0.id)
    }

    /// Get the scale factor the window should apply to UI.
    pub fn scale(&self) -> f64 {
        self.0
            .platform_application
            .borrow_mut()
            .get_window_scale(self.0.id)
    }

    /// Requests that this window receive another `Draw` event.
    /// Extra redraw requests will be ignored.
    pub fn request_redraw(&self) {
        self.0
            .platform_application
            .borrow_mut()
            .redraw_window(self.0.id);
    }

    pub fn window_id(&self) -> WindowId {
        self.0.id
    }
}

impl Drop for WindowInner {
    fn drop(&mut self) {
        self.platform_application.borrow_mut().close_window(self.id);
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.0
            .platform_application
            .borrow_mut()
            .raw_window_handle(self.0.id)
    }
}
