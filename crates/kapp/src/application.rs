use crate::platform::*;
use crate::platform::{PlatformApplicationTrait, PlatformEventLoopTrait};
use crate::state_tracker::StateTracker;
use std::cell::RefCell;
use std::rc::Rc;

/// A handle used to do things like quit,
/// request a new frame, or create windows.
#[derive(Clone)]
pub struct Application {
    pub(crate) platform_application: Rc<RefCell<PlatformApplication>>,
    state_tracker: Rc<RefCell<StateTracker>>,
}

/// Create an Application and EventLoop.
pub fn initialize() -> (Application, EventLoop) {
    let platform_application = Rc::new(RefCell::new(PlatformApplication::new()));
    let platform_event_loop = platform_application.borrow_mut().event_loop();
    let state_tracker = Rc::new(RefCell::new(StateTracker::new()));
    (
        Application {
            platform_application: platform_application.clone(),
            state_tracker: state_tracker.clone(),
        },
        EventLoop {
            platform_event_loop,
            state_tracker: state_tracker.clone(),
        },
    )
}

impl Application {
    /// Returns a new window builder.
    /// Call .build() on the window builder to complete the creation of the window.
    /// See [`crate::window_builder::WindowBuilder`] for more ways to setup a window.
    pub fn new_window(&self) -> crate::window_builder::WindowBuilder {
        crate::window_builder::WindowBuilder::new(self)
    }

    /// Immediately quits the application.
    pub fn quit(&self) {
        self.platform_application.borrow().quit();
    }

    /// Prevents the mouse from moving until a call to `unlock_mouse_position`
    pub fn lock_mouse_position(&self) {
        self.platform_application.borrow_mut().lock_mouse_position();
    }

    /// Sets the mouse position relative to the screen.
    /// Coordinates are expressed in physical coordinates.
    pub fn unlock_mouse_position(&self) {
        self.platform_application
            .borrow_mut()
            .unlock_mouse_position();
    }

    pub fn set_cursor(&self, cursor: Cursor) {
        self.platform_application.borrow_mut().set_cursor(cursor);
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        if visible {
            self.platform_application.borrow_mut().show_cursor();
        } else {
            self.platform_application.borrow_mut().hide_cursor();
        }
    }

    /// Enable text input.
    /// `kapp` will send `CharacterReceived` events until `stop_text_input` is called.
    /// Operating system UI related to text input may appear.
    /// ONLY SUPPORTED ON MAC (for now)
    pub fn start_text_input(&self) {
        self.platform_application.borrow_mut().start_text_input()
    }

    /// Disable text input.
    /// Used to end text input after a call to `start_text_input`.
    pub fn end_text_input(&self) {
        self.platform_application.borrow_mut().end_text_input()
    }

    /// Returns if the key is currently pressed
    pub fn key(&self, key: Key) -> bool {
        self.state_tracker.borrow().key(key)
    }

    /// Returns true if the key has been pressed since the last draw
    pub fn key_down(&self, key: Key) -> bool {
        self.state_tracker.borrow().key_down(key)
    }

    /// Returns true if all the keys specified been pressed since the last draw.
    /// Right now this doesn't work perfectly for keyboard shortcuts because
    /// the different modifier keys are split out into their left and right versions.
    pub fn keys_down(&self, keys: &[Key]) -> bool {
        self.state_tracker.borrow().keys_down(keys)
    }

    /// Returns true if the pointer button is pressed
    pub fn pointer_button(&self, button: PointerButton) -> bool {
        self.state_tracker.borrow().pointer_button(button)
    }

    /// Returns true if the pointer button has been pressed since the last draw
    pub fn pointer_button_down(&self, button: PointerButton) -> bool {
        self.state_tracker.borrow().pointer_button_down(button)
    }

    /// Returns the current pointer position
    /// The current screen is unspecified, but perhaps that should change
    /// in the future.
    pub fn pointer_position(&self) -> (f64, f64) {
        self.state_tracker.borrow().pointer_position()
    }

    pub fn get_user_event_sender(&self) -> UserEventSender {
        UserEventSender {
            sender: self.platform_application.borrow().get_user_event_sender(),
            prevent_send: std::marker::PhantomData
        }
    }
}

/// Call the 'run' or 'run_async' function on an EventLoop instance to start your program.
pub struct EventLoop {
    platform_event_loop: PlatformEventLoop,
    state_tracker: Rc<RefCell<StateTracker>>,
}

impl EventLoop {
    /// Run the application. The callback is called for each new event.
    pub fn run<T>(&self, mut callback: T)
    where
        T: 'static + FnMut(Event),
    {
        let state_tracker = self.state_tracker.clone();
        let callback_wrapper = move |event: Event| {
            state_tracker.borrow_mut().handle_event(&event);
            callback(event.clone());

            match event {
                Event::Draw { .. } => {
                    state_tracker.borrow_mut().clear();
                }
                _ => {}
            };
        };
        self.platform_event_loop.run(Box::new(callback_wrapper));
    }
}

/// [UserEventSender] can be used to send [Event::UserEvent]s.
/// Presently [UserEventSender] cannot be send between threads
/// due to how the web backend is implemented.
pub struct UserEventSender {
    sender: PlatformUserEventSender,
    // This field is added to prevent this from being send.
    // Presently the web backend is not Send due to its use 
    // of thread locals but when that's correct this can be removed.
    prevent_send: std::marker::PhantomData<std::cell::Cell<*const ()>>
}

impl UserEventSender {
    pub fn send(&self, id: usize, data: usize) {
        self.sender.send(id, data)
    }
}
