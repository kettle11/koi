// This file is a bunch of stuff needed for calling into MacOS code.
use objc::runtime::Class;

pub type c_int = i64;

pub static mut NSResponderClass: *const Class = null();
pub static mut NSViewClass: *const Class = null();
pub static mut NSApplicationClass: *const Class = null();
pub static mut NSCursorClass: *const Class = null();

// By manually querying for Selectors and Classes a ton of generated code is avoided.
pub unsafe fn msg<R>(
    target: *const impl objc::Message,
    sel: *const c_void,
    args: impl objc::MessageArguments,
) -> R
where
    R: core::any::Any,
{
    objc::__send_message(target, Sel::from_ptr(sel), args).unwrap()
}

pub mod Sels {
    use std::ffi::c_void;
    use std::ptr::null;

    pub static mut inLiveResize: *const c_void = null();
    pub static mut contentView: *const c_void = null();
    pub static mut setNeedsDisplay: *const c_void = null();
    pub static mut terminate: *const c_void = null();
    pub static mut run: *const c_void = null();
    pub static mut sharedApplication: *const c_void = null();
    pub static mut setActivationPolicy: *const c_void = null();
    pub static mut new: *const c_void = null();
    pub static mut setDelegate: *const c_void = null();
    pub static mut screen: *const c_void = null();
    pub static mut frame: *const c_void = null();
    pub static mut backingScaleFactor: *const c_void = null();
    pub static mut setFrameTopLeftPoint: *const c_void = null();
    pub static mut setContentSize: *const c_void = null();
    pub static mut setTitle: *const c_void = null();
    pub static mut miniaturize: *const c_void = null();
    pub static mut toggleFullScreen: *const c_void = null();
    pub static mut close: *const c_void = null();
    pub static mut arrowCursor: *const c_void = null();
    pub static mut IBeamCursor: *const c_void = null();
    pub static mut pointingHandCursor: *const c_void = null();
    pub static mut openHandCursor: *const c_void = null();
    pub static mut closedHandCursor: *const c_void = null();
    pub static mut set: *const c_void = null();
    pub static mut unhide: *const c_void = null();
    pub static mut hide: *const c_void = null();
    pub static mut object: *const c_void = null();
    pub static mut windowShouldClose: *const c_void = null();
    pub static mut windowDidMiniaturize: *const c_void = null();
    pub static mut windowDidDeminiaturize: *const c_void = null();
    pub static mut windowDidEnterFullScreen: *const c_void = null();
    pub static mut windowDidExitFullScreen: *const c_void = null();
    pub static mut windowDidMove: *const c_void = null();
    pub static mut windowDidResize: *const c_void = null();
    pub static mut windowWillStartLiveResize: *const c_void = null();
    pub static mut windowDidEndLiveResize: *const c_void = null();
    pub static mut windowDidChangeBackingProperties: *const c_void = null();
    pub static mut windowDidBecomeKey: *const c_void = null();
    pub static mut windowDidResignKey: *const c_void = null();
    pub static mut applicationShouldTerminateAfterLastWindowClosed: *const c_void = null();
    pub static mut applicationShouldTerminate: *const c_void = null();
    pub static mut applicationWillTerminate: *const c_void = null();
    pub static mut window: *const c_void = null();
    pub static mut keyCode: *const c_void = null();
    pub static mut isARepeat: *const c_void = null();
    pub static mut modifierFlags: *const c_void = null();
    pub static mut buttonNumber: *const c_void = null();
    pub static mut scrollingDeltaX: *const c_void = null();
    pub static mut scrollingDeltaY: *const c_void = null();
    pub static mut magnification: *const c_void = null();
    pub static mut magnifyWithEvent: *const c_void = null();
    pub static mut drawRect: *const c_void = null();
    pub static mut acceptsFirstResponder: *const c_void = null();
    pub static mut scrollWheel: *const c_void = null();
    pub static mut otherMouseDown: *const c_void = null();
    pub static mut otherMouseUp: *const c_void = null();
    pub static mut rightMouseDown: *const c_void = null();
    pub static mut rightMouseUp: *const c_void = null();
    pub static mut mouseDown: *const c_void = null();
    pub static mut mouseUp: *const c_void = null();
    pub static mut mouseMoved: *const c_void = null();
    pub static mut mouseDragged: *const c_void = null();
    pub static mut rightMouseDragged: *const c_void = null();
    pub static mut otherMouseDragged: *const c_void = null();
    pub static mut keyDown: *const c_void = null();
    pub static mut keyUp: *const c_void = null();
    pub static mut flagsChanged: *const c_void = null();
    pub static mut timestamp: *const c_void = null();
    pub static mut locationInWindow: *const c_void = null();
    pub static mut clickCount: *const c_void = null();
    pub static mut userInfo: *const c_void = null();
    pub static mut valueForKey: *const c_void = null();
    pub static mut floatValue: *const c_void = null();

    pub fn get_sel(name: &str) -> *const c_void {
        objc::runtime::Sel::register(name).as_ptr()
    }

    pub unsafe fn load_all() {
        clickCount = get_sel("clickCount");
        inLiveResize = get_sel("inLiveResize");
        contentView = get_sel("contentView");
        setNeedsDisplay = get_sel("setNeedsDisplay:");
        terminate = get_sel("terminate:");
        run = get_sel("run");
        sharedApplication = get_sel("sharedApplication");
        setActivationPolicy = get_sel("setActivationPolicy:");
        new = get_sel("new");
        setDelegate = get_sel("setDelegate:");
        screen = get_sel("screen");
        frame = get_sel("frame");
        backingScaleFactor = get_sel("backingScaleFactor");
        setFrameTopLeftPoint = get_sel("setFrameTopLeftPoint:");
        setContentSize = get_sel("setContentSize:");
        setTitle = get_sel("setTitle:");
        miniaturize = get_sel("miniaturize:");
        toggleFullScreen = get_sel("toggleFullScreen:");
        close = get_sel("close");
        arrowCursor = get_sel("arrowCursor");
        IBeamCursor = get_sel("IBeamCursor");
        pointingHandCursor = get_sel("pointingHandCursor");
        openHandCursor = get_sel("openHandCursor");
        closedHandCursor = get_sel("closedHandCursor");
        set = get_sel("set");
        unhide = get_sel("unhide");
        hide = get_sel("hide");
        windowDidMiniaturize = get_sel("windowDidMiniaturize:");
        windowShouldClose = get_sel("windowShouldClose:");
        windowDidDeminiaturize = get_sel("windowDidDeminiaturize:");
        windowDidEnterFullScreen = get_sel("windowDidEnterFullScreen:");
        windowDidExitFullScreen = get_sel("windowDidExitFullScreen:");
        windowDidMove = get_sel("windowDidMove:");
        windowDidResize = get_sel("windowDidResize:");
        windowWillStartLiveResize = get_sel("windowWillStartLiveResize:");
        windowDidEndLiveResize = get_sel("windowDidEndLiveResize:");
        windowDidChangeBackingProperties = get_sel("windowDidChangeBackingProperties:");
        windowDidBecomeKey = get_sel("windowDidBecomeKey:");
        windowDidResignKey = get_sel("windowDidResignKey:");
        applicationShouldTerminateAfterLastWindowClosed =
            get_sel("applicationShouldTerminateAfterLastWindowClosed:");
        applicationShouldTerminate = get_sel("applicationShouldTerminate:");
        applicationWillTerminate = get_sel("applicationWillTerminate:");
        window = get_sel("window");
        keyCode = get_sel("keyCode");
        isARepeat = get_sel("isARepeat");
        modifierFlags = get_sel("modifierFlags");
        object = get_sel("object");
        buttonNumber = get_sel("buttonNumber");
        scrollingDeltaX = get_sel("scrollingDeltaX");
        scrollingDeltaY = get_sel("scrollingDeltaY");
        magnification = get_sel("magnification");
        magnifyWithEvent = get_sel("magnifyWithEvent:");
        drawRect = get_sel("drawRect:");
        acceptsFirstResponder = get_sel("acceptsFirstResponder");
        scrollWheel = get_sel("scrollWheel:");
        otherMouseDown = get_sel("otherMouseDown:");
        otherMouseUp = get_sel("otherMouseUp:");
        rightMouseDown = get_sel("rightMouseDown:");
        rightMouseUp = get_sel("rightMouseUp:");
        mouseDown = get_sel("mouseDown:");
        mouseUp = get_sel("mouseUp:");
        mouseMoved = get_sel("mouseMoved:");
        mouseDragged = get_sel("mouseDragged:");
        rightMouseDragged = get_sel("rightMouseDragged:");
        otherMouseDragged = get_sel("otherMouseDragged:");
        keyDown = get_sel("keyDown:");
        keyUp = get_sel("keyUp:");
        flagsChanged = get_sel("flagsChanged:");
        timestamp = get_sel("timestamp");
        locationInWindow = get_sel("locationInWindow");
        userInfo = get_sel("userInfo");
        valueForKey = get_sel("valueForKey:");
        floatValue = get_sel("floatValue");
    }
}

