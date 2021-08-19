use super::apple::*;
use super::window_mac::WindowState;
use kapp_platform_common::*;

use std::cell::RefCell;
use std::ffi::c_void;

thread_local!(pub(crate) static APPLICATION_DATA: RefCell<Box<ApplicationData>> = RefCell::new(Box::new(ApplicationData::new())));

// Global singleton data shared by the application struct.
pub(crate) struct ApplicationData {
    ns_application: *mut Object,
    pub modifier_flags: u64,      // Key modifier flags
    pub actually_terminate: bool, // Set when quit is called. Indicates the program should quit.
    pub text_input_enabled: bool, // Should text input be sent in addition to KeyDown events?
    pub mouse_lock: bool,
    pub user_event_sender: std::sync::mpsc::Sender<(usize, usize)>,
    pub custom_event_receiver: std::sync::mpsc::Receiver<(usize, usize)>,
}

impl ApplicationData {
    pub fn new() -> Self {
        let (user_event_sender, custom_event_receiver) = std::sync::mpsc::channel();
        Self {
            ns_application: std::ptr::null_mut(),
            modifier_flags: 0,
            actually_terminate: false,
            text_input_enabled: false,
            mouse_lock: false,
            user_event_sender,
            custom_event_receiver,
        }
    }
}

fn window_delegate_declaration() -> *const objc::runtime::Class {
    let superclass = unsafe { &*NSResponderClass };
    let mut decl = ClassDecl::new("kappWindowClass", superclass).unwrap();
    super::events_mac::add_window_events_to_decl(&mut decl);
    decl.register()
}

fn view_delegate_declaration() -> *const objc::runtime::Class {
    let superclass = unsafe { &*NSViewClass };
    let mut decl = ClassDecl::new("kappViewClass", superclass).unwrap();
    super::events_mac::add_view_events_to_decl(&mut decl);
    decl.register()
}

fn application_delegate_declaration() -> *const objc::runtime::Class {
    let superclass = unsafe { &*NSResponderClass };
    let mut decl = ClassDecl::new("kappApplicationClass", superclass).unwrap();
    super::events_mac::add_application_events_to_decl(&mut decl);
    decl.register()
}

fn create_run_loop_source() -> CFRunLoopSourceRef {
    extern "C" fn event_loop_proxy_handler(_: *mut std::ffi::c_void) {}

    unsafe {
        let rl = CFRunLoopGetMain();
        let mut context: CFRunLoopSourceContext = std::mem::zeroed();
        context.perform = Some(event_loop_proxy_handler);
        let source =
            CFRunLoopSourceCreate(std::ptr::null_mut(), CFIndex::max_value() - 1, &mut context);
        CFRunLoopAddSource(rl, source, kCFRunLoopCommonModes);
        CFRunLoopWakeUp(rl);
        source
    }
}

