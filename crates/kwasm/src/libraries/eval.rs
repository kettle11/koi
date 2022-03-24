use crate::*;

thread_local! {
    static EVAL_FUNCTION: JSObject = JSObject::NULL;
}

pub fn eval(source: &str) -> Option<JSObject> {
    let source_str: JSString = JSString::new(source);

    EVAL_FUNCTION.with(|e| {
        if e.is_null() {
            e.swap(&JS_SELF.get_property(&"eval"));
        }
        e.call_1_arg(&source_str)
    })
}
