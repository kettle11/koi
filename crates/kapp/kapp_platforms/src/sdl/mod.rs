mod keys_sdl;
use kapp_platform_common::*;
use keys_sdl::*;

use fermium::{events::*, keyboard::*, mouse::*, rect::*, stdinc::*, touch::*, video::*, *};

use core::cell::Cell;
use std::ffi::{CStr, CString};
use std::time::Duration;

pub mod prelude {
    pub use super::*;
    pub use kapp_platform_common::*;
}

pub struct PlatformApplication {
    // These cursors are deallocated with `SDL_FreeCursor` in PlatformApplication's Drop
    arrow_cursor: *mut SDL_Cursor,
    ibeam_cursor: *mut SDL_Cursor,
    open_hand_cursor: *mut SDL_Cursor,
}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;
    type UserEventSender = PlatformUserEventSender;

    fn new() -> Self {
        unsafe {
            assert!(SDL_Init(SDL_INIT_EVERYTHING) == 0);

            Self {
                arrow_cursor: SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_ARROW),
                ibeam_cursor: SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_IBEAM),
                open_hand_cursor: SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_HAND),
            }
        }
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {}
    }

    fn set_window_position(&mut self, window_id: WindowId, x: u32, y: u32) {
        unsafe {
            SDL_SetWindowPosition(window_id.raw() as *mut SDL_Window, x as i32, y as i32);
        }
    }

    fn set_window_size(&mut self, window_id: WindowId, width: u32, height: u32) {
        unsafe {
            SDL_SetWindowPosition(
                window_id.raw() as *mut SDL_Window,
                width as i32,
                height as i32,
            );
        }
    }
    fn set_window_title(&mut self, window_id: WindowId, title: &str) {
        unsafe {
            let c_string = CString::new(title).unwrap();
            SDL_SetWindowTitle(window_id.raw() as *mut SDL_Window, c_string.as_ptr());
        }
    }
    fn minimize_window(&mut self, window_id: WindowId) {
        unsafe {
            SDL_MinimizeWindow(window_id.raw() as *mut SDL_Window);
        }
    }
    fn maximize_window(&mut self, window_id: WindowId) {
        unsafe {
            SDL_MaximizeWindow(window_id.raw() as *mut SDL_Window);
        }
    }
    fn get_window_size(&mut self, window_id: WindowId) -> (u32, u32) {
        let mut width = 0;
        let mut height = 0;
        unsafe {
            SDL_GL_GetDrawableSize(window_id.raw() as *mut SDL_Window, &mut width, &mut height);
        }
        (width as u32, height as u32)
    }

    fn get_window_scale(&mut self, window_id: WindowId) -> f64 {
        let (logical_width, _logical_height) = self.get_window_size(window_id);
        let mut physical_width = 0;
        let mut _physical_height = 0;

        // This call returns the actual pixel widths that would be in a framebuffer.
        unsafe {
            SDL_GL_GetDrawableSize(
                window_id.raw() as *mut SDL_Window,
                &mut physical_width,
                &mut _physical_height,
            );
        }
        logical_width as f64 / physical_width as f64
    }

    fn fullscreen_window(&mut self, window_id: WindowId) {
        unsafe {
            SDL_SetWindowFullscreen(window_id.raw() as *mut SDL_Window, SDL_WINDOW_FULLSCREEN.0);
        }
    }
    fn restore_window(&mut self, window_id: WindowId) {
        unsafe {
            SDL_RestoreWindow(window_id.raw() as *mut SDL_Window);
        }
    }
    fn close_window(&mut self, window_id: WindowId) {
        unsafe {
            SDL_DestroyWindow(window_id.raw() as *mut SDL_Window);
        }
    }
    fn redraw_window(&mut self, window_id: WindowId) {
        redraw_manager::add_draw_request(window_id);
    }
    fn lock_mouse_position(&mut self) {
        unsafe {
            SDL_SetRelativeMouseMode(SDL_TRUE);
        }
    }
    fn unlock_mouse_position(&mut self) {
        unsafe {
            SDL_SetRelativeMouseMode(SDL_FALSE);
        }
    }

    fn new_window(&mut self, window_parameters: &WindowParameters) -> WindowId {
        let (x, y) = window_parameters.position.unwrap_or((
            SDL_WINDOWPOS_UNDEFINED as u32,
            SDL_WINDOWPOS_UNDEFINED as u32,
        ));

        // TODO: Width and height are presently incorrect as SDL interprets them as logical pixels.
        // DPI scale factor needs to be accounted for.
        let (width, height) = window_parameters.size.unwrap();

        // SDL_WINDOW_OPENGL is probably not something `kapp`
        // wants to assume.
        // But this is tolerable for now.
        let mut flags = SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI;
        if window_parameters.resizable {
            flags |= SDL_WINDOW_RESIZABLE;
        }

        if window_parameters.resizable {
            flags |= SDL_WINDOW_RESIZABLE;
        }
        unsafe {
            let window = SDL_CreateWindow(
                b"demo\0".as_ptr().cast(),
                x as i32,
                y as i32,
                (width / 2) as i32,
                (height / 2) as i32,
                flags.0,
            );

            // How can min / max sizes be unset later?
            if let Some((min_width, min_height)) = window_parameters.minimum_size {
                SDL_SetWindowMinimumSize(window, min_width as i32, min_height as i32)
            }

            if let Some((max_width, max_height)) = window_parameters.maximum_size {
                SDL_SetWindowMaximumSize(window, max_width as i32, max_height as i32)
            }

            let c_string = std::ffi::CString::new(window_parameters.title.clone()).unwrap();
            SDL_SetWindowTitle(window, c_string.as_ptr());

            let window_id = WindowId::new(window as *mut c_void);
            // When a window is created immediately request that it should redraw
            redraw_manager::add_draw_request(window_id);

            window_id
        }
    }

    fn quit(&self) {
        unsafe {
            SDL_Quit();
            ACTUALLY_QUIT.with(|b| b.set(true));
        }
    }

    fn set_cursor(&mut self, cursor: Cursor) {
        let cursor = match cursor {
            Cursor::IBeam => self.ibeam_cursor,
            Cursor::OpenHand => self.open_hand_cursor,
            _ => self.arrow_cursor,
        };
        unsafe {
            SDL_SetCursor(cursor);
        }
    }

    fn hide_cursor(&mut self) {
        unsafe {
            SDL_ShowCursor(SDL_DISABLE);
        }
    }
    fn show_cursor(&mut self) {
        unsafe {
            SDL_ShowCursor(SDL_ENABLE);
        }
    }

    fn raw_window_handle(&self, window_id: WindowId) -> RawWindowHandle {
        unsafe {
            use syswm::*;
            let window = window_id.raw() as *mut SDL_Window;
            let mut info = SDL_SysWMinfo::default();
            version::SDL_VERSION(&mut info.version);
            if SDL_TRUE == SDL_GetWindowWMInfo(window, &mut info) {
                let subsystem = info.subsystem;
                let info = info.info;

                match subsystem {
                    #[cfg(target_os = "macos")]
                    SDL_SYSWM_COCOA => {
                        use raw_window_handle::macos::MacOSHandle;
                        RawWindowHandle::MacOS(MacOSHandle {
                            ns_window: info.cocoa.window as *mut c_void,
                            ns_view: 0 as *mut c_void, // SDL does not provide this.
                            ..MacOSHandle::empty()
                        })
                    }
                    #[cfg(target_os = "windows")]
                    SDL_SYSWM_WINDOWS => RawWindowHandle::Windows(WindowsHandle {
                        hwnd: info.cocoa.window as *mut c_void,
                        ..WindowsHandle::empty()
                    }),
                    #[cfg(any(
                        target_os = "linux",
                        target_os = "dragonfly",
                        target_os = "freebsd",
                        target_os = "netbsd",
                        target_os = "openbsd",
                    ))]
                    SDL_SYSWM_X11 => {
                        use self::raw_window_handle::unix::XlibHandle;
                        RawWindowHandle::Xlib(XlibHandle {
                            window: info.x11.window,
                            display: info.x11.display as *mut c_void,
                            ..XlibHandle::empty()
                        })
                    }
                    #[cfg(any(
                        target_os = "linux",
                        target_os = "dragonfly",
                        target_os = "freebsd",
                        target_os = "netbsd",
                        target_os = "openbsd",
                    ))]
                    SDL_SYSWM_WAYLAND => {
                        use self::raw_window_handle::unix::WaylandHandle;
                        RawWindowHandle::Wayland(WaylandHandle {
                            surface: info.wl.surface as *mut c_void,
                            display: info.wl.display as *mut c_void,
                            ..WaylandHandle::empty()
                        })
                    }
                    #[cfg(any(target_os = "ios"))]
                    SDL_SYSWM_UIKIT => {
                        use self::raw_window_handle::ios::IOSHandle;
                        RawWindowHandle::IOS(IOSHandle {
                            ui_window: info.uikit.window as *mut c_void,
                            ui_view: 0 as *mut c_void, // SDL does not provide this.
                            ..IOSHandle::empty()
                        })
                    }
                    #[cfg(any(target_os = "android"))]
                    SDL_SYSWM_ANDROID => {
                        use self::raw_window_handle::android::AndroidHandle;
                        RawWindowHandle::Android(AndroidHandle {
                            a_native_window: info.android.window as *mut c_void,
                            ..AndroidHandle::empty()
                        })
                    }
                    _ => unimplemented!(),
                }
            } else {
                panic!()
            }
        }
    }

    fn start_text_input(&mut self) {
        unsafe {
            SDL_StartTextInput();
        }
    }

    fn end_text_input(&mut self) {
        unsafe {
            SDL_StopTextInput();
        }
    }

    fn set_text_input_rectangle(
        &mut self,
        _window_id: WindowId,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        let mut rectangle = SDL_Rect {
            x: x as c_int,
            y: y as c_int,
            w: width as c_int,
            h: height as c_int,
        };
        unsafe {
            SDL_SetTextInputRect(&mut rectangle);
        }
    }

    fn get_user_event_sender(&self) -> Self::UserEventSender {
        PlatformUserEventSender
    }
}

