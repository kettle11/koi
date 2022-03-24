use crate::*;

thread_local! {
    static LOAD_IMAGE_CALL: JSObjectFromString = JSObjectFromString::new(r#"
        function f(path) {                
            let url = new URL(path, self.kwasm_base_uri).href
            return fetch(url)
                .then(response => response.blob()).
                then(blob => {
                    return createImageBitmap(blob, {colorSpaceConversion: "none"});
                })
        };
        f
        "#);
}

pub struct ImageLoadResult {
    pub image_js_object: JSObject,
    pub width: u32,
    pub height: u32,
}

// This is OK because JSFutures are always run on the main thread.
// We will only use the JSObjectDynamic on the main thread.
unsafe impl Send for ImageLoadResult {}
unsafe impl Sync for ImageLoadResult {}

pub async fn load_image(path: &str) -> Result<ImageLoadResult, ()> {
    let path = path.to_owned();
    let js_future = crate::JSFuture::new(
        move || {
            // This runs on the other thread.
            let path = JSString::new(&path);
            LOAD_IMAGE_CALL.with(|fetch_call| fetch_call.call_1_arg(&path).unwrap())
        },
        |js_object| {
            let width = js_object.get_property("width").get_value_u32();
            let height = js_object.get_property("height").get_value_u32();

            Some(Box::new(ImageLoadResult {
                image_js_object: js_object,
                width,
                height,
            }))
        },
    );
    let data = js_future.await;
    let data: ImageLoadResult = *data.downcast().unwrap();
    Ok(data)
}
