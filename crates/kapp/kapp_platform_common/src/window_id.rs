/// A unique ID associated with a Window.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "kserde", derive(kserde::SerializeDeserialize))]
pub struct WindowId {
    #[cfg_attr(feature = "kserde", skip)]
    raw_id: usize,
}

impl WindowId {
    /// Constructs a new WindowId
    /// There should never be a reason to call this directly.
    pub fn new(raw_id: *mut std::ffi::c_void) -> Self {
        Self {
            raw_id: raw_id as usize,
        }
    }

    /// # Safety
    ///
    /// Returns the raw window pointer.
    /// On MacOS this is a pointer to the NSWindow object.
    /// On Web this is just '0'
    pub unsafe fn raw(self) -> *mut std::ffi::c_void {
        self.raw_id as _
    }
}

// raw_id is only used as a unique identifier
// or carefully used on the UI thread if the platform requires it.
unsafe impl Send for WindowId {}
unsafe impl Sync for WindowId {}
