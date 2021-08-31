use crate::*;

thread_local! {
    static FETCH_CALL: JSObjectFromString = JSObjectFromString::new(r#"
        function f(path) {                
            let url = new URL(path, self.kwasm_base_uri).href
            return fetch(url)
                .then(response => {
                    return response.arrayBuffer();
                })
        };
        f
        "#);


    static READY_DATA_FOR_TRANSFER: JSObjectFromString = JSObjectFromString::new(r#"
        function f(result) {
            let pointer = self.kwasm_exports.kwasm_reserve_space(result.byteLength);
            let destination = new Uint8Array(kwasm_memory.buffer, pointer, result.byteLength);
            destination.set(new Uint8Array(result));
        };
        f
        "#);
}

pub async fn fetch(path: &str) -> Result<Vec<u8>, ()> {
    let path = path.to_owned();
    let js_future = crate::JSFuture::new(
        move || {
            // This runs on the other thread.

            let path = JSString::new(&path);
            FETCH_CALL.with(|fetch_call| fetch_call.call_1_arg(&JSObject::NULL, &path).unwrap())
        },
        |js_object| {
            READY_DATA_FOR_TRANSFER.with(|f| f.call_1_arg(&JSObject::NULL, &js_object));
            let result = DATA_FROM_HOST.with(|d| d.take());
            Some(Box::new(result))
        },
    );
    let data = js_future.await;
    let data: Vec<u8> = *data.downcast().unwrap();
    Ok(data)
}

/*
|| FETCH_CALL.with(|fetch_call| fetch_call.deref().clone()),
|js_value| {
    READY_DATA_FOR_TRANSFER.with(|f| f.call_1_arg(&JSObject::NULL, &js_value));
    let result = DATA_FROM_HOST.with(|d| d.take());
    Box::new(result)
},
*/
