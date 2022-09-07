use kmath::Vec2;

use crate::{Event, Key, PointerButton, PointerSource};
use std::collections::HashMap;
use std::time::Duration;

// In the future this could be extended to track:
// * Window positions and status.
// * Window scale factors
// * Window color spaces

/// Tracks key and pointer input state based on events.
pub struct StateTracker {
    pub all_events_since_last_frame: Vec<Event>,
    keys_down_since_last_frame: HashMap<Key, Duration>, // Key was pressed since the last clear for any window.
    keys_pressed: HashMap<Key, Duration>,
    pointer_buttons_down_since_last_frame: HashMap<PointerButton, Duration>, // pointer was pressed since the last clear for any window.
    pointer_buttons_released_since_last_frame: HashMap<PointerButton, Duration>, // pointer was pressed since the last clear for any window.
    pointer_buttons_pressed: HashMap<PointerButton, Duration>,
    pointer_position: (f64, f64),
    mouse_motion: (f64, f64),
    /// How much pinching (the gesture used to zoom on touch devices) occured in the last frame
    pinch: f64,
    scroll: (f64, f64),
    pub touch_state: TouchState,
}

impl Default for StateTracker {
    fn default() -> Self {
        Self::new()
    }
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
            pinch: 0.0,
            scroll: (0.0, 0.0),
            touch_state: TouchState::new(),
        }
    }

    /// Constructs a [StateTracker] from a series of events all at once.
    pub fn set_with_events(&mut self, events: Vec<Event>) {
        self.clear();

        for event in &events {
            self.handle_event_inner(event);
        }
        self.all_events_since_last_frame = events;
    }

    fn handle_event_inner(&mut self, event: &Event) {
        self.touch_state.handle_event(event);

        match event {
            Event::KeyDown { key, timestamp } => {
                self.keys_pressed.insert(*key, *timestamp);
                self.keys_down_since_last_frame.insert(*key, *timestamp);
            }
            Event::KeyUp { key, .. } => {
                self.keys_pressed.remove(key);
            }
            Event::PointerDown {
                button,
                timestamp,
                x,
                y,
                ..
            } => {
                self.pointer_position = (*x, *y);
                self.pointer_buttons_pressed.insert(*button, *timestamp);
                self.pointer_buttons_down_since_last_frame
                    .insert(*button, *timestamp);
            }
            Event::PointerUp {
                button,
                timestamp,
                x,
                y,
                ..
            } => {
                self.pointer_position = (*x, *y);
                self.pointer_buttons_released_since_last_frame
                    .insert(*button, *timestamp);
                self.pointer_buttons_pressed.remove(button);
            }
            Event::PointerMoved { x, y, .. } => self.pointer_position = (*x, *y),
            Event::MouseMotion {
                delta_x, delta_y, ..
            } => self.mouse_motion = (self.mouse_motion.0 + delta_x, self.mouse_motion.1 + delta_y),
            Event::PinchGesture { delta, .. } => self.pinch += *delta,
            Event::Scroll {
                delta_x, delta_y, ..
            } => {
                self.scroll.0 += *delta_x;
                self.scroll.1 += *delta_y;
            }
            _ => {}
        };
    }
    pub fn handle_event(&mut self, event: &Event) {
        self.handle_event_inner(event);
        self.all_events_since_last_frame.push(event.clone());
    }

    /// Reset any "button down" states
    pub fn clear(&mut self) {
        self.all_events_since_last_frame.clear();
        self.pointer_buttons_down_since_last_frame.clear();
        self.pointer_buttons_released_since_last_frame.clear();
        self.keys_down_since_last_frame.clear();
        self.mouse_motion = (0., 0.);
        self.pinch = 0.0;
        self.scroll = (0.0, 0.0);
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
            pressed |= self.keys_down_since_last_frame.contains_key(key)
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

    /// Returns a trackpad or touch pinch event.
    pub fn pinch(&self) -> f32 {
        let pinch = self.pinch as f32;
        let touch_pinch = self.touch_state.pinch();
        if pinch.abs() > touch_pinch.abs() {
            pinch
        } else {
            touch_pinch
        }
    }

    pub fn two_finger_pan(&self) -> Vec2 {
        self.touch_state.two_finger_pan()
    }

    pub fn scroll(&self) -> (f64, f64) {
        self.scroll
    }

    pub fn all_events_since_last_frame(&self) -> &[Event] {
        &self.all_events_since_last_frame
    }

    pub fn reset_touch(&mut self) {
        self.touch_state.set_old_positions();
    }

    /// Iterate over all keys currently pressed.
    pub fn keys_currently_pressed(&self) -> impl Iterator<Item = Key> + '_ {
        self.keys_pressed.iter().map(|k| k.0.clone())
    }
}

