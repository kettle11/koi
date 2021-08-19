use super::external_windows::*;
use super::utils_windows::*;
use std::convert::TryInto;
use std::ptr::{null, null_mut};

use kapp_platform_common::*;

// These should be made into something safe.
pub static mut CURRENT_CURSOR: HCURSOR = null_mut();
pub static mut WINDOWS_TO_REDRAW: Vec<WindowId> = Vec::new();

pub struct PlatformApplication {
    window_class_name: Vec<u16>,
    h_instance: HINSTANCE,
}

pub(crate) struct WindowData {
    pub minimum_width: u32,
    pub minimum_height: u32,
    pub maximum_width: u32,
    pub maximum_height: u32,
}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;
    type UserEventSender = PlatformUserEventSender;

    fn new() -> Self {
        unsafe {
            SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);

            // Register the window class.
            let window_class_name = win32_string("windowing_rust");
            let h_instance = GetModuleHandleW(null_mut());

            let window_class = WNDCLASSW {
                style: CS_DBLCLKS, // Handle double clicks
                lpfnWndProc: Some(super::event_loop_windows::window_callback),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: h_instance,
                hIcon: null_mut(),
                hCursor: null_mut(), // This may not be what is desired. Potentially this makes it annoying to change the cursor later.
                hbrBackground: null_mut(),
                lpszMenuName: null_mut(),
                lpszClassName: window_class_name.as_ptr(),
            };

            CURRENT_CURSOR = LoadCursorW(null_mut(), IDC_ARROW);
            RegisterClassW(&window_class);

            Self {
                window_class_name,
                h_instance,
            }
        }
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {}
    }

    fn set_window_position(&mut self, window_id: WindowId, x: u32, y: u32) {
        unsafe {
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            GetWindowRect(window_id.raw() as HWND, &mut rect);
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            MoveWindow(
                window_id.raw() as HWND,
                x as i32,
                y as i32,
                width,
                height,
                FALSE,
            );
        }
    }
    fn set_window_size(&mut self, window_id: WindowId, width: u32, height: u32) {
        unsafe {
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            GetWindowRect(window_id.raw() as HWND, &mut rect);
            MoveWindow(
                window_id.raw() as HWND,
                rect.left,
                rect.top,
                width as i32,
                height as i32,
                FALSE,
            );
        }
    }
    fn set_window_title(&mut self, window_id: WindowId, title: &str) {
        let title = win32_string(title);
        unsafe {
            SetWindowTextW(window_id.raw() as HWND, title.as_ptr());
        }
    }
    fn minimize_window(&mut self, window_id: WindowId) {
        unsafe {
            ShowWindow(window_id.raw() as HWND, SW_MINIMIZE);
        }
    }
    fn maximize_window(&mut self, window_id: WindowId) {
        unsafe {
            ShowWindow(window_id.raw() as HWND, SW_MAXIMIZE);
        }
    }
    fn fullscreen_window(&mut self, window_id: WindowId) {
        // This implementation is windowless fullscreen, not true fullscreen.
        unsafe {
            let hwnd = window_id.raw() as HWND;
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);
            SetWindowLongPtrW(hwnd, GWL_STYLE, (WS_VISIBLE | WS_POPUP).try_into().unwrap());
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            GetWindowRect(window_id.raw() as HWND, &mut rect);
            MoveWindow(
                window_id.raw() as HWND,
                0,
                0,
                screen_width as i32,
                screen_height as i32,
                FALSE,
            );
        }
    }
    fn restore_window(&mut self, window_id: WindowId) {
        unsafe {
            let hwnd = window_id.raw() as HWND;
            let window_style = WS_OVERLAPPEDWINDOW | WS_VISIBLE | CS_OWNDC;
            SetWindowLongPtrW(hwnd, GWL_STYLE, window_style.try_into().unwrap());
            ShowWindow(window_id.raw() as HWND, SW_RESTORE);
        }
    }
    fn close_window(&mut self, window_id: WindowId) {
        unsafe {
            CloseWindow(window_id.raw() as HWND);
        }
    }

    fn redraw_window(&mut self, window_id: WindowId) {
        redraw_manager::add_draw_request(window_id);
    }

    fn get_window_size(&mut self, window_id: WindowId) -> (u32, u32) {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        unsafe {
            GetClientRect(window_id.raw() as HWND, &mut rect);
        }
        (
            (rect.right - rect.left) as u32,
            (rect.bottom - rect.top) as u32,
        )
    }

    fn get_window_scale(&mut self, window_id: WindowId) -> f64 {
        let dpi = unsafe { GetDpiForWindow(window_id.raw() as HWND) };
        dpi as f64 / USER_DEFAULT_SCREEN_DPI as f64
    }

    fn lock_mouse_position(&mut self) {
        unsafe {
            let mut position = POINT { x: 0, y: 0 };
            GetCursorPos(&mut position);
            let rect = RECT {
                left: position.x,
                top: position.y,
                right: position.x,
                bottom: position.y,
            };
            ClipCursor(&rect);
        }
    }

    fn unlock_mouse_position(&mut self) {
        unsafe {
            ClipCursor(null());
        }
    }

    fn new_window(&mut self, window_parameters: &WindowParameters) -> WindowId {
        unsafe {
            let extended_style = WS_EX_APPWINDOW;
            let window_style = WS_OVERLAPPEDWINDOW | WS_VISIBLE | CS_OWNDC;
            let title = win32_string(&window_parameters.title);

            let (x, y) = if let Some(position) = window_parameters.position {
                (position.0 as i32, position.1 as i32)
            } else {
                (CW_USEDEFAULT, CW_USEDEFAULT)
            };

            let (width, height) =
                window_parameters
                    .size
                    .map_or((CW_USEDEFAULT, CW_USEDEFAULT), |d| {
                        let mut rect = RECT {
                            left: 0,
                            top: 0,
                            right: d.0 as i32,
                            bottom: d.1 as i32,
                        };

                        // Windows will provide a window with a smaller client area than desired (because it includes borders in the window size).
                        // This call returns an adjusted rect accounting for the borders based on the window_style.
                        AdjustWindowRectEx(&mut rect, window_style, FALSE, extended_style);

                        (rect.right - rect.left, rect.bottom - rect.top)
                    });

            let (minimum_width, minimum_height) = window_parameters.minimum_size.unwrap_or((
                GetSystemMetrics(SM_CXMINTRACK) as u32,
                GetSystemMetrics(SM_CYMINTRACK) as u32,
            ));
            let (maximum_width, maximum_height) = window_parameters.maximum_size.unwrap_or((
                GetSystemMetrics(SM_CXMAXTRACK) as u32,
                GetSystemMetrics(SM_CYMAXTRACK) as u32,
            ));
            let window_data = Box::new(WindowData {
                minimum_width,
                minimum_height,
                maximum_width,
                maximum_height,
            });

            let data = Box::leak(window_data) as *mut WindowData as *mut std::ffi::c_void;
            let window_handle = CreateWindowExW(
                extended_style,
                self.window_class_name.as_ptr(),
                title.as_ptr(),
                window_style,
                x as i32,
                y as i32,
                width,
                height,
                null_mut(),
                null_mut(),
                self.h_instance,
                data,
            );

            let window_id = WindowId::new(window_handle as *mut std::ffi::c_void);
            // When a window is created immediately request that it should redraw
            redraw_manager::add_draw_request(window_id);
            WINDOWS_TO_REDRAW.push(window_id); // Send the window an initial Draw event.
            window_id
        }
    }

    fn quit(&self) {
        unsafe {
            PostQuitMessage(0);
        }
    }

    fn set_cursor(&mut self, cursor: Cursor) {
        unsafe {
            // List of cursors here:
            // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursora
            let cursor = match cursor {
                Cursor::Arrow => LoadCursorW(null_mut(), IDC_ARROW),
                Cursor::IBeam => LoadCursorW(null_mut(), IDC_IBEAM),

                // There's no default for this on Windows
                Cursor::PointingHand => LoadCursorW(null_mut(), IDC_ARROW),
                Cursor::OpenHand => LoadCursorW(null_mut(), IDC_HAND),

                // There's no default for this on Windows
                Cursor::ClosedHand => LoadCursorW(null_mut(), IDC_HAND),
            };

            SetCursor(super::application_windows::CURRENT_CURSOR);

            // This is a workaround.
            // The cursor doesn't immediately update because the WM_SETCURSOR event isn't
            // sent immediately. By setting the position here, the mouse moves and WM_SETCURSOR is sent.
            let mut position = POINT { x: 0, y: 0 };
            GetCursorPos(&mut position);
            SetCursorPos(position.x, position.y);

            CURRENT_CURSOR = cursor;
        }
    }
    fn hide_cursor(&mut self) {
        unsafe {
            ShowCursor(FALSE);
        }
    }
    fn show_cursor(&mut self) {
        unsafe {
            ShowCursor(TRUE);
        }
    }

    fn raw_window_handle(&self, window_id: WindowId) -> RawWindowHandle {
        raw_window_handle::RawWindowHandle::Windows(raw_window_handle::windows::WindowsHandle {
            hwnd: unsafe { window_id.raw() },
            hinstance: self.h_instance as *mut std::ffi::c_void,
            ..raw_window_handle::windows::WindowsHandle::empty()
        })
    }

    fn start_text_input(&mut self) {
        todo!()
    }

    fn end_text_input(&mut self) {
        todo!()
    }

    fn set_text_input_rectangle(
        &mut self,
        _window_id: WindowId,
        _x: f64,
        _y: f64,
        _width: f64,
        _height: f64,
    ) {
        todo!()
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
    fn run(&self, callback: Box<dyn FnMut(kapp_platform_common::Event)>) {
        super::event_loop_windows::run(callback);
    }
}

// TODO
pub struct PlatformUserEventSender;

impl PlatformUserEventSenderTrait for PlatformUserEventSender {
    fn send(&self, id: usize, data: usize) {
        todo!()
    }
}