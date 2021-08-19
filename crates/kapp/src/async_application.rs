use crate::Application;
use crate::Event;
use crate::EventLoop;
use std::future::Future;

use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

pub fn run_async<F>(run: impl Fn(Application, Events) -> F)
where
    F: 'static + Future<Output = ()>,
{
    let (application, event_loop) = crate::initialize();
    event_loop.run_async(application, run);
}

pub struct EventFuture<'a> {
    events: &'a Events,
}

impl<'a> Future for EventFuture<'a> {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, _ctx: &mut Context) -> Poll<Self::Output> {
        if let Some(event) = self.events.queue.borrow_mut().pop() {
            Poll::Ready(event)
        } else {
            Poll::Pending
        }
    }
}

impl EventLoop {
    /// Events are sent to the program immediately as they're ready.
    /// However if the main program is blocked then events are queued.
    /// ```no_run
    /// use kapp::*;
    /// fn main() {
    ///    let (app, mut event_loop) = initialize();
    ///     event_loop.run_async(app, run);
    /// }
    ///
    /// async fn run(app: Application, mut events: Events) {
    ///     let mut _window = app.new_window().build().unwrap();
    ///
    ///     // Loop forever!
    ///     loop {
    ///         match events.next().await {
    ///             Event::WindowCloseRequested { .. } => app.quit(),
    ///             Event::Draw { .. } => {}
    ///             _ => {}
    ///         }
    ///     }
    /// }
    /// ```

    pub fn run_async<F>(
        &self,
        application: Application,
        run_function: impl Fn(Application, Events) -> F,
    ) where
        F: 'static + Future<Output = ()>,
    {
        let events = Events::new();
        let mut program =
            AsyncProgram::new(events.clone(), run_function(application.clone(), events));

        // A proper context and waker need to be setup here.
        self.run(move |event| {
            program.update(event);
        });

        // Probably nothing should be done here, but on web this is reached whenever the main loop is setup.
    }
}

pub struct AsyncProgram {
    events: Events,
    program: Pin<Box<dyn Future<Output = ()>>>,
}

impl AsyncProgram {
    pub fn new(events: Events, future: impl Future<Output = ()> + 'static) -> Self {
        let mut pin = Box::pin(future);

        let waker = waker::create();
        let mut context = Context::from_waker(&waker);

        // Poll the program once to give it a chance to setup things.
        if Poll::Ready(()) == pin.as_mut().poll(&mut context) {
            // Program immediately exited on first run.
        }

        Self {
            events,
            program: pin,
        }
    }

    pub fn update(&mut self, event: Event) {
        self.events.queue.borrow_mut().push(event);

        // This waker does nothing presently,
        // This means that completed futures won't actually wake up the main loop.
        // However the main loop has a chance to continue immediately on the next event.
        // In the future an artificial event should be triggered to ensure the main loop
        // continues immediately.
        // That artificial event may need to be implemented per platform.
        let waker = waker::create();
        let mut context = Context::from_waker(&waker);

        // The user main loop is polled here.
        // If it's awaiting something else this will immediately return 'pending'.
        // For an example if the main loop is blocked waiting for something to load
        // and a 'Draw' is requested it will not immediately occur which can cause
        // window resizing to flicker.
        if Poll::Ready(()) == self.program.as_mut().poll(&mut context) {
            // The main application loop has exited.
            // Do something here!
        }
    }
}

/// Passed to an asynchronous function to get the next event.
///
#[derive(Clone)]
pub struct Events {
    queue: Rc<RefCell<Vec<crate::Event>>>,
}

impl Events {
    pub fn new() -> Self {
        Events {
            queue: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl Events {
    pub fn next(&self) -> self::EventFuture {
        self::EventFuture { events: self }
    }
}

mod waker {
    use std::task::{RawWaker, RawWakerVTable, Waker};

    pub fn create() -> Waker {
        unsafe { Waker::from_raw(RAW_WAKER) }
    }

    const RAW_WAKER: RawWaker = RawWaker::new(std::ptr::null(), &VTABLE);
    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    unsafe fn clone(_: *const ()) -> RawWaker {
        RAW_WAKER
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}
}
