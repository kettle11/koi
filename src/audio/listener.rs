use crate::Vec3;
use kecs::*;

#[derive(Component, Clone)]
pub struct Listener {
    pub(super) last_position: Option<Vec3>,
}

impl Listener {
    pub fn new() -> Self {
        Self {
            last_position: None,
        }
    }
}

impl Default for Listener {
    fn default() -> Self {
        Listener::new()
    }
}
