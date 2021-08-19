/// This file provides some boilerplate for managing draw requests.
/// Draw requests are queued, but a draw event can produce more draw requests
/// So two queues are maintained that are swapped.
/// Draw requests can be fulfilled by calling 'begin_draw_flush' and
/// then 'get_draw_request' until None is returned.
/// Or 'draw' can be called by a system call to fulfill a queued draw request.
use crate::{Event, WindowId};
use std::cell::RefCell;

thread_local!(
    static DRAW_REQUESTS: RefCell<Vec<WindowId>> = RefCell::new(Vec::new());
    static DRAW_REQUESTS_SWAP: RefCell<Vec<WindowId>> = RefCell::new(Vec::new());
);

pub fn add_draw_request(window_id: WindowId) {
    DRAW_REQUESTS.with(|d| {
        let mut requests = d.borrow_mut();

        // Only allow one queued redraw per window.
        if !requests.contains(&window_id) {
            requests.push(window_id);
        }
    })
}

/// Called when the system requests a window redraw.
/// If a redraw is requested this call fulfills that redraw request.
pub fn draw(window_id: WindowId) {
    DRAW_REQUESTS.with(|requests| {
        // First remove the draw request to avoid it being fulfilled twice.
        let position = requests.borrow().iter().position(|w| w == &window_id);
        if let Some(position) = position {
            requests.borrow_mut().swap_remove(position);
        }

        crate::event_receiver::send_event(Event::Draw { window_id });
    });
}

pub fn draw_requests_count() -> usize {
    DRAW_REQUESTS.with(|d| d.borrow().len())
}

/// Called when starting to iterate through all draw requests.
pub fn begin_draw_flush() {
    DRAW_REQUESTS_SWAP.with(|swap| {
        DRAW_REQUESTS.with(|requests| requests.swap(swap));
    });
}

pub fn get_draw_request() -> Option<WindowId> {
    DRAW_REQUESTS_SWAP.with(|swap| swap.borrow_mut().pop())
}
