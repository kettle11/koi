use kapp::*;
use kudo::*;

#[derive(NotCloneComponent)]
pub struct Input {
    pub(crate) state: kapp::StateTracker,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            state: kapp::StateTracker::new(),
        }
    }

    /// Returns true if the key is held down.
    pub fn key(&self, key: Key) -> bool {
        self.state.key(key)
    }

    /// Returns true if the key has been pressed since the last frame.
    pub fn key_just_pressed(&self, key: Key) -> bool {
        self.state.key_down(key)
    }

    /// Returns true if the PointerButton is currently held.
    pub fn pointer_button(&self, button: PointerButton) -> bool {
        self.state.pointer_button(button)
    }

    /// Returns true if the pointer button has been pressed since the last frame.
    pub fn pointer_just_pressed(&self, button: PointerButton) -> bool {
        self.state.pointer_button_down(button)
    }

    /// Returns true if the pointer button has been pressed since the last frame.
    pub fn pointer_just_released(&self, button: PointerButton) -> bool {
        self.state.pointer_button_released(button)
    }

    /// Gets the last updated position of the pointer.
    /// The pointer may be a mouse or a touch.
    /// Probably this should return a Vec2 instead.
    pub fn pointer_position(&self) -> (f64, f64) {
        self.state.pointer_position()
    }

    /// How the mouse position has changed since the last frame
    pub fn mouse_motion(&self) -> (f64, f64) {
        self.state.mouse_motion()
    }

    pub fn all_events_since_last_frame(&self) -> &[kapp::Event] {
        self.state.all_events_since_last_frame()
    }
}
