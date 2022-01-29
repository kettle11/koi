use super::XRBackendTrait;

/// An XR backend that does nothing.
pub struct DoNothingXrBackend;

impl DoNothingXrBackend {
    pub fn initialize() -> Result<Self, ()> {
        Ok(Self)
    }
}

impl XRBackendTrait for DoNothingXrBackend {
    fn start(&mut self) {
        // Do nothing!
    }
    fn stop(&mut self) {
        // Also do nothing
    }
    fn running(&self) -> bool {
        // Of course we aren't!
        false
    }
    fn button_state(&self, _controller_index: usize, _button_index: usize) -> bool {
        // Definitely no XR button pressed
        false
    }
    fn button_just_pressed(&self, _controller_index: usize, _button_index: usize) -> bool {
        // This didn't happen either
        false
    }
    fn button_just_released(&self, _controller_index: usize, _button_index: usize) -> bool {
        // There's nothing to release!
        false
    }
}
