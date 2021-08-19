use kapp_platform_common::*;
use kwasm::*;
use std::convert::TryInto;

thread_local! {
    static KAPP_JS_FUNCTION: JSObjectFromString = JSObjectFromString::new(include_str!("kapp.js").into());
}

fn call_js_function(command: HostCommands) {
    KAPP_JS_FUNCTION.with(|f| f.call_raw(&JSObject::NULL, &[command as u32]));
}

pub struct PlatformApplication {}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;
    type UserEventSender = PlatformUserEventSender;
    fn new() -> Self {
        kwasm::setup_panic_hook();
        call_js_function(HostCommands::SetCallbacks);
        call_js_function(HostCommands::RequestAnimationFrame);
        Self {}
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {}
    }

    fn set_window_position(&mut self, _window_id: WindowId, _x: u32, _y: u32) {}
    fn set_window_size(&mut self, _window_id: WindowId, _width: u32, _height: u32) {}
    fn set_window_title(&mut self, _window_id: WindowId, _title: &str) {}
    fn minimize_window(&mut self, _window_id: WindowId) {}
    fn maximize_window(&mut self, _window_id: WindowId) {}
    fn get_window_size(&mut self, _window_id: WindowId) -> (u32, u32) {
        call_js_function(HostCommands::GetWindowSize);
        kwasm::DATA_FROM_HOST.with(|d| {
            let d = d.borrow();
            (
                f32::from_ne_bytes(d[0..4].try_into().unwrap()) as u32,
                f32::from_ne_bytes(d[4..8].try_into().unwrap()) as u32,
            )
        })
    }
    fn get_window_scale(&mut self, _window_id: WindowId) -> f64 {
        call_js_function(HostCommands::GetDevicePixelRatio);

        kwasm::DATA_FROM_HOST.with(|d| {
            let d = d.borrow();
            let d: [u8; 4] = d[0..4].try_into().unwrap();
            f32::from_ne_bytes(d) as f64
        })
    }
    fn fullscreen_window(&mut self, _window_id: WindowId) {
        todo!()
    }
    fn restore_window(&mut self, _window_id: WindowId) {
        todo!()
    }
    fn close_window(&mut self, _window_id: WindowId) {}
    fn redraw_window(&mut self, _window_id: WindowId) {
        call_js_function(HostCommands::RequestAnimationFrame)
    }

    fn lock_mouse_position(&mut self) {
        call_js_function(HostCommands::LockCursor);
    }

    fn unlock_mouse_position(&mut self) {
        call_js_function(HostCommands::UnlockCursor);
    }

    fn new_window(&mut self, _window_parameters: &WindowParameters) -> WindowId {
        WindowId::new(0 as *mut std::ffi::c_void)
    }

    fn quit(&self) {
        // Does nothing on web
    }

    fn set_cursor(&mut self, _cursor: Cursor) {
        klog::log!("kapp: SET CURSOR NOT IMPLEMENTED YET");
        //  todo!()
    }
    fn hide_cursor(&mut self) {
        klog::log!("kapp: SET CURSOR NOT IMPLEMENTED YET");
        //  todo!()
    }
    fn show_cursor(&mut self) {
        klog::log!("kapp: SET CURSOR NOT IMPLEMENTED YET");
        //  todo!()
    }

    fn raw_window_handle(&self, _window_id: WindowId) -> RawWindowHandle {
        RawWindowHandle::Web(raw_window_handle::web::WebHandle::empty())
    }

    fn start_text_input(&mut self) {}

    fn end_text_input(&mut self) {}

    fn set_text_input_rectangle(
        &mut self,
        _window_id: WindowId,
        _x: f64,
        _y: f64,
        _width: f64,
        _height: f64,
    ) {
        // Perhaps a hidden text input field could be moved to make IME input appear in the right place.
    }

    fn get_user_event_sender(&self) -> Self::UserEventSender {
        PlatformUserEventSender
    }
}

// When the application is dropped, quit the program.
impl Drop for PlatformApplication {
    fn drop(&mut self) {
        self.quit();
    }
}

pub struct PlatformEventLoop {}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&self, callback: Box<dyn FnMut(Event)>) {
        super::event_loop_web::run(callback);
    }
}

#[repr(u32)]
pub(crate) enum HostCommands {
    RequestAnimationFrame = 0,
    // GetCanvasSize = 1,
    SetCallbacks = 2,
    GetDevicePixelRatio = 3,
    GetWindowSize = 4,
    LockCursor = 5,
    UnlockCursor = 6,
}

#[derive(Clone)]
pub struct PlatformUserEventSender;

impl PlatformUserEventSenderTrait for PlatformUserEventSender {
    fn send(&self, id: usize, data: usize) {
        event_receiver::send_event(Event::UserEvent {
            id, data
        });
    }
}