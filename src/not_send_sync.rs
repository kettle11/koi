use kecs::*;
use std::ops::{Deref, DerefMut};

/// Ensure a component can only be accessed on the thread that created it.
/// Presently everything runs on the main thread, so this isn't yet complete.
// I'm not particularly fond of this approach, but it seems reasonable for now.
// For now label all components that use this as not Clone.
#[derive(NotCloneComponent)]
pub struct NotSendSync<T: 'static> {
    value: T,
    thread_id: std::thread::ThreadId,
}

impl<T: 'static + Clone> Clone for NotSendSync<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            thread_id: self.thread_id.clone(),
        }
    }
}

unsafe impl<T> Send for NotSendSync<T> {}
unsafe impl<T> Sync for NotSendSync<T> {}

impl<T> NotSendSync<T> {
    pub fn new(t: T) -> Self {
        Self {
            value: t,
            thread_id: std::thread::current().id(),
        }
    }

    pub fn get(&self) -> &T {
        assert!(std::thread::current().id() == self.thread_id);
        &self.value
    }

    pub fn take(self) -> T {
        assert!(std::thread::current().id() == self.thread_id);
        self.value
    }
}

impl<T> Deref for NotSendSync<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Crash if we're trying to access this on the wrong thread.
        assert!(std::thread::current().id() == self.thread_id);
        &self.value
    }
}

impl<T> DerefMut for NotSendSync<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Crash if we're trying to access this on the wrong thread.
        assert!(std::thread::current().id() == self.thread_id);
        &mut self.value
    }
}