// When the application is dropped, quit the program.
impl Drop for PlatformApplication {
    fn drop(&mut self) {
        unsafe {
            SDL_FreeCursor(self.arrow_cursor);
            SDL_FreeCursor(self.ibeam_cursor);
            SDL_FreeCursor(self.open_hand_cursor);
            SDL_Quit();
        }
    }
}

thread_local! {
    static ACTUALLY_QUIT: Cell<bool> = Cell::new(false);
}

fn process_event(callback: &mut Box<dyn FnMut(Event)>, event: &SDL_Event) {
    unsafe {
        match event.type_ {
            SDL_QUIT => callback(Event::QuitRequested),
            SDL_WINDOWEVENT => {
                let window_event = event.window;
                let window_id =
                    WindowId::new(SDL_GetWindowFromID(window_event.windowID) as *mut c_void);
                match window_event.event {
                    SDL_WINDOWEVENT_MINIMIZED => callback(Event::WindowMinimized { window_id }),
                    SDL_WINDOWEVENT_MAXIMIZED => callback(Event::WindowMaximized { window_id }),
                    // There is no SDL_WINDOWEVENT_FULLSCREENED
                    // There is no equivalent to WindowStartResize
                    // There is no equivalent to WindowEndResize
                    // There is no equivalent to WindowScaleChanged
                    SDL_WINDOWEVENT_RESTORED => callback(Event::WindowRestored { window_id }),
                    SDL_WINDOWEVENT_MOVED => callback(Event::WindowMoved {
                        window_id,
                        x: window_event.data1 as u32,
                        y: window_event.data2 as u32,
                    }),
                    SDL_WINDOWEVENT_FOCUS_GAINED => {
                        callback(Event::WindowGainedFocus { window_id })
                    }
                    SDL_WINDOWEVENT_FOCUS_LOST => callback(Event::WindowLostFocus { window_id }),
                    SDL_WINDOWEVENT_CLOSE => callback(Event::WindowCloseRequested { window_id }),
                    // Presently SDL will block during resizing, which isn't ideal and doesn't match the other
                    // `kapp` platforms. There are ways to alleviate it, but investigation is required.
                    SDL_WINDOWEVENT_SIZE_CHANGED => callback(Event::WindowResized {
                        window_id,
                        width: window_event.data1 as u32,
                        height: window_event.data2 as u32,
                    }),
                    _ => {}
                }
            }
            SDL_KEYDOWN | SDL_KEYUP => {
                let keyboard_event = event.key;

                // Are milliseconds the correct units?
                let timestamp = Duration::from_millis(keyboard_event.timestamp as u64);

                let key = scancode_to_key(keyboard_event.keysym.scancode);
                match keyboard_event.type_ {
                    SDL_KEYDOWN => {
                        if keyboard_event.repeat > 0 {
                            callback(Event::KeyRepeat { key, timestamp })
                        } else {
                            callback(Event::KeyDown { key, timestamp })
                        }
                    }
                    SDL_KEYUP => callback(Event::KeyUp { key, timestamp }),
                    _ => {}
                }
            }
            SDL_MOUSEMOTION => {
                let mouse_motion_event = event.motion;

                // Are milliseconds the correct units?
                let timestamp = Duration::from_millis(mouse_motion_event.timestamp as u64);
                let source = match mouse_motion_event.which {
                    SDL_TOUCH_MOUSEID => PointerSource::Touch,
                    _ => PointerSource::Mouse,
                };

                // Do these need to be scaled by the window DPI?
                callback(Event::MouseMotion {
                    delta_x: mouse_motion_event.xrel as f64,
                    delta_y: mouse_motion_event.yrel as f64,
                    timestamp,
                });
                callback(Event::PointerMoved {
                    x: mouse_motion_event.x as f64,
                    y: mouse_motion_event.y as f64,
                    source,
                    timestamp,
                });
            }
            SDL_MOUSEBUTTONDOWN => {
                let event = event.button;

                let source = match event.which {
                    SDL_TOUCH_MOUSEID => PointerSource::Touch,
                    _ => PointerSource::Mouse,
                };

                // Are milliseconds the correct units?
                let timestamp = Duration::from_millis(event.timestamp as u64);
                let button = match event.button as u32 {
                    SDL_BUTTON_LEFT => PointerButton::Primary,
                    SDL_BUTTON_MIDDLE => PointerButton::Auxillary,
                    SDL_BUTTON_RIGHT => PointerButton::Secondary,
                    SDL_BUTTON_X1 => PointerButton::Extra1,
                    SDL_BUTTON_X2 => PointerButton::Extra2,
                    _ => PointerButton::Unknown,
                };

                callback(Event::PointerDown {
                    x: event.x as f64,
                    y: event.y as f64,
                    source,
                    button,
                    timestamp,
                });

                if event.clicks == 2 {
                    callback(Event::DoubleClickDown {
                        x: event.x as f64,
                        y: event.y as f64,
                        button,
                        timestamp,
                    });
                    callback(Event::DoubleClick {
                        x: event.x as f64,
                        y: event.y as f64,
                        button,
                        timestamp,
                    });
                }
            }
            SDL_MOUSEBUTTONUP => {
                let event = event.button;

                let source = match event.which {
                    SDL_TOUCH_MOUSEID => PointerSource::Touch,
                    _ => PointerSource::Mouse,
                };

                // Are milliseconds the correct units?
                let timestamp = Duration::from_millis(event.timestamp as u64);
                let button = match event.button as u32 {
                    SDL_BUTTON_LEFT => PointerButton::Primary,
                    SDL_BUTTON_MIDDLE => PointerButton::Auxillary,
                    SDL_BUTTON_RIGHT => PointerButton::Secondary,
                    SDL_BUTTON_X1 => PointerButton::Extra1,
                    SDL_BUTTON_X2 => PointerButton::Extra2,
                    _ => PointerButton::Unknown,
                };
                callback(Event::PointerUp {
                    x: event.x as f64,
                    y: event.y as f64,
                    source,
                    button,
                    timestamp,
                });
                if event.clicks == 2 {
                    callback(Event::DoubleClickUp {
                        x: event.x as f64,
                        y: event.y as f64,
                        button,
                        timestamp,
                    });
                }
            }
            SDL_MOUSEWHEEL => {
                let event = event.wheel;
                let mut delta_x = event.x as f64;
                let mut delta_y = event.y as f64;

                if event.direction == SDL_MOUSEWHEEL_FLIPPED {
                    delta_x *= -1.0;
                    delta_y *= -1.0;
                }

                let window_id = WindowId::new(SDL_GetWindowFromID(event.windowID) as *mut c_void);
                let timestamp = Duration::from_millis(event.timestamp as u64);

                callback(Event::Scroll {
                    delta_x,
                    delta_y,
                    window_id,
                    timestamp,
                });
            }
            SDL_TEXTINPUT => {
                let c_str = CStr::from_ptr(event.text.text.as_ptr()).to_str().unwrap();
                for character in c_str.chars() {
                    // Send a character received for each key.
                    callback(Event::CharacterReceived { character });
                }
            }
            SDL_TEXTEDITING => {
                let c_str = CStr::from_ptr(event.text.text.as_ptr()).to_str().unwrap();
                callback(Event::IMEComposition {
                    composition: c_str.to_string(),
                });
            }
            SDL_USEREVENT => {
                callback(Event::UserEvent {
                    // This cast is incorrect but probably won't cause issues for now
                    id: event.user.code as usize,
                    data: event.user.data1 as usize
                }); 
            }
            _ => {}
        }
    }
}

