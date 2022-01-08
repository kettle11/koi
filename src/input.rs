use core::ops::Deref;
use kecs::*;

#[derive(NotCloneComponent)]
pub struct Input {
    pub(crate) state: kapp::StateTracker,
}

impl Deref for Input {
    type Target = kapp::StateTracker;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            state: kapp::StateTracker::new(),
        }
    }
}
