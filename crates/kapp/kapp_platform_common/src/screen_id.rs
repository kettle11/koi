/// A unique ID associated per screen.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct ScreenId {
    raw_id: *mut std::ffi::c_void,
}

impl ScreenId {
    /// Constructs a new WindowId
    /// There should never be a reason to call this directly.
    pub fn new(raw_id: *mut std::ffi::c_void) -> Self {
        Self { raw_id }
    }

    /// # Safety
    ///
    /// Returns the raw screen pointer.
    /// On MacOS this is a pointer to the NSScreen object.
    /// On Web this is just '0'
    pub unsafe fn raw(self) -> *mut std::ffi::c_void {
        self.raw_id
    }
}

// raw_id is only used as a unique identifier
// or carefully used on the UI thread if the platform requires it.
unsafe impl Send for ScreenId {}