pub fn get_class(name: &str) -> *const Class {
    unsafe {
        let class = objc::runtime::objc_getClass(name.as_ptr() as *const _) as *const Class;
        if class.is_null() {
            panic!("Could not find: {:?}", name);
        } else {
            class
        }
    }
}

/// Only call this once!
pub(crate) unsafe fn initialize_classes() {
    NSResponderClass = get_class("NSResponder\u{0}");
    NSViewClass = get_class("NSView\u{0}");
    NSApplicationClass = get_class("NSApplication\u{0}");
    NSCursorClass = get_class("NSCursor\u{0}");
    Sels::load_all();
}

pub type c_long = i64;
pub type c_ulong = u64;

use std::ffi::c_void;
use std::os::raw::c_double;
use std::ptr::null;

pub use objc::{
    declare::ClassDecl,
    runtime::{Object, Sel, BOOL, NO, YES},
};

#[link(name = "AppKit", kind = "framework")]
extern "C" {
    pub static NSBackingPropertyOldScaleFactorKey: *const c_void;
}

#[cfg(target_pointer_width = "32")]
pub type NSInteger = c_int;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = c_uint;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = c_long;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = c_ulong;

pub const nil: *mut Object = 0 as *mut Object;

pub const NSTrackingMouseEnteredAndExited: NSInteger = 0x01;
pub const NSTrackingMouseMoved: NSInteger = 0x02;
pub const NSTrackingActiveInKeyWindow: NSInteger = 0x20;
pub const NSTrackingInVisibleRect: NSInteger = 0x200;

pub const NX_DEVICELSHIFTKEYMASK: u64 = 0x2;
pub const NX_DEVICERSHIFTKEYMASK: u64 = 0x4;

pub const NX_DEVICELCTLKEYMASK: u64 = 0x1;
pub const NX_DEVICERCTLKEYMASK: u64 = 0x2000;

pub const NX_DEVICELALTKEYMASK: u64 = 0x20;
pub const NX_DEVICERALTKEYMASK: u64 = 0x40;

pub const NX_DEVICELCMDKEYMASK: u64 = 0x8;
pub const NX_DEVICERCMDKEYMASK: u64 = 0x10;

pub const NSTerminateNow: NSUInteger = 1;
pub const NSTerminateCancel: NSUInteger = 0;

pub const NSEventModifierFlagCapsLock: NSUInteger = 1 << 16;

pub const kCFRunLoopBeforeWaiting: CFRunLoopActivity = 1 << 5;

pub const NSWindowStyleMaskTitled: NSUInteger = 1;
pub const NSWindowStyleMaskClosable: NSUInteger = 1 << 1;
pub const NSWindowStyleMaskMiniaturizable: NSUInteger = 1 << 2;
pub const NSWindowStyleMaskResizable: NSUInteger = 1 << 3;
pub const NSWindowStyleMaskFullSizeContentView: NSUInteger = 1 << 15;

pub const NSBackingStoreBuffered: NSUInteger = 2;
pub const UTF8_ENCODING: usize = 4;

#[repr(i64)]
pub enum NSApplicationActivationPolicy {
    NSApplicationActivationPolicyRegular = 0,
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub fn CFRunLoopGetMain() -> CFRunLoopRef;
    pub fn CFRunLoopWakeUp(rl: CFRunLoopRef);

    pub static kCFRunLoopCommonModes: CFRunLoopMode;

    pub fn CFRunLoopObserverCreate(
        allocator: CFAllocatorRef,
        activities: CFOptionFlags,
        repeats: BOOL,
        order: CFIndex,
        callout: CFRunLoopObserverCallBack,
        context: *const CFRunLoopObserverContext,
    ) -> CFRunLoopObserverRef;
    pub fn CFRunLoopAddObserver(
        rl: CFRunLoopRef,
        observer: CFRunLoopObserverRef,
        mode: CFRunLoopMode,
    );

