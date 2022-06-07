use crate::*;

thread_local! {
    static DATABASE_SETUP: RefCell<bool> = RefCell::new(false);
    static SETUP_DATABASE: JSObjectFromString = JSObjectFromString::new(r#"
        function f() {            
            let openRequest = indexedDB.open("kwasm_db", 1);

            this.kwasm_db = new Promise(function(resolve, reject) {
                openRequest.onsuccess = function(event) {
                    this.kwasm_db = event.target.result;
                    resolve(event);
                };
            });
            openRequest.onerror = event => {
                console.log("Database could not be setup!: ", event);
            };

            // create/upgrade the database without version checks
            openRequest.onupgradeneeded = function(event) {
                this.kwasm_db = event.target.result;
                if (!this.kwasm_db.objectStoreNames.contains('array_buffers')) {
                    this.kwasm_db.createObjectStore('array_buffers', {keyPath: 'id'});
                }
                console.log("Upgrading or creating IndexedDB for the first time");
            };
        }
        f
        "#);

    static ADD_TO_DATABASE: JSObjectFromString = JSObjectFromString::new(r#"
        function f (data_ptr, data_len, fileName_index) {
            let fileName = self.kwasm_get_object(fileName_index);
            const message_data = new Uint8Array(new Uint8Array(self.kwasm_memory.buffer, data_ptr, data_len));

            Promise.resolve(this.kwasm_db).then(function(e) {
                let db = e.target.result;
                let transaction = db.transaction("array_buffers", "readwrite");
    
                let array_buffers = transaction.objectStore("array_buffers");
                let entry = {
                    id: fileName,
                    data: message_data
                };

                let request = array_buffers.put(entry);
                request.onsuccess = function() {
                    //console.log("File added to the store: ", fileName, request.result);
                };
    
                request.onerror = function() {
                    console.log("Error", request.error);
                };
            })
        }
        f
        "#);

    static DELETE_FROM_DATABASE: JSObjectFromString = JSObjectFromString::new(r#"
        function f (fileName) {
            let promise = Promise.resolve(this.kwasm_db).then(function(e) {
                let db = e.target.result;
             
                let transaction = db.transaction("array_buffers", "readwrite");
                let array_buffers = transaction.objectStore("array_buffers");
                let request = array_buffers.delete(fileName);
              
                let promise = new Promise(function(resolve, reject) {
                    request.onsuccess = function(e) {
                        if (e.target.result) {
                            resolve(null);
                        } else {
                            resolve(null);
                        }
                    };
                    request.onerror = function() {
                        console.log("Database Error", request.error);
                        reject(request.error);
                    };
                });
                return promise;
            });
            return promise;
        }
        f
        "#);

    static GET_FROM_DATABASE: JSObjectFromString = JSObjectFromString::new(r#"
        function f (fileName) {
            let promise = Promise.resolve(this.kwasm_db).then(function(e) {
                let db = e.target.result;
             
                let transaction = db.transaction("array_buffers", "readonly");
                let array_buffers = transaction.objectStore("array_buffers");
                let request = array_buffers.get(fileName);
              
                let promise = new Promise(function(resolve, reject) {
                    request.onsuccess = function(e) {
                        if (e.target.result) {
                            resolve(e.target.result.data);
                        } else {
                            resolve(null);
                        }
                    };
                    request.onerror = function() {
                        console.log("Database Error", request.error);
                        reject(request.error);
                    };
                });
                return promise;
            });
            return promise;
        }
        f
        "#);
}

fn setup_database() {
    DATABASE_SETUP.with(|b| {
        let mut b = b.borrow_mut();
        if !*b {
            SETUP_DATABASE.with(|f| f.call());
            *b = true;
        }
    });
}
/// Save bytes to an indexeddb that can later be retrieved by calling 'load_bytes_with_key'
pub fn save_bytes_with_key(key: &str, data: &[u8]) {
    setup_database();

    let js_name = JSString::new(key);

    ADD_TO_DATABASE
        .with(|f| f.call_raw(&[data.as_ptr() as u32, data.len() as u32, js_name.index()]));
}

pub async fn load_bytes_with_key(key: &str) -> Option<Vec<u8>> {
    setup_database();

    let key = key.to_owned();
    let js_future = crate::JSFuture::new(
        move || {
            // This runs on the other thread.
            let key = JSString::new(&key);
            GET_FROM_DATABASE.with(|f| f.call_1_arg(&key).unwrap())
        },
        |js_object| {
            let result = if js_object.is_null() {
                None
            } else {
                crate::libraries::READY_DATA_FOR_TRANSFER.with(|f| f.call_1_arg(&js_object));
                Some(DATA_FROM_HOST.with(|d| d.take()))
            };

            Some(Box::new(result))
        },
    );

    let data = js_future.await;
    let data: Option<Vec<u8>> = *data.downcast().unwrap();
    data
}

pub fn delete_with_key(key: &str) {
    setup_database();
    let js_name = JSString::new(key);
    DELETE_FROM_DATABASE.with(|f| f.call_raw(&[js_name.index()]));
}
