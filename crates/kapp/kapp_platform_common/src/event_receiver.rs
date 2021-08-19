use crate::Event;
/// This file handles sending events to a thread local callback that contains the
/// crate user's code.
/// Some calls from within the user's code will immediately produce new events.
/// In that case push the events to an overflow queue that is processed
/// when the callback is available again.
use std::cell::RefCell;

thread_local!(
    static PROGRAM_CALLBACK: RefCell<Box<dyn 'static + FnMut(Event)>> =
        RefCell::new(Box::new(|_| {
            panic!("KAPP INTERNAL ERROR: Sent event to unitialized user-callback")
        }));
    static OVERFLOW_EVENTS: RefCell<Vec<Event>> = RefCell::new(Vec::new());
);

pub fn set_callback(callback: Box<dyn FnMut(Event)>) {
    PROGRAM_CALLBACK.with(|p| {
        let _ = p.replace(callback);
    });
}

/// Sends an event to the user callback
/// This should only be called from the main thread.
pub fn send_event(event: Event) {
    // try_with because events may be sent during destruction, which should be ignored.
    let _ = PROGRAM_CALLBACK.try_with(|p| {
        if let Ok(mut callback) = p.try_borrow_mut() {
            (callback.as_mut())(event);

            // Flush events here to somewhat preserve the ordering of events.
            flush_overflow_events(&mut callback);
        } else {
            // If the callback is in use then push the event to overflow events to be
            // processed later.
            OVERFLOW_EVENTS.with(|events| {
                events.borrow_mut().push(event);
            });
        }
    });
}

/// Sends events that could not be sent because the user callback was borrowed.
fn flush_overflow_events(callback: &mut Box<dyn 'static + FnMut(Event)>) {
    // Temporarily borrow the overflow event queue and pop from it to avoid
    // holding a reference to it during the callback.
    let mut next_event = {
        OVERFLOW_EVENTS
            .try_with(|events| events.borrow_mut().pop())
            .unwrap_or(None)
    };
    while let Some(event) = next_event {
        callback(event);

        next_event = {
            OVERFLOW_EVENTS
                .try_with(|events| events.borrow_mut().pop())
                .unwrap_or(None)
        };
    }
}