pub struct PlatformEventLoop {}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&self, mut callback: Box<dyn FnMut(Event)>) {
        unsafe {
            let mut event = std::mem::zeroed();
            loop {
                if ACTUALLY_QUIT.with(|b| b.get()) {
                    break;
                }
                // Wait for a new event if we don't have any redraw requests
                if redraw_manager::draw_requests_count() == 0 {
                    SDL_WaitEvent(&mut event);
                    process_event(&mut callback, &event);
                }

                // Process all events.
                while SDL_PollEvent(&mut event) != 0 {
                    process_event(&mut callback, &event);
                }

                // When there are no events remaining, we're at the end of the event loop
                callback(Event::EventsCleared);

                // Send a draw event for each window that needs to be drawn.
                redraw_manager::begin_draw_flush();
                while let Some(window_id) = redraw_manager::get_draw_request() {
                    callback(Event::Draw { window_id });
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct PlatformUserEventSender;

impl PlatformUserEventSenderTrait for PlatformUserEventSender {
    fn send(&self, id: usize, data: usize) {
        let mut user_event = SDL_Event {
            user: SDL_UserEvent {
                type_: SDL_USEREVENT,
                // This cast isn't correct, but it probably won't cause any issues right now
                code: id as i32,
                data1: data as *mut c_void,
                ..Default::default()
            },
        };
        unsafe {
            let result = SDL_PushEvent(&mut user_event as *mut _);
        }
    }
}