    pub fn CFRunLoopSourceCreate(
        allocator: CFAllocatorRef,
        order: CFIndex,
        context: *mut CFRunLoopSourceContext,
    ) -> CFRunLoopSourceRef;
    pub fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFRunLoopMode);
    #[allow(dead_code)]
    pub fn CFRunLoopSourceInvalidate(source: CFRunLoopSourceRef);
    // pub fn CFRunLoopSourceSignal(source: CFRunLoopSourceRef);
}

extern "C" {
    // pub fn CGWarpMouseCursorPosition(new_cursor_position: CGPoint) -> i32;
    pub fn CGAssociateMouseAndMouseCursorPosition(connected: bool) -> i32;
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct CFRunLoopSourceContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<extern "C" fn(*const c_void) -> *const c_void>,
    pub release: Option<extern "C" fn(*const c_void)>,
    pub copyDescription: Option<extern "C" fn(*const c_void) -> CFStringRef>,
    pub equal: Option<extern "C" fn(*const c_void, *const c_void) -> BOOL>,
    pub hash: Option<extern "C" fn(*const c_void) -> CFHashCode>,
    pub schedule: Option<extern "C" fn(*mut c_void, CFRunLoopRef, CFRunLoopMode)>,
    pub cancel: Option<extern "C" fn(*mut c_void, CFRunLoopRef, CFRunLoopMode)>,
    pub perform: Option<extern "C" fn(*mut c_void)>,
}

pub type CFHashCode = c_ulong;
pub enum CFRunLoopSource {}
pub type CFRunLoopSourceRef = *mut CFRunLoopSource;

pub enum CFAllocator {}
pub type CFAllocatorRef = *mut CFAllocator;
pub enum CFRunLoop {}
pub type CFRunLoopRef = *mut CFRunLoop;
pub type CFRunLoopMode = CFStringRef;
pub enum CFRunLoopObserver {}
pub type CFRunLoopObserverRef = *mut CFRunLoopObserver;

pub type CFStringRef = *const Object; // CFString
pub type CFIndex = std::os::raw::c_long;
pub type CFOptionFlags = std::os::raw::c_ulong;
pub type CFRunLoopActivity = CFOptionFlags;

pub type CFRunLoopObserverCallBack =
    extern "C" fn(observer: CFRunLoopObserverRef, activity: CFRunLoopActivity, info: *mut c_void);

// https://developer.apple.com/documentation/corefoundation/cfrunloopobservercontext?language=objc
#[repr(C)]
pub struct CFRunLoopObserverContext {
    pub copyDescription: *const c_void,
    pub info: *const c_void,
    pub release: *const c_void,
    pub version: CFIndex,
    pub retain: *const c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

impl CGPoint {
    pub fn new(x: CGFloat, y: CGFloat) -> Self {
        Self { x, y }
    }
}

pub type NSPoint = CGPoint;

pub type CGFloat = c_double;

#[repr(C)]
#[derive(Clone)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

#[repr(C)]
#[derive(Clone)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    pub fn new(width: CGFloat, height: CGFloat) -> Self {
        Self { width, height }
    }
}

pub type NSSize = CGSize;

impl CGRect {
    pub fn new(origin: CGPoint, size: CGSize) -> Self {
        Self { origin, size }
    }
}

unsafe impl objc::Encode for CGRect {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{CGRect={}{}}}",
            NSPoint::encode().as_str(),
            NSSize::encode().as_str()
        );
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

unsafe impl objc::Encode for CGPoint {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{CGPoint={}{}}}",
            CGFloat::encode().as_str(),
            CGFloat::encode().as_str()
        );
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

unsafe impl objc::Encode for CGSize {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{CGSize={}{}}}",
            CGFloat::encode().as_str(),
            CGFloat::encode().as_str()
        );
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

pub type NSRect = CGRect;

pub struct NSString {
    pub raw: *mut Object,
}

impl NSString {
    pub fn new(string: &str) -> Self {
        unsafe {
            let raw: *mut Object = msg_send![class!(NSString), alloc];
            let raw: *mut Object = msg_send![
                raw,
                initWithBytes: string.as_ptr()
                length: string.len()
                encoding:UTF8_ENCODING as *mut Object
            ];

            Self { raw }
        }
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        unsafe {
            let () = msg_send![self.raw, release];
        }
    }
}

#[repr(C)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

unsafe impl objc::Encode for NSRange {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{NSRange={}{}}}",
            NSUInteger::encode().as_str(),
            NSUInteger::encode().as_str(),
        );
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}