extern "C" fn control_flow_end_handler(
    _: CFRunLoopObserverRef,
    _: CFRunLoopActivity,
    _: *mut std::ffi::c_void,
) {
    // This loop is constructed weird to avoid borrow APPLICATION_DATA while sending the custom event.
    loop {
        let custom_event = APPLICATION_DATA.with(|d| d.borrow().custom_event_receiver.try_recv());
        if let Ok((id, data)) = custom_event {
            event_receiver::send_event(Event::UserEvent { id, data });
        } else {
            break;
        }
    }
    event_receiver::send_event(Event::EventsCleared);

    redraw_manager::begin_draw_flush();
    while let Some(window_id) = redraw_manager::get_draw_request() {
        // If live resizing redraw only in response to a 'drawRect' event in order to keep
        // resizing smooth.
        // Redrawing during resize will always produce events in sync with the monitor refresh rate.
        let in_live_resize: bool =
            unsafe { msg(window_id.raw() as *mut Object, Sels::inLiveResize, ()) };

        if in_live_resize {
            unsafe {
                let content_view: *mut Object =
                    msg(window_id.raw() as *mut Object, Sels::contentView, ());
                let () = msg(content_view, Sels::setNeedsDisplay, (YES,));
            }
        } else {
            // July 22 2021: I've enabled setNeedsDisplay call *again* because frame-rates were stuttering
            // when issuing a `Draw` event directly here. This means VSync is broken on Macs again.
            // Previous comment:
            // We previously used this call to request a redraw.
            // Getting redraw requests via `setNeedsDisplay` prevents window moving lag when using VSync with the Magnet extension installed.
            // I (@kettle11) misattributed the cause to something the MacOS compositor was doing, but eventually discovered
            // the `Magnet` utility app was the source of the bug.
            // Issuing a Draw event immediately here allows other libraries more control over how they render
            // and when they VSync, so we'll use that approach instead of the old one,
            // even if it causes bugs for those with Magnet installed.
            // We will still rely on the system for VSync during resizing
            // as this ensures smooth synchronization with the OS compositor.

            unsafe {
                let content_view: *mut Object =
                    msg_send![window_id.raw() as *mut Object, contentView];
                let () = msg_send![content_view, setNeedsDisplay: YES];
            }

            // If the `setNeedsDisplay` were directly sent here that would effectively disable VSync for this window.
            // However this seems to introduce lag moving the window when used with VSync.
            // By using the above code instead this prevents VSync from being disabled on MacOS.
            // event_receiver::send_event(Event::Draw { window_id });
        }
    }

    // Termination, if requested, occurs here.
    // Termination occurs here to avoid holding the program closure while termination is requested.
    unsafe {
        let data = {
            APPLICATION_DATA.try_with(|d| {
                let actually_terminate = d.borrow().actually_terminate;
                (actually_terminate, d.borrow_mut().ns_application)
            })
        };

        if let Ok((should_terminate, ns_application)) = data {
            if should_terminate {
                let () = msg(ns_application, Sels::terminate, (nil,));
            }
        }
    }

    // If there are any redraw requests wake up the main loop and run it again.
    if redraw_manager::draw_requests_count() > 0 {
        unsafe {
            let rl = CFRunLoopGetMain();
            CFRunLoopWakeUp(rl);
        }
    }
}

pub struct PlatformEventLoop {
    ns_application: *mut Object,
}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&self, callback: Box<dyn FnMut(Event)>) {
        event_receiver::set_callback(callback);

        unsafe {
            let () = msg(self.ns_application, Sels::run, ());
        }
    }
}

pub struct PlatformApplication {
    // application_data: Rc<RefCell<ApplicationData>>,
    window_class: *const objc::runtime::Class,
    view_class: *const objc::runtime::Class,
    ns_application: *mut Object,
    _run_loop_custom_event_source: CFRunLoopSourceRef,
}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;
    type UserEventSender = PlatformUserEventSender;

    fn new() -> Self {
        unsafe {
            // Requests and loads the relevant Objc classes.
            initialize_classes();

            // https://developer.apple.com/documentation/appkit/nsapplication
            // Retrieve the global 'sharedApplication'
            let ns_application: *mut Object = msg(NSApplicationClass, Sels::sharedApplication, ());

            // https://developer.apple.com/documentation/appkit/nsapplicationactivationpolicy/nsapplicationactivationpolicyregular?language=objc
            // "The application is an ordinary app that appears in the Dock and may have a user interface."
            // Apple claims this is the default, but without manually setting it the application does not appear.
            let () = msg(
                ns_application,
                Sels::setActivationPolicy,
                (NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,),
            );

            // This line is necessary to ensure the app becomes active.
            // But does it improperly steal focus if app activation takes a while and the
            // user launches another app while they wait?
            let () = msg_send![ns_application, activateIgnoringOtherApps: YES];

            // Setup the application delegate to handle application events.
            let ns_application_delegate_class = application_delegate_declaration();
            // Create an instance of the delegate. The delegate's functions receives events for the application.
            let ns_application_delegate: *mut Object =
                msg(ns_application_delegate_class, Sels::new, ());
            // Assign the delegate to the application.
            let () = msg(
                ns_application,
                Sels::setDelegate,
                (ns_application_delegate,),
            );

            // Create an observer that runs at the end of the event loop to
            // produce `Draw` and `EventsCleared` events.
            let run_loop_custom_event_source = self::create_run_loop_source();
            let observer = CFRunLoopObserverCreate(
                std::ptr::null_mut(),
                kCFRunLoopBeforeWaiting,
                YES,                  // Indicates we want this to run repeatedly
                CFIndex::min_value(), // Prioritize this to run last.
                control_flow_end_handler,
                std::ptr::null(),
            );
            CFRunLoopAddObserver(CFRunLoopGetMain(), observer, kCFRunLoopCommonModes);

            // Store the application in a thread local.
            APPLICATION_DATA.with(|d| {
                d.borrow_mut().ns_application = ns_application;
            });

            Self {
                window_class: window_delegate_declaration(),
                view_class: view_delegate_declaration(),
                ns_application,
                _run_loop_custom_event_source: run_loop_custom_event_source,
            }
        }
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {
            ns_application: self.ns_application,
        }
    }

    fn set_window_position(&mut self, window_id: WindowId, x: u32, y: u32) {
        unsafe {
            let screen: *const Object = msg(window_id.raw() as *mut Object, Sels::screen, ());
            let screen_frame: CGRect = msg(screen, Sels::frame, ());

            let backing_scale = get_backing_scale(window_id);
            let () = msg(
                window_id.raw() as *mut Object,
                Sels::setFrameTopLeftPoint,
                (NSPoint::new(
                    (x as f64) / backing_scale,
                    screen_frame.size.height - (y as f64) / backing_scale,
                ),),
            );
        }
    }

    fn set_window_size(&mut self, window_id: WindowId, width: u32, height: u32) {
        unsafe {
            let backing_scale = get_backing_scale(window_id);
            let () = msg(
                window_id.raw() as *mut Object,
                Sels::setContentSize,
                (NSSize::new(
                    (width as f64) / backing_scale,
                    (height as f64) / backing_scale,
                ),),
            );
        }
    }

    fn set_window_title(&mut self, window_id: WindowId, title: &str) {
        unsafe {
            let title = NSString::new(&title);
            let () = msg(window_id.raw() as *mut Object, Sels::setTitle, (title.raw,));
        }
    }

    fn minimize_window(&mut self, window_id: WindowId) {
        unsafe {
            let () = msg(window_id.raw() as *mut Object, Sels::miniaturize, (nil,));
        }
    }

    fn maximize_window(&mut self, _window_id: WindowId) {
        // Not implemented on Mac
        // There is no analogous behavior?
    }

    fn fullscreen_window(&mut self, window_id: WindowId) {
        unsafe {
            let () = msg(
                window_id.raw() as *mut Object,
                Sels::toggleFullScreen,
                (nil,),
            );
        }
    }

    fn restore_window(&mut self, _window_id: WindowId) {
        todo!()
    }

    fn close_window(&mut self, window_id: WindowId) {
        unsafe {
            let () = msg(window_id.raw() as *mut Object, Sels::close, ());
        }
    }

    fn redraw_window(&mut self, window_id: WindowId) {
        // If we were to call 'setNeedsDisplay' here it would immediately trigger and create a potentially infinite loop.
        // Instead 'setNeedsDisplay' is called at the end of the loop where it seems to be immune to such dangers.
        redraw_manager::add_draw_request(window_id);
    }

    fn get_window_size(&mut self, window_id: WindowId) -> (u32, u32) {
        unsafe {
            let backing_scale = get_backing_scale(window_id);
            let content_view: *mut Object =
                msg(window_id.raw() as *mut Object, Sels::contentView, ());
            let frame: CGRect = msg(content_view, Sels::frame, ());

            (
                (frame.size.width * backing_scale) as u32,
                (frame.size.height * backing_scale) as u32,
            )
        }
    }

    fn get_window_scale(&mut self, window_id: WindowId) -> f64 {
        get_backing_scale(window_id)
    }

    fn lock_mouse_position(&mut self) {
        unsafe {
            CGAssociateMouseAndMouseCursorPosition(false);
        }
        APPLICATION_DATA.with(|d| {
            d.borrow_mut().mouse_lock = true;
        });
    }

    fn unlock_mouse_position(&mut self) {
        unsafe {
            CGAssociateMouseAndMouseCursorPosition(true);
        }
        APPLICATION_DATA.with(|d| {
            d.borrow_mut().mouse_lock = false;
        });
    }

    // https://developer.apple.com/documentation/appkit/nscursor?language=objc
    fn set_cursor(&mut self, cursor: Cursor) {
        let ns_cursor = unsafe { &*NSCursorClass };
        let cursor: *mut Object = unsafe {
            match cursor {
                Cursor::Arrow => msg(ns_cursor, Sels::arrowCursor, ()),
                Cursor::IBeam => msg(ns_cursor, Sels::IBeamCursor, ()),
                Cursor::PointingHand => msg(ns_cursor, Sels::pointingHandCursor, ()),
                Cursor::OpenHand => msg(ns_cursor, Sels::openHandCursor, ()),
                Cursor::ClosedHand => msg(ns_cursor, Sels::closedHandCursor, ()),
            }
        };
        let () = unsafe { msg(cursor, Sels::set, ()) };
    }

    fn hide_cursor(&mut self) {
        // For every call to 'hide' an 'unhide' must be called to make the cursor visible.
        // Because of this 'unhide' is always called before every call to hide.
        // This ensures that there is only ever one hide active at once.
        let ns_cursor = unsafe { &*NSCursorClass };
        unsafe {
            let () = msg(ns_cursor, Sels::unhide, ());
            let () = msg(ns_cursor, Sels::hide, ());
        }
    }

    fn show_cursor(&mut self) {
        let ns_cursor = unsafe { &*NSCursorClass };
        unsafe {
            let () = msg(ns_cursor, Sels::unhide, ());
        }
    }

    fn new_window(&mut self, window_parameters: &WindowParameters) -> WindowId {
        let result =
            super::window_mac::build(window_parameters, self.window_class, self.view_class);
        result.unwrap()
    }

    fn quit(&self) {
        // This thread local cannot be accessed if the program is already terminating.
        let _ = APPLICATION_DATA.try_with(|d| {
            d.borrow_mut().actually_terminate = true;
        });

        // Actual termination is postponed until the end of the event loop
        // to give the user program a chance to process events.
    }

    fn raw_window_handle(&self, window_id: WindowId) -> RawWindowHandle {
        unsafe {
            let ns_window = window_id.raw();
            let ns_view: *mut c_void = msg(window_id.raw() as *mut Object, Sels::contentView, ());
            raw_window_handle::RawWindowHandle::MacOS(raw_window_handle::macos::MacOSHandle {
                ns_window,
                ns_view,
                ..raw_window_handle::macos::MacOSHandle::empty()
            })
        }
    }

    fn start_text_input(&mut self) {
        APPLICATION_DATA.with(|d| {
            d.borrow_mut().text_input_enabled = true;
        });
    }

    fn end_text_input(&mut self) {
        APPLICATION_DATA.with(|d| {
            d.borrow_mut().text_input_enabled = false;
        });
    }

    fn set_text_input_rectangle(
        &mut self,
        window_id: WindowId,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        unsafe {
            let ns_view: &Object = msg(window_id.raw() as *mut Object, Sels::contentView, ());
            let window_state: *mut c_void = *ns_view.get_ivar("kappState");
            let window_state = window_state as *mut WindowState;
            (*window_state).text_input_rectangle = (x, y, width, height);
        }
    }

    fn get_user_event_sender(&self) -> Self::UserEventSender {
        Self::UserEventSender {
            sender: APPLICATION_DATA.with(|d| d.borrow().user_event_sender.clone()),
        }
    }
}

pub fn get_backing_scale(window_id: WindowId) -> CGFloat {
    unsafe { msg(window_id.raw() as *mut Object, Sels::backingScaleFactor, ()) }
}

// When the application is dropped, quit the program.
impl Drop for PlatformApplication {
    fn drop(&mut self) {
        self.quit();
    }
}

#[derive(Clone)]
pub struct PlatformUserEventSender {
    sender: std::sync::mpsc::Sender<(usize, usize)>,
}

impl PlatformUserEventSenderTrait for PlatformUserEventSender {
    fn send(&self, id: usize, data: usize) {
        // Wake up the main thread
        unsafe {
            let rl = CFRunLoopGetMain();
            CFRunLoopWakeUp(rl);
        }
        let _ = self.sender.send((id, data));
    }
}
