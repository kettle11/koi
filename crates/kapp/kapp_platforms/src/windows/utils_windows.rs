use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::prelude::*;

pub fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}
