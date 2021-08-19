use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::os::windows::prelude::*;
pub fn error_if_false(i: i32) -> Result<(), Error> {
    if i == 0 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn error_if_null<T>(pointer: *const T) -> Result<(), Error> {
    if pointer.is_null() {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}
