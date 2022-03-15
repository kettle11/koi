use std::future::Future;

use crate::*;

thread_local! {
    static CHOOSE_FILE: JSObjectFromString = JSObjectFromString::new(r#"
        function f() {
            let promise = new Promise(function(resolve, reject) {
                this.kwasm_input.onchange = function(e) {
                    resolve(e);
                };
            }).then(result => {
                return this.kwasm_input.files[0].arrayBuffer();
            });
            return promise;
        }
        f
        "#);

        static CLICK_INPUT: JSObjectFromString = JSObjectFromString::new(r#"
            function f() {
                this.kwasm_input = document.createElement('input');
                this.kwasm_input.type = 'file';
                this.kwasm_input.click();
            }
            f
        "#);
}

pub struct FileOpenResult {
    pub result: Vec<u8>,
}

// This is OK because JSFutures are always run on the main thread.
// We will only use the JSObjectDynamic on the main thread.
unsafe impl Send for FileOpenResult {}
unsafe impl Sync for FileOpenResult {}

pub fn open_file_picker() -> impl Future<Output = Result<FileOpenResult, ()>> {
    CLICK_INPUT.with(|d| d.call());
    open_file_picker_async()
}

async fn open_file_picker_async() -> Result<FileOpenResult, ()> {
    let js_future = crate::JSFuture::new(
        move || {
            // This runs on the other thread.
            CHOOSE_FILE.with(|v| v.call()).unwrap()
        },
        |js_object| {
            crate::libraries::READY_DATA_FOR_TRANSFER.with(|f| f.call_1_arg(&js_object));
            let result = DATA_FROM_HOST.with(|d| d.take());
            Some(Box::new(FileOpenResult { result }))
        },
    );

    let data = js_future.await;
    let data: FileOpenResult = *data.downcast().unwrap();
    Ok(data)
}
