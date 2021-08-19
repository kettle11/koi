/// This file is a stripped down version of WinAPI with just the parts relevant to kapp.
/// https://github.com/retep998/winapi-rs
///
/// WinAPI is licensed under Apache 2.0 or MIT:
///
/// Copyright (c) 2015-2018 The winapi-rs Developers
///
/// Permission is hereby granted, free of charge, to any person obtaining a copy
/// of this software and associated documentation files (the "Software"), to deal
/// in the Software without restriction, including without limitation the rights
/// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
/// copies of the Software, and to permit persons to whom the Software is
/// furnished to do so, subject to the following conditions:
///
/// The above copyright notice and this permission notice shall be included in all
/// copies or substantial portions of the Software.
///
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
/// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
/// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
/// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
/// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
/// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
/// SOFTWARE.

// Copied from https://github.com/retep998/winapi-rs/blob/77426a9776f4328d2842175038327c586d5111fd/src/macros.rs
macro_rules! DECLARE_HANDLE {
    ($name:ident, $inner:ident) => {
        pub enum $inner {}
        pub type $name = *mut $inner;
    };
}

macro_rules! STRUCT {
    (#[debug] $($rest:tt)*) => (
        STRUCT!{#[cfg_attr(feature = "impl-debug", derive(Debug))] $($rest)*}
    );
    ($(#[$attrs:meta])* struct $name:ident {
        $($field:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] #[derive(Copy)] $(#[$attrs])*
        pub struct $name {
            $(pub $field: $ftype,)+
        }
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
    );
}

macro_rules! FN {
    (stdcall $func:ident($($t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "system" fn($($t,)*) -> $ret>;
    );
    (stdcall $func:ident($($p:ident: $t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "system" fn($($p: $t,)*) -> $ret>;
    );
    (cdecl $func:ident($($t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "C" fn($($t,)*) -> $ret>;
    );
    (cdecl $func:ident($($p:ident: $t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "C" fn($($p: $t,)*) -> $ret>;
    );
}

// Copied from https://github.com/retep998/winapi-rs/blob/0.3/src/shared/ntdef.rs
pub type LONG = c_long;
pub type WCHAR = wchar_t;
pub type LPCWSTR = *const WCHAR;

// Copied from https://github.com/retep998/winapi-rs/blob/0.3/src/lib.rs
pub mod ctypes {
    #[cfg(feature = "std")]
    pub use std::os::raw::c_void;
    // #[cfg(not(feature = "std"))]
    // pub enum c_void {}
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type __int8 = i8;
    pub type __uint8 = u8;
    pub type __int16 = i16;
    pub type __uint16 = u16;
    pub type __int32 = i32;
    pub type __uint32 = u32;
    pub type __int64 = i64;
    pub type __uint64 = u64;
    pub type wchar_t = u16;
}
use ctypes::*;

// Copied from https://github.com/retep998/winapi-rs/blob/0.3/src/shared/basetsd.rs
pub type UINT_PTR = usize;
pub type LONG_PTR = isize;

// Copied from https://github.com/retep998/winapi-rs/blob/0.3/src/shared/minwindef.rs

pub const FALSE: BOOL = 0;
pub const TRUE: BOOL = 1;
pub type DWORD = c_ulong;
pub type BOOL = c_int;
pub type WORD = c_ushort;
pub type LPVOID = *mut std::ffi::c_void;
pub type UINT = c_uint;
pub type WPARAM = UINT_PTR;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;
pub type ATOM = WORD;

#[inline]
pub fn LOWORD(l: DWORD) -> WORD {
    (l & 0xffff) as WORD
}
#[inline]
pub fn HIWORD(l: DWORD) -> WORD {
    ((l >> 16) & 0xffff) as WORD
}

DECLARE_HANDLE! {HINSTANCE, HINSTANCE__}
pub type HMODULE = HINSTANCE;

// copied from https://github.com/retep998/winapi-rs/blob/0.3/src/shared/windef.rs
DECLARE_HANDLE! {HWND, HWND__}
DECLARE_HANDLE! {HICON, HICON__}
DECLARE_HANDLE! {HMENU, HMENU__}
DECLARE_HANDLE! {HBRUSH, HBRUSH__}

DECLARE_HANDLE! {HIMC, HIMC__}

pub type HCURSOR = HICON;
type LPRECT = *mut RECT;
pub type LPPOINT = *mut POINT;

STRUCT! {struct POINT {
    x: LONG,
    y: LONG,
}}

STRUCT! {#[debug] struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}}

STRUCT! {struct CREATESTRUCTA {
    lpCreateParams: LPVOID,
    hInstance: HINSTANCE,
    hMenu: HMENU,
    hwndParent: HWND,
    cy: c_int,
    cx: c_int,
    y: c_int,
    x: c_int,
    style: LONG,
    lpszName: LPCSTR,
    lpszClass: LPCSTR,
    dwExStyle: DWORD,
}}

type LPCSTR = *const i8;

// Copied from https://github.com/retep998/winapi-rs/blob/0.3/src/shared/windowsx.rs
#[inline]
pub fn GET_X_LPARAM(lp: LPARAM) -> c_int {
    LOWORD(lp as DWORD) as c_short as c_int
}
#[inline]
pub fn GET_Y_LPARAM(lp: LPARAM) -> c_int {
    HIWORD(lp as DWORD) as c_short as c_int
}

// Copied from https://github.com/retep998/winapi-rs/blob/0.3/src/um/libloaderapi.rs
extern "system" {
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
}

// Copied from https://github.com/retep998/winapi-rs/blob/0.3/src/um/winuser.rs
FN! {stdcall WNDPROC(
    HWND,
    UINT,
    WPARAM,
    LPARAM,
) -> LRESULT}

pub type LPMSG = *mut MSG;

#[link(name = "user32")]
extern "system" {
    pub fn AdjustWindowRectEx(
        lpRect: LPRECT,
        dwStyle: DWORD,
        bMenu: BOOL,
        dwExStyle: DWORD,
    ) -> BOOL;
    pub fn CloseWindow(hWnd: HWND) -> BOOL;
    pub fn MoveWindow(
        hWnd: HWND,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        bRepaint: BOOL,
    ) -> BOOL;
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    pub fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn PostQuitMessage(nExitCode: c_int);
    pub fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
    pub fn GetCursorPos(lpPoint: LPPOINT) -> BOOL;
    pub fn GetMessageTime() -> LONG;
    pub fn GetSystemMetrics(nIndex: c_int) -> c_int;
    pub fn GetWindowRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    pub fn GetClientRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;
    pub fn PeekMessageW(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;
    pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
    pub fn SetCursor(hCursor: HCURSOR) -> HCURSOR;
    pub fn SetCursorPos(X: c_int, Y: c_int) -> BOOL;
    pub fn ShowCursor(bShow: BOOL) -> c_int;
    pub fn SetWindowLongW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> LONG;
    pub fn GetWindowLongW(hWnd: HWND, nIndex: c_int) -> LONG;
    #[cfg(target_pointer_width = "64")]
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;

    #[cfg(target_pointer_width = "64")]
    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;

    pub fn SetWindowTextW(hWnd: HWND, lpString: LPCWSTR) -> BOOL;
    pub fn TranslateMessage(lpmsg: *const MSG) -> BOOL;
    pub fn GetDpiForWindow(hwnd: HWND) -> UINT;

    pub fn ClipCursor(lpRect: *const RECT);
}

#[link(name = "Imm32")]
extern "system" {
    pub fn ImmGetContext(Arg1: HWND) -> HIMC;
    pub fn ImmReleaseContext(arg1: HWND, arg2: HIMC) -> BOOL;
    pub fn ImmGetCompositionStringW(
        arg1: HIMC,
        arg2: DWORD,
        lpBug: LPVOID,
        dwBufLen: DWORD,
    ) -> LONG;
}

#[cfg(target_pointer_width = "32")]
pub use self::SetWindowLongW as SetWindowLongPtrW;

#[cfg(target_pointer_width = "32")]
pub use self::GetWindowLongW as GetWindowLongPtrW;

#[link(name = "Shcore")]
extern "system" {
    pub fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> HRESULT;
}

pub const USER_DEFAULT_SCREEN_DPI: c_long = 96;
pub const PROCESS_PER_MONITOR_DPI_AWARE: PROCESS_DPI_AWARENESS = 2;
type PROCESS_DPI_AWARENESS = u32;
type HRESULT = c_long;

STRUCT! {struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
}}

STRUCT! {struct WNDCLASSW {
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
}}

STRUCT! {struct MINMAXINFO {
    ptReserved: POINT,
    ptMaxSize: POINT,
    ptMaxPosition: POINT,
    ptMinTrackSize: POINT,
    ptMaxTrackSize: POINT,
}}

pub const WS_OVERLAPPED: DWORD = 0x00000000;
pub const WS_POPUP: DWORD = 0x80000000;

pub const WS_VISIBLE: DWORD = 0x10000000;
pub const WS_CAPTION: DWORD = 0x00C00000;

pub const WS_SYSMENU: DWORD = 0x00080000;
pub const WS_THICKFRAME: DWORD = 0x00040000;

pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;

pub const WS_OVERLAPPEDWINDOW: DWORD =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

pub const WS_EX_APPWINDOW: DWORD = 0x00040000;
pub const CS_OWNDC: UINT = 0x0020;
pub const CS_DBLCLKS: UINT = 0x0008;

#[allow(overflowing_literals)]
pub const CW_USEDEFAULT: c_int = 0x80000000;
pub const GWL_STYLE: c_int = -16;
pub const GWLP_USERDATA: c_int = -21;

pub const IDC_ARROW: LPCWSTR = 32512 as LPCWSTR;
pub const IDC_IBEAM: LPCWSTR = 32513 as LPCWSTR;
pub const IDC_HAND: LPCWSTR = 32649 as LPCWSTR;

pub const PM_REMOVE: UINT = 0x0001;

pub const SM_CXSCREEN: c_int = 0;
pub const SM_CYSCREEN: c_int = 1;
pub const SM_CXMINTRACK: c_int = 34;
pub const SM_CYMINTRACK: c_int = 35;
pub const SM_CXMAXTRACK: c_int = 59;
pub const SM_CYMAXTRACK: c_int = 60;

pub const SIZE_RESTORED: WPARAM = 0;
pub const SIZE_MINIMIZED: WPARAM = 1;
pub const SIZE_MAXIMIZED: WPARAM = 2;

pub const SW_MAXIMIZE: c_int = 3;
pub const SW_MINIMIZE: c_int = 6;
pub const SW_RESTORE: c_int = 9;
pub const VK_CANCEL: c_int = 0x03;
pub const VK_BACK: c_int = 0x08;
pub const VK_TAB: c_int = 0x09;
pub const VK_CLEAR: c_int = 0x0C;
pub const VK_RETURN: c_int = 0x0D;
pub const VK_SHIFT: c_int = 0x10;
pub const VK_CONTROL: c_int = 0x11;
pub const VK_MENU: c_int = 0x12;
pub const VK_PAUSE: c_int = 0x13;
pub const VK_CAPITAL: c_int = 0x14;
pub const VK_KANA: c_int = 0x15;
pub const VK_JUNJA: c_int = 0x17;
pub const VK_FINAL: c_int = 0x18;
pub const VK_HANJA: c_int = 0x19;
pub const VK_ESCAPE: c_int = 0x1B;
pub const VK_CONVERT: c_int = 0x1C;
pub const VK_NONCONVERT: c_int = 0x1D;
pub const VK_ACCEPT: c_int = 0x1E;
pub const VK_MODECHANGE: c_int = 0x1F;
pub const VK_SPACE: c_int = 0x20;
pub const VK_PRIOR: c_int = 0x21;
pub const VK_NEXT: c_int = 0x22;
pub const VK_END: c_int = 0x23;
pub const VK_HOME: c_int = 0x24;
pub const VK_LEFT: c_int = 0x25;
pub const VK_UP: c_int = 0x26;
pub const VK_RIGHT: c_int = 0x27;
pub const VK_DOWN: c_int = 0x28;
pub const VK_SELECT: c_int = 0x29;
pub const VK_PRINT: c_int = 0x2A;
pub const VK_EXECUTE: c_int = 0x2B;
pub const VK_SNAPSHOT: c_int = 0x2C;
pub const VK_INSERT: c_int = 0x2D;
pub const VK_DELETE: c_int = 0x2E;
pub const VK_HELP: c_int = 0x2F;
pub const VK_LWIN: c_int = 0x5B;
pub const VK_RWIN: c_int = 0x5C;
pub const VK_APPS: c_int = 0x5D;
pub const VK_SLEEP: c_int = 0x5F;
pub const VK_NUMPAD0: c_int = 0x60;
pub const VK_NUMPAD1: c_int = 0x61;
pub const VK_NUMPAD2: c_int = 0x62;
pub const VK_NUMPAD3: c_int = 0x63;
pub const VK_NUMPAD4: c_int = 0x64;
pub const VK_NUMPAD5: c_int = 0x65;
pub const VK_NUMPAD6: c_int = 0x66;
pub const VK_NUMPAD7: c_int = 0x67;
pub const VK_NUMPAD8: c_int = 0x68;
pub const VK_NUMPAD9: c_int = 0x69;
pub const VK_MULTIPLY: c_int = 0x6A;
pub const VK_ADD: c_int = 0x6B;
pub const VK_SEPARATOR: c_int = 0x6C;
pub const VK_SUBTRACT: c_int = 0x6D;
pub const VK_DECIMAL: c_int = 0x6E;
pub const VK_DIVIDE: c_int = 0x6F;
pub const VK_F1: c_int = 0x70;
pub const VK_F2: c_int = 0x71;
pub const VK_F3: c_int = 0x72;
pub const VK_F4: c_int = 0x73;
pub const VK_F5: c_int = 0x74;
pub const VK_F6: c_int = 0x75;
pub const VK_F7: c_int = 0x76;
pub const VK_F8: c_int = 0x77;
pub const VK_F9: c_int = 0x78;
pub const VK_F10: c_int = 0x79;
pub const VK_F11: c_int = 0x7A;
pub const VK_F12: c_int = 0x7B;
pub const VK_F13: c_int = 0x7C;
pub const VK_F14: c_int = 0x7D;
pub const VK_F15: c_int = 0x7E;
pub const VK_F16: c_int = 0x7F;
pub const VK_F17: c_int = 0x80;
pub const VK_F18: c_int = 0x81;
pub const VK_F19: c_int = 0x82;
pub const VK_F20: c_int = 0x83;
pub const VK_F21: c_int = 0x84;
pub const VK_F22: c_int = 0x85;
pub const VK_F23: c_int = 0x86;
pub const VK_F24: c_int = 0x87;

pub const VK_NUMLOCK: c_int = 0x90;
pub const VK_SCROLL: c_int = 0x91;
pub const VK_LSHIFT: c_int = 0xA0;
pub const VK_RSHIFT: c_int = 0xA1;
pub const VK_LCONTROL: c_int = 0xA2;
pub const VK_RCONTROL: c_int = 0xA3;
pub const VK_LMENU: c_int = 0xA4;
pub const VK_RMENU: c_int = 0xA5;
pub const VK_BROWSER_BACK: c_int = 0xA6;
pub const VK_BROWSER_FORWARD: c_int = 0xA7;
pub const VK_BROWSER_REFRESH: c_int = 0xA8;
pub const VK_BROWSER_STOP: c_int = 0xA9;
pub const VK_BROWSER_SEARCH: c_int = 0xAA;
pub const VK_BROWSER_FAVORITES: c_int = 0xAB;
pub const VK_BROWSER_HOME: c_int = 0xAC;
pub const VK_VOLUME_MUTE: c_int = 0xAD;
pub const VK_VOLUME_DOWN: c_int = 0xAE;
pub const VK_VOLUME_UP: c_int = 0xAF;
pub const VK_MEDIA_NEXT_TRACK: c_int = 0xB0;
pub const VK_MEDIA_PREV_TRACK: c_int = 0xB1;
pub const VK_MEDIA_STOP: c_int = 0xB2;
pub const VK_MEDIA_PLAY_PAUSE: c_int = 0xB3;
pub const VK_LAUNCH_MAIL: c_int = 0xB4;
pub const VK_LAUNCH_MEDIA_SELECT: c_int = 0xB5;
pub const VK_LAUNCH_APP1: c_int = 0xB6;
pub const VK_LAUNCH_APP2: c_int = 0xB7;
pub const VK_OEM_1: c_int = 0xBA;
pub const VK_OEM_PLUS: c_int = 0xBB;
pub const VK_OEM_COMMA: c_int = 0xBC;
pub const VK_OEM_MINUS: c_int = 0xBD;
pub const VK_OEM_PERIOD: c_int = 0xBE;
pub const VK_OEM_2: c_int = 0xBF;
pub const VK_OEM_3: c_int = 0xC0;

pub const VK_OEM_4: c_int = 0xDB;
pub const VK_OEM_5: c_int = 0xDC;
pub const VK_OEM_6: c_int = 0xDD;
pub const VK_OEM_7: c_int = 0xDE;
pub const VK_OEM_8: c_int = 0xDF;
pub const VK_OEM_102: c_int = 0xE2;

pub const VK_ATTN: c_int = 0xF6;
pub const VK_CRSEL: c_int = 0xF7;
pub const VK_EXSEL: c_int = 0xF8;
pub const VK_EREOF: c_int = 0xF9;
pub const VK_PLAY: c_int = 0xFA;
pub const VK_ZOOM: c_int = 0xFB;

pub const VK_OEM_CLEAR: c_int = 0xFE;
pub const VK_PROCESSKEY: c_int = 0xE5;

pub const WM_SIZE: UINT = 0x0005;
pub const WM_PAINT: UINT = 0x000F;
pub const WM_CLOSE: UINT = 0x0010;
pub const WM_QUIT: UINT = 0x0012;
pub const WM_SETCURSOR: UINT = 0x0020;

pub const WM_ENTERSIZEMOVE: UINT = 0x0231;
pub const WM_EXITSIZEMOVE: UINT = 0x0232;

pub const WM_KEYDOWN: UINT = 0x0100;
pub const WM_KEYUP: UINT = 0x0101;
pub const WM_CHAR: UINT = 0x0102;
pub const WM_SYSKEYDOWN: UINT = 0x0104;
pub const WM_SYSKEYUP: UINT = 0x0105;
pub const WM_IME_STARTCOMPOSITION: UINT = 0x010d;
pub const WM_IME_ENDCOMPOSITION: UINT = 0x010e;
pub const WM_IME_COMPOSITION: UINT = 0x010f;
pub const WM_LBUTTONDOWN: UINT = 0x0201;
pub const WM_LBUTTONUP: UINT = 0x0202;
pub const WM_MBUTTONDOWN: UINT = 0x0207;
pub const WM_MBUTTONUP: UINT = 0x0208;
pub const WM_MOUSEMOVE: UINT = 0x0200;
pub const WM_RBUTTONDOWN: UINT = 0x0204;
pub const WM_RBUTTONUP: UINT = 0x0205;
pub const WM_SIZING: UINT = 0x0214;
pub const WM_XBUTTONDOWN: UINT = 0x020B;
pub const WM_XBUTTONUP: UINT = 0x020C;
pub const WM_LBUTTONDBLCLK: UINT = 0x0203;
pub const WM_MBUTTONDBLCLK: UINT = 0x0209;
pub const WM_RBUTTONDBLCLK: UINT = 0x0206;
pub const WM_XBUTTONDBLCLK: UINT = 0x020D;
pub const WM_DPICHANGED: UINT = 0x02E0;
pub const WM_NCDESTROY: UINT = 0x0082;
pub const WM_GETMINMAXINFO: UINT = 0x0024;
//pub const WM_NCCREATE: UINT = 0x0081;
pub const WM_CREATE: UINT = 0x0001;

pub const XBUTTON1: WORD = 0x0001;
pub const XBUTTON2: WORD = 0x0002;

pub const GCS_COMPSTR: UINT = 8;
