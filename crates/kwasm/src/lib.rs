//! Kwasm helps Rust code interact with a web-browser host environment
//! in a light-weight and reusable way.
//!
//! Kwasm allows flexible communication with Javascript, but
//! does not attempt to replace all Javascript with Rust.
//! The library also helps facilitate multi-threaded browser code.
//! It can work alongside `wasm-bindgen` or stand-alone.
//! Kwasm uses eval to initialize Javascript code from Rust libraries.

use std::cell::RefCell;
use std::ffi::c_void;
use std::ops::Deref;

pub mod libraries {
    pub mod console;
    pub mod download;
    pub mod eval;
    pub mod fetch;
    pub mod instant;
    pub mod load_image;
    pub mod open_file;
    pub mod store_data;
    pub use console::*;
    pub use download::*;
    pub use eval::*;
    pub use fetch::*;
    pub use instant::*;
    pub use load_image::*;
    pub use open_file::*;
    pub use store_data::*;
}

mod js_object;
mod panic_hook;

pub use js_object::*;

pub mod web_worker;

mod js_future;
pub(crate) use js_future::*;

use std::sync::Once;

use libraries::eval;
pub use panic_hook::setup_panic_hook;

#[cfg_attr(
    feature = "wasm_bindgen_support",
    wasm_bindgen(module = "/js/kwasm.js")
)]
extern "C" {
    pub(crate) fn kwasm_new_string(data: *const u8, data_length: u32) -> u32;
    pub(crate) fn kwasm_free_js_object(object: u32);
    pub(crate) fn kwasm_js_object_property(function_object: u32, property: u32) -> u32;
    pub(crate) fn kwasm_get_js_object_value_u32(object: u32) -> u32;
    pub(crate) fn kwasm_get_js_object_value_f64(object: u32) -> f64;

    pub(crate) fn kwasm_call_js_with_args(
        function_object: u32,
        args_data: *const c_void,
        data_length: u32,
    ) -> u32;
    pub(crate) fn kwasm_call_js_with_args_raw(
        function_object: u32,
        args_data: *const c_void,
        data_length: u32,
    ) -> u32;
    pub(crate) fn kwasm_new_worker(
        entry_point: u32,
        stack_pointer: u32,
        thread_local_storage_pointer: u32,
    );
    pub(crate) fn kwasm_run_promise(entry_point_pointer: u32);
}

thread_local! {
    /// Data sent from the host.
    /// Unique to this Wasm thread.
    pub static DATA_FROM_HOST: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

/// This will return 1 for pages that are not cross-origin isolated, or for browsers
/// that don't support SharedArrayBuffer.
/// See here for more info about Cross Origin Isolation: https://web.dev/cross-origin-isolation-guide/
pub fn available_threads() -> u32 {
    let result = eval(
        r#"
            let result;
            if (!crossOriginIsolated) {
                result = 1;
            } else {
                result = navigator.hardwareConcurrency;
            }
            result
        "#,
    )
    .unwrap();
    result.get_value_u32()
}

#[cfg(feature = "wasm_bindgen_support")]
use wasm_bindgen::prelude::*;

/// Called by the host to reserve scratch space to pass data into kwasm.
/// returns a pointer to the allocated data.
#[no_mangle]
pub extern "C" fn kwasm_reserve_space(space: usize) -> *mut u8 {
    DATA_FROM_HOST.with(|d| {
        let mut d = d.borrow_mut();
        d.clear();
        d.resize(space, 0);
        d.as_mut_ptr()
    })
}

// The main thread needs its thread local storage initialized.
// Web Workers will also use this to allocate their own thread local storage which is deallocated
// when the worker is dropped.
pub(crate) static mut THREAD_LOCAL_STORAGE_SIZE: u32 = 0;
pub(crate) static mut THREAD_LOCAL_STORAGE_ALIGNMENT: u32 = 0;
static THREAD_LOCAL_STORAGE_METADATA_INIT: Once = Once::new();

#[no_mangle]
pub(crate) extern "C" fn kwasm_alloc_thread_local_storage() -> u32 {
    unsafe {
        THREAD_LOCAL_STORAGE_METADATA_INIT.call_once(|| {
            THREAD_LOCAL_STORAGE_SIZE = eval("self.kwasm_exports.__tls_size.value")
                .unwrap()
                .get_value_u32();
            THREAD_LOCAL_STORAGE_ALIGNMENT = eval("self.kwasm_exports.__tls_align.value")
                .unwrap()
                .get_value_u32();
        });

        let thread_local_storage_layout = core::alloc::Layout::from_size_align(
            THREAD_LOCAL_STORAGE_SIZE as usize,
            THREAD_LOCAL_STORAGE_ALIGNMENT as usize,
        )
        .unwrap();
        std::alloc::alloc(thread_local_storage_layout) as u32
    }
}

#[cfg(feature = "wasm_bindgen_support")]
use wasm_bindgen::prelude::*;

/// This is a horrible hack.
/// wasm-bindgen immediately calls main if this isn't here, this gives kwasm a chance
/// to setup and then main can be called from the Javascript side.
/// It'd be nice to remove this.
/// This could be skipped when using `wasm-bindgen` without workers.
#[cfg_attr(feature = "wasm_bindgen_support", wasm_bindgen(start))]
pub fn kwasm_fake_start() {
    #[cfg(feature = "wasm_bindgen_support")]
    initialize_kwasm_for_wasmbindgen();
}

#[cfg(feature = "wasm_bindgen_support")]
fn initialize_kwasm_for_wasmbindgen() {
    use std::sync::Once;
    static THREAD_LOCAL_STORAGE_METADATA_INIT: Once = Once::new();
    THREAD_LOCAL_STORAGE_METADATA_INIT.call_once(|| {
        unsafe {
            // Smuggle out the Wasm instance's exports right from under `wasm-bindgen`'s nose.
            js_sys::eval("self.kwasm_exports = wasm;").unwrap();

            #[cfg_attr(
                feature = "wasm_bindgen_support",
                wasm_bindgen(module = "/js/kwasm.js")
            )]
            extern "C" {
                pub fn kwasm_initialize_wasmbindgen(module: JsValue, function_table: JsValue);
            }
            kwasm_initialize_wasmbindgen(wasm_bindgen::module(), wasm_bindgen::memory());
        }
    });
}

pub struct JSObjectFromString {
    source: String,
    inner_object: JSObject,
}

impl JSObjectFromString {
    pub fn new(source: &str) -> Self {
        Self {
            source: String::from(source),
            inner_object: JSObject::NULL,
        }
    }

    fn check_initialized(&self) {
        if self.inner_object.is_null() {
            self.inner_object.swap(
                &eval(&self.source)
                    .unwrap_or_else(|| panic!("JSObjectFromString code source returned null")),
            )
        }
    }

    /*
    pub fn inner_object(&self) -> &JSObject {
        self.check_initialized();
        &self.inner_object
    }
    */
}

impl Deref for JSObjectFromString {
    type Target = JSObject;
    fn deref(&self) -> &Self::Target {
        self.check_initialized();
        &self.inner_object
    }
}

pub fn get_string_from_host() -> String {
    DATA_FROM_HOST.with(|d| {
        let d = d.take();
        String::from_utf8(d).unwrap()
    })
}