#[derive(Clone)]
pub struct Touch {
    old_position: Vec2,
    position: Vec2,
}

impl Touch {
    pub fn delta(&self) -> Vec2 {
        self.position - self.old_position
    }
}
#[derive(Clone)]
pub struct TouchState {
    pub touches: std::collections::HashMap<usize, Touch>,
    pinch_pair: Option<(usize, usize)>,
}

impl Default for TouchState {
    fn default() -> Self {
        Self::new()
    }
}

impl TouchState {
    pub fn new() -> Self {
        Self {
            touches: std::collections::HashMap::new(),
            pinch_pair: None,
        }
    }
    pub fn set_old_positions(&mut self) {
        for touch in self.touches.iter_mut() {
            touch.1.old_position = touch.1.position;
        }
    }
    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::PointerDown {
                x,
                y,
                source: PointerSource::Touch,
                id,
                ..
            } => {
                let position = Vec2::new(*x as f32, *y as f32);
                self.touches.insert(
                    *id,
                    Touch {
                        old_position: position,
                        position,
                    },
                );
                if self.touches.len() == 2 && self.pinch_pair.is_none() {
                    let mut iter = self.touches.keys();
                    self.pinch_pair = Some((*iter.next().unwrap(), *iter.next().unwrap()));
                }
            }
            Event::PointerMoved {
                x,
                y,
                source: PointerSource::Touch,
                id,
                ..
            } => {
                let position = Vec2::new(*x as f32, *y as f32);
                // I thought this
                if let Some(touch) = self.touches.get_mut(id) {
                    touch.position = position;
                }
            }
            Event::PointerUp {
                source: PointerSource::Touch,
                id,
                ..
            } => {
                self.touches.remove(id);
                if let Some(pinch_pair) = self.pinch_pair {
                    if pinch_pair.0 == *id || pinch_pair.1 == *id {
                        self.pinch_pair = None;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn pinch(&self) -> f32 {
        if let Some(pinch_pair) = self.pinch_pair {
            let touch0 = self.touches.get(&pinch_pair.0).unwrap();
            let touch1 = self.touches.get(&pinch_pair.1).unwrap();

            let previous_amount_apart = (touch0.old_position - touch1.old_position).length();
            let current_amount_apart = (touch0.position - touch1.position).length();

            let pinch = current_amount_apart - previous_amount_apart;

            // 500. is completely arbitrary here. Probably this should be scaled by the UI scale factor as well.
            pinch / 500.
        } else {
            0.0
        }
    }

    /// Returns how much two finger panning has occurred since the last frame.
    pub fn two_finger_pan(&self) -> Vec2 {
        let len = self.touches.len();
        if len < 2 {
            Vec2::ZERO
        } else {
            let len = len as f32;
            let mut previous_center = Vec2::ZERO;
            let mut current_center = Vec2::ZERO;

            for touch in self.touches.iter() {
                previous_center += touch.1.old_position;
                current_center += touch.1.position;
            }

            previous_center /= len as f32;
            current_center /= len as f32;
            (current_center - previous_center) / 200.
        }
    }
}
