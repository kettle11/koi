use crate::*;

thread_local! {
    static CONSOLE_LOG: JSObjectFromString = JSObjectFromString::new("console.log");
    static CONSOLE_ERROR: JSObjectFromString = JSObjectFromString::new("console.error");
}

pub fn log(string: &str) {
    let js_string_object = JSString::new(string);
    log_js_string(&js_string_object);
}

pub fn error(string: &str) {
    let js_string_object = JSString::new(string);
    error_js_string(&js_string_object);
}

pub fn log_js_string(js_string: &JSString) {
    CONSOLE_LOG.with(|f| f.call_1_arg(&JSObject::NULL, js_string));
}

pub fn error_js_string(js_string: &JSString) {
    CONSOLE_ERROR.with(|f| {
        f.call_1_arg(&JSObject::NULL, js_string);
    })
}
