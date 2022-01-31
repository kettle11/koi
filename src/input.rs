use core::ops::Deref;
use kecs::*;

#[derive(NotCloneComponent)]
pub struct Input(pub(crate) kapp::StateTracker);

impl Deref for Input {
    type Target = kapp::StateTracker;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Input {
    pub(crate) fn new() -> Self {
        Self(kapp::StateTracker::new())
    }
}
