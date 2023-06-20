use std::sync::Mutex;

use crate::*;

thread_local! {
    static REGISTER_HANDLER: JSObjectFromString = JSObjectFromString::new(r#"
        function f() {
            document.body.ondragenter = document.body.ondragover = function(event) {
                event.stopPropagation();
                event.preventDefault();
            };
            document.body.ondrop = function(event) {
                event.stopPropagation();
                event.preventDefault();
                for (file of event.dataTransfer.files) {
                    file.arrayBuffer().then((result) => {
                        self.kwasm_pass_string_to_client(file.name);
                        self.kwasm_exports.drag_and_drop_file_name();

                        let pointer = self.kwasm_exports.kwasm_reserve_space(result.byteLength);
                        let destination = new Uint8Array(kwasm_memory.buffer, pointer, result.byteLength);
                        destination.set(new Uint8Array(result));
                        self.kwasm_exports.drag_and_drop_file();
                    });
                }
            };
        };
        f
        "#);
}

static FILE_NAME: Mutex<String> = Mutex::new(String::new());
static HANDLER: Mutex<Option<Box<dyn FnMut(&str, Vec<u8>) + Send>>> = Mutex::new(None);

#[no_mangle]
extern "C" fn drag_and_drop_file_name() {
    crate::DATA_FROM_HOST.with(|d| {
        let d = d.borrow();
        let file_name = std::str::from_utf8(&d).unwrap();
        *FILE_NAME.lock().unwrap() = file_name.into();
    });
}

#[no_mangle]
extern "C" fn drag_and_drop_file() {
    crate::DATA_FROM_HOST.with(|d| {
        let d = d.take();
        HANDLER
            .lock()
            .unwrap()
            .as_mut()
            .map(|h| (h)(FILE_NAME.lock().unwrap().deref(), d));
    });
}

pub fn register_drag_and_drop_handler(handler: impl FnMut(&str, Vec<u8>) + Send + 'static) {
    *HANDLER.lock().unwrap() = Some(Box::new(handler));
    REGISTER_HANDLER.with(|f| f.call());
}
