use crate::{Event, Key, PointerButton};
use std::collections::HashMap;
use std::time::Duration;

// In the future this could be extended to track:
// * Window positions and status.
// * Window scale factors
// * Window color spaces

/// Tracks key and pointer input state based on events.
pub struct StateTracker {
    all_events_since_last_frame: Vec<Event>,
    keys_down_since_last_frame: HashMap<Key, Duration>, // Key was pressed since the last clear for any window.
    keys_pressed: HashMap<Key, Duration>,
    pointer_buttons_down_since_last_frame: HashMap<PointerButton, Duration>, // pointer was pressed since the last clear for any window.
    pointer_buttons_released_since_last_frame: HashMap<PointerButton, Duration>, // pointer was pressed since the last clear for any window.
    pointer_buttons_pressed: HashMap<PointerButton, Duration>,
    pointer_position: (f64, f64),
    mouse_motion: (f64, f64),
}

impl StateTracker {
    pub fn new() -> Self {
        Self {
            all_events_since_last_frame: Vec::new(),
            keys_down_since_last_frame: HashMap::with_capacity(256), // Arbitrary numbers to avoid resize
            keys_pressed: HashMap::with_capacity(256),
            pointer_buttons_down_since_last_frame: HashMap::with_capacity(16),
            pointer_buttons_released_since_last_frame: HashMap::with_capacity(16),
            pointer_buttons_pressed: HashMap::with_capacity(16),
            pointer_position: (0., 0.),
            mouse_motion: (0., 0.),
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown { key, timestamp } => {
                self.keys_pressed.insert(*key, *timestamp);
                self.keys_down_since_last_frame.insert(*key, *timestamp);
            }
            Event::KeyUp { key, .. } => {
                self.keys_pressed.remove(&key);
            }
            Event::PointerDown {
                button, timestamp, ..
            } => {
                self.pointer_buttons_pressed.insert(*button, *timestamp);
                self.pointer_buttons_down_since_last_frame
                    .insert(*button, *timestamp);
            }
            Event::PointerUp {
                button, timestamp, ..
            } => {
                self.pointer_buttons_released_since_last_frame
                    .insert(*button, *timestamp);
                self.pointer_buttons_pressed.remove(&button);
            }
            Event::PointerMoved { x, y, .. } => self.pointer_position = (*x, *y),
            Event::MouseMotion {
                delta_x, delta_y, ..
            } => self.mouse_motion = (self.mouse_motion.0 + delta_x, self.mouse_motion.1 + delta_y),
            _ => {}
        };
        self.all_events_since_last_frame.push(event.clone());
    }

    /// Reset any "button down" states
    pub fn clear(&mut self) {
        self.all_events_since_last_frame.clear();
        self.pointer_buttons_down_since_last_frame.clear();
        self.pointer_buttons_released_since_last_frame.clear();
        self.keys_down_since_last_frame.clear();
        self.mouse_motion = (0., 0.);
    }

    /// Returns true if the key has been pressed since the last call to clear.
    pub fn key_down(&self, key: Key) -> bool {
        self.keys_down_since_last_frame.contains_key(&key)
    }

    /// Returns true if all the keys specified been pressed since the last call to clear.
    /// Right now this doesn't work perfectly for keyboard shortcuts because
    /// the different modifier keys are split out into their left and right versions.
    pub fn keys_down(&self, keys: &[Key]) -> bool {
        let mut pressed = true;
        for key in keys {
            pressed |= self.keys_down_since_last_frame.contains_key(&key)
        }
        pressed
    }

    /// Returns if the key is currently down
    pub fn key(&self, key: Key) -> bool {
        self.keys_pressed.contains_key(&key)
    }

    /// Returns true if the pointer button has been pressed since the last call to clear.
    pub fn pointer_button_down(&self, button: PointerButton) -> bool {
        self.pointer_buttons_down_since_last_frame
            .contains_key(&button)
    }

    /// Returns true if the pointer button has been pressed since the last call to clear.
    pub fn pointer_button_released(&self, button: PointerButton) -> bool {
        self.pointer_buttons_released_since_last_frame
            .contains_key(&button)
    }

    /// Returns true if the pointer button is pressed
    pub fn pointer_button(&self, button: PointerButton) -> bool {
        self.pointer_buttons_pressed.contains_key(&button)
    }

    pub fn pointer_position(&self) -> (f64, f64) {
        self.pointer_position
    }

    pub fn mouse_motion(&self) -> (f64, f64) {
        self.mouse_motion
    }

    pub fn all_events_since_last_frame(&self) -> &[Event] {
        &self.all_events_since_last_frame
    }
}
