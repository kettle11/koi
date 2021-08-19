use crate::platform::*;
use crate::{Application, Window};

pub struct WindowBuilder<'a> {
    application: &'a Application,
    window_parameters: WindowParameters,
}

impl<'a> WindowBuilder<'a> {
    pub fn new(application: &'a Application) -> Self {
        Self {
            application,
            window_parameters: WindowParameters {
                position: None,
                size: Some((500, 500)),
                minimum_size: None,
                maximum_size: None,
                resizable: true,
                without_titlebar: false,
                title: "Untitled".to_string(),
            },
        }
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.window_parameters.title = title.to_string();
        self
    }

    /// Specify if the window should be resizably by dragging the corner.
    pub fn resizable(&mut self, resizable: bool) -> &mut Self {
        self.window_parameters.resizable = resizable;
        self
    }

    /// Specifies the lower left corner of the window.
    pub fn position(&mut self, x: u32, y: u32) -> &mut Self {
        self.window_parameters.position = Some((x, y));
        self
    }

    /// Sets the size of the window's content area (excluding the titlebar and borders)
    /// Specified with physical coordinates
    pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_parameters.size = Some((width, height));
        self
    }

    /// Sets the minimum size of the window's content area (excluding the titlebar and borders)
    pub fn minimum_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_parameters.minimum_size = Some((width, height));
        self
    }

    /// Sets the maximum size of the window's content area (excluding the titlebar and borders)
    pub fn maximum_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_parameters.maximum_size = Some((width, height));
        self
    }

    /// Only available on MacOS for now.
    /// This feature isn't well tested and may result in sizing errors.
    pub fn without_titlebar(&mut self) -> &mut Self {
        self.window_parameters.without_titlebar = true;
        self
    }

    pub fn build(&mut self) -> Result<Window, ()> {
        // Clamp the window size to the minimum width and height
        if let Some(size) = &mut self.window_parameters.size {
            if let Some((min_width, min_height)) = self.window_parameters.minimum_size {
                *size = (size.0.max(min_width), size.1.max(min_height))
            }
        }

        Ok(Window::new(
            self.application
                .platform_application
                .borrow_mut()
                .new_window(&self.window_parameters),
            self.application.platform_application.clone(),
        ))
    }
}
