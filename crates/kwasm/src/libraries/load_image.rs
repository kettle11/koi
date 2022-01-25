use crate::*;

thread_local! {
    static LOAD_IMAGE_CALL: JSObjectFromString = JSObjectFromString::new(r#"
        function f(path) {                
            let url = new URL(path, self.kwasm_base_uri).href
            return fetch(url)
                .then(response => response.blob()).
                then(blob => createImageBitmap(blob))
        };
        f
        "#);
}

struct JSObjectCarrier(JSObjectDynamic);

// This is OK because JSFutures are always run on the main thread.
// We will only use the JSObjectDynamic on the main thread.
unsafe impl Send for JSObjectCarrier {}
unsafe impl Sync for JSObjectCarrier {}

pub async fn load_image(path: &str) -> Result<JSObjectDynamic, ()> {
    let path = path.to_owned();
    let js_future = crate::JSFuture::new(
        move || {
            // This runs on the other thread.
            let path = JSString::new(&path);
            LOAD_IMAGE_CALL.with(|fetch_call| fetch_call.call_1_arg(&path).unwrap())
        },
        |js_object| Some(Box::new(JSObjectCarrier(js_object))),
    );
    let data = js_future.await;
    let data: Box<JSObjectCarrier> = *data.downcast().unwrap();
    Ok(data.0)
}
