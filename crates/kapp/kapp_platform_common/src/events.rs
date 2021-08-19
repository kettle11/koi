use crate::keys::Key;
use crate::WindowId;
use std::time::Duration;

/// Input and system events.
/// All user input events have timestamps.
/// Timestamps on MacOS and Windows represent time since the computer was turned on.
/// On Web timestamps represent time since the current document was created.
/// Precision of timestamps varies between platforms.

// Event members are ordered by how important the information is.
// f64 is used for all input events.
// u32 is used for window positioning and movements.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
    /// A recommendation for when a window should draw.
    ///
    /// On MacOS `Draw` is sent after `EventsCleared` or in response to
    /// a system event during resizing. While resizing draw will be sent at the screen's
    /// refresh rate.
    ///
    /// On Web `Draw` is triggered by requestAnimationFrame
    ///
    /// On Windows `Draw` is sent at the end of the event loop.
    Draw {
        window_id: WindowId,
    },
    /// A character input as reported by the OS.
    /// This event should be used for text entry instead of `KeyDown`.
    /// Combinations of key presses will produce characters not reported by
    /// `KeyDown`
    CharacterReceived {
        character: char,
    },
    /// The composition status of the IME (Input Method Editor).
    /// This event is fired when a keystroke causes the composition status to change.
    /// For proper IME support, use this data as an "intermediate" visualization of
    /// user input, prepended to the caret position.
    IMEComposition {
        composition: String,
    },
    /// IME composition has ended.
    /// The "intermediate" visualization should be cleared.
    IMEEndComposition,
    // ------------------- Input Events ---------------------
    /// A key is pressed.
    /// For text input use the `CharacterReceived` event instead.
    KeyDown {
        key: Key,
        timestamp: Duration,
    },
    /// A key is released.
    KeyUp {
        key: Key,
        timestamp: Duration,
    },
    /// A repeat of a held key.
    KeyRepeat {
        key: Key,
        timestamp: Duration,
    },
    /// The pointer position has changed.
    /// Reports physical coordinates in relation to the pointer's window
    PointerMoved {
        x: f64,
        y: f64,
        source: PointerSource,
        timestamp: Duration,
    },
    /// How much the mouse has moved regardless of the pointer position.
    /// This event will be sent even if the mouse position is locked.
    MouseMotion {
        delta_x: f64,
        delta_y: f64,
        timestamp: Duration,
    },
    /// A pointer, mouse, touch or, or stylus has been pressed down.
    /// Note that this is sent by multiple web events, not just web's "pointerdown" event.
    PointerDown {
        x: f64,
        y: f64,
        source: PointerSource,
        button: PointerButton,
        timestamp: Duration,
    },
    /// Reports physical coordinates in relation to the pointer's window
    PointerUp {
        x: f64,
        y: f64,
        source: PointerSource,
        button: PointerButton,
        timestamp: Duration,
    },
    /// Occurs when pressing a mouse button twice in quick succession.
    /// On Windows, this event occurs after the second click but before its release.
    /// On MacOS and Web, this event occurs after two click and release pairs in quick succession.
    DoubleClick {
        x: f64,
        y: f64,
        button: PointerButton,
        timestamp: Duration,
    },
    /// Occurs when pressing a mouse button twice in quick succession.
    /// This event occurs after the second click but before its release.
    /// This event should be used to make double clicks feel more responsive.
    /// For the standard behaviour per platform, use `DoubleClick` event instead.
    DoubleClickDown {
        x: f64,
        y: f64,
        button: PointerButton,
        timestamp: Duration,
    },
    /// Occurs when pressing a mouse button twice in quick succession.
    /// This event occurs after two click and release pairs in quick succession.
    /// For the standard behaviour per platform, use `DoubleClick` event instead.
    DoubleClickUp {
        x: f64,
        y: f64,
        button: PointerButton,
        timestamp: Duration,
    },
    /// If delta_x is set it horizontal scrolling from something like a trackpad.
    /// Momentum may be added to this value
    ///
    /// Note that on web this doesn't correspond to "scroll" events and instead
    /// corresponds to "wheel". Web "scroll" events can be triggered by moving the scrollbar
    Scroll {
        delta_x: f64,
        delta_y: f64,
        window_id: WindowId,
        timestamp: Duration,
    },
    /// A number corresponding to a pinch gesture.
    /// Presently only sent on MacOS.
    PinchGesture {
        delta: f64,
        timestamp: Duration,
    },
    // ------------------- Window Events  ---------------------
    WindowMinimized {
        window_id: WindowId,
    },
    /// This event will not be sent on MacOS, only the WindowFullscreened event will be sent.
    WindowMaximized {
        window_id: WindowId,
    },
    WindowFullscreened {
        window_id: WindowId,
    },
    /// A window is 'restored' when it returns from being minimized, maximized, or fullscreened.
    WindowRestored {
        window_id: WindowId,
    },
    /// When the window has begun resizing.
    WindowStartResize {
        window_id: WindowId,
    },
    /// When the window is done resizing.
    WindowEndResize {
        window_id: WindowId,
    },
    /// On web this event is only sent right before a draw event.
    WindowResized {
        width: u32,
        height: u32,
        window_id: WindowId,
    },
    /// Reports the new x and y position for the upper left corner of the window.
    WindowMoved {
        x: u32,
        y: u32,
        window_id: WindowId,
    },
    WindowGainedFocus {
        window_id: WindowId,
    },
    WindowLostFocus {
        window_id: WindowId,
    },
    /// A close is requested for the WindowId.
    /// The Window will not actually close until the associated 'Window' is dropped.
    WindowCloseRequested {
        window_id: WindowId,
    },
    /// When a window moves between monitors the operating system may
    /// report that the window's user interface should be scaled differently.
    /// Multiply this scale by the UI size to properly scale the UI.
    WindowScaleChanged {
        scale: f64,
        window_id: WindowId,
    },
    // ------------------- Application Events  ---------------------
    /// The application is going to quit immediately after this event is processed.
    /// Perform any final cleanup that's necessary. The quit cannot be cancelled.
    Quit,
    /// A quit is requested, but it is up to the program to call quit().
    QuitRequested,
    /// When the event loop sends its last event
    EventsCleared,
    /// An event that can be issued by libraries that aren't `kapp`.
    UserEvent {
        id: usize,
        data: usize,
    },
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum PointerSource {
    Mouse,
    Touch,
    Pen,
    Unknown,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum PointerButton {
    None,
    Primary,
    Secondary,
    Auxillary,
    Extra1,
    Extra2,
    Unknown,
}
