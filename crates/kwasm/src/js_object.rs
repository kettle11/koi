use crate::*;
use std::{borrow::Borrow, cell::Cell, ffi::c_void, ops::Deref, rc::Rc};

#[cfg(feature = "wasm_bindgen_support")]
use wasm_bindgen::prelude::*;

fn kwasm_call_js_with_args0(function_object: u32, args: &[u32]) -> u32 {
    unsafe {
        kwasm_call_js_with_args(
            function_object,
            args.as_ptr() as *const c_void,
            args.len() as u32,
        )
    }
}

fn kwasm_call_js_with_args_raw0(function_object: u32, args: &[u32]) -> u32 {
    unsafe {
        kwasm_call_js_with_args_raw(
            function_object,
            args.as_ptr() as *const c_void,
            args.len() as u32,
        )
    }
}

/// Window.self
/// Accesses the global scope.
/// https://developer.mozilla.org/en-US/docs/Web/API/Window/self
pub const JS_SELF: JSObject = JSObject(Cell::new(1));

// This is used to avoid the Rc for the many cases where null is returned.
#[derive(Debug)]
enum JSObjectDynamicInner {
    Weak(JSObject),
    Strong(Rc<JSObject>),
}

impl Clone for JSObjectDynamicInner {
    fn clone(&self) -> Self {
        match self {
            JSObjectDynamicInner::Weak(js_object) => {
                JSObjectDynamicInner::Weak(JSObject(js_object.0.clone()))
            }
            JSObjectDynamicInner::Strong(js_object) => {
                JSObjectDynamicInner::Strong(js_object.clone())
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct JSObjectDynamic(JSObjectDynamicInner);

impl JSObjectDynamic {
    pub const NULL: Self = JSObjectDynamic(JSObjectDynamicInner::Weak(JSObject::NULL));

    /// Leaks the value if there's only one reference to it, otherwise panics.
    pub unsafe fn leak(self) -> u32 {
        let index = self.index();
        match self.0 {
            JSObjectDynamicInner::Strong(js_object) => {
                std::mem::forget(Rc::try_unwrap(js_object).unwrap())
            }
            _ => {}
        }
        index
    }
}

impl Deref for JSObjectDynamic {
    type Target = JSObject;
    fn deref(&self) -> &Self::Target {
        match &self.0 {
            JSObjectDynamicInner::Weak(js_object) => js_object,
            JSObjectDynamicInner::Strong(js_object) => js_object,
        }
    }
}

#[derive(Debug)]
pub struct JSObject(Cell<u32>);

#[derive(Debug, Clone)]
pub struct JSObjectInner(u32);

impl JSObject {
    pub const NULL: Self = JSObject(Cell::new(0));

    pub unsafe fn leak(self) -> u32 {
        let inner = self.index();
        std::mem::forget(self);
        inner
    }

    pub fn get_property(&self, string: &str) -> JSObject {
        let string = JSString::new(string);
        unsafe {
            JSObject(Cell::new(kwasm_js_object_property(
                self.index(),
                string.index(),
            )))
        }
    }

    pub fn to_dynamic(self) -> JSObjectDynamic {
        JSObjectDynamic(JSObjectDynamicInner::Strong(Rc::new(self)))
    }

    pub fn null() -> JSObject {
        JSObject(Cell::new(0))
    }

    pub fn is_null(&self) -> bool {
        self.index() == 0
    }

    pub fn index(&self) -> u32 {
        self.0.borrow().get()
    }

    // If this value is a u32, return it as a u32
    pub fn get_value_u32(&self) -> u32 {
        unsafe { kwasm_get_js_object_value_u32(self.index()) }
    }

    // If this value is a f64, return it as a f64
    pub fn get_value_f64(&self) -> f64 {
        unsafe { kwasm_get_js_object_value_f64(self.index()) }
    }

    /// Replaces the inner JSObject with the new JSObject.
    pub fn swap(&self, object: &JSObject) {
        self.0.swap(&object.0)
    }

    pub unsafe fn new_raw(index: u32) -> JSObject {
        JSObject(Cell::new(index))
    }

    #[inline]
    fn check_result(result: u32) -> Option<JSObject> {
        if result == 0 {
            None
        } else {
            Some(JSObject(Cell::new(result)))
        }
    }

    /// Call a function with each u32 passed as a separate argument to the JavaScript side.
    pub fn call_raw(&self, args: &[u32]) -> Option<JSObject> {
        let result = kwasm_call_js_with_args_raw0(self.index(), args);
        Self::check_result(result)
    }

    /// Call this as a function with one arg.
    pub fn call(&self) -> Option<JSObject> {
        let result = kwasm_call_js_with_args0(self.index(), &[]);
        Self::check_result(result)
    }

    /// Call this as a function with one arg.
    pub fn call_1_arg(&self, argument: &JSObject) -> Option<JSObject> {
        let result = kwasm_call_js_with_args0(self.index(), &[argument.index()]);

        Self::check_result(result)
    }

    /// Call this as a function with one arg.
    pub fn call_2_arg(&self, arg0: &JSObject, arg1: &JSObject) -> Option<JSObject> {
        let result = kwasm_call_js_with_args0(self.index(), &[arg0.index(), arg1.index()]);

        Self::check_result(result)
    }
}

impl Drop for JSObject {
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { kwasm_free_js_object(self.index()) }
        }
    }
}

pub struct JSString {
    // string: &'a str,
    js_object: JSObject,
}

impl JSString {
    pub fn new(string: &str) -> Self {
        let js_object =
            unsafe { JSObject::new_raw(kwasm_new_string(string.as_ptr(), string.len() as u32)) };

        JSString { js_object }
    }
}

impl Deref for JSString {
    type Target = JSObject;
    fn deref(&self) -> &Self::Target {
        &self.js_object
    }
}
