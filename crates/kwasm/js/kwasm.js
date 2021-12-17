function kwasm_stuff() {
    // This is used to decode strings passed from Wasm to Javascript.
    const decoder = new TextDecoder();
    const encoder = new TextEncoder();

    var kwasm_js_objects = [null, self];
    var kwasm_js_objects_free_indices = [];

    let kwasm_workers = [];
    if (typeof document !== 'undefined') {
        self.kwasm_base_uri = document.baseURI;
    }

    self.kwasm_get_object = function (index) {
        return kwasm_js_objects[index];
    }

    self.kwasm_new_js_object = function (item) {
        let index = kwasm_js_objects_free_indices.pop();
        if (!index) {
            return kwasm_js_objects.push(item) - 1;
        } else {
            kwasm_js_objects[index] = item;
            return index;
        }
    };

    self.kwasm_pass_string_to_client = function (string) {
        // Unfortunately TextEncoder can't write directly to Wasm memory (yet).
        // See this issue: https://github.com/whatwg/encoding/issues/172
        const string_data = encoder.encode(string);
        let length = string_data.byteLength;
        let pointer = self.kwasm_exports.kwasm_reserve_space(length);
        const client_string = new Uint8Array(self.kwasm_memory.buffer, pointer, length);
        client_string.set(string_data);
    };

    self.kwasm_free_js_object = function (index) {
        if (index > 1) {
            kwasm_js_objects[index] = null;
            kwasm_js_objects_free_indices.push(index);
        }
    }

    let kwasm_import_functions = {
        kwasm_free_js_object: function (index) {
            self.kwasm_free_js_object(index);
        },
        kwasm_new_string: function (data, data_length) {
            const message_data = new Uint8Array(self.kwasm_memory.buffer, data, data_length);
            const decoded_string = decoder.decode(new Uint8Array(message_data));
            return self.kwasm_new_js_object(decoded_string);
        },
        // Calls a function but directly passes the u32 args instead of turning
        // them into JS objects first.
        // This expects that the function will return a u32.
        kwasm_call_js_with_args_raw: function (function_object, arg_data_ptr, args_length) {
            const args = new Uint32Array(self.kwasm_memory.buffer, arg_data_ptr, args_length);
            let f = kwasm_js_objects[function_object];
            let result = f.apply(self, args);
            if (result == undefined) {
                return 0;
            } else {
                return self.kwasm_new_js_object(result);
            }
            result
        },
        kwasm_call_js_with_args: function (function_object, arg_data_ptr, args_length) {
            const args = new Uint32Array(self.kwasm_memory.buffer, arg_data_ptr, args_length);
            let f = kwasm_js_objects[function_object];
            // Convert to Array first because Uint32Array's version of map
            // expects a typed array as the return value.
            let args0 = Array.from(args);
            let args1 = args0.map(a => kwasm_js_objects[a]);
            let result = f.apply(self, args1);
            if (result == undefined) {
                return 0;
            } else {
                return self.kwasm_new_js_object(result);
            }
        },
        kwasm_js_object_property: function (object_index, property_name_index) {
            let object = kwasm_js_objects[object_index];
            let property_name = kwasm_js_objects[property_name_index];
            let property_object = object[property_name];
            if (property_object == undefined) {
                console.log(object + " does not have property: " + property_name);
                return 0;
            } else {
                return self.kwasm_new_js_object(property_object);
            }
        },
        // Returns this value as a u32
        kwasm_get_js_object_value_u32: function (object_index) {
            return kwasm_js_objects[object_index];
        },
        // Returns this value as a f64
        kwasm_get_js_object_value_f64: function (object_index) {
            return kwasm_js_objects[object_index];
        },
        kwasm_new_worker: function (entry_point, stack_pointer, thread_local_storage_pointer) {
            let worker = new Worker(kwasm_stuff_blob);

            // This does nothing, but without it Firefox / Safari seem to do some sort of 
            // fault optimization that incorrectly sets up or kills the worker early.
            kwasm_workers.push(worker);

            worker.postMessage({
                kwasm_memory: self.kwasm_memory,
                kwasm_module: self.kwasm_module,
                entry_point: entry_point,
                stack_pointer: stack_pointer,
                thread_local_storage_pointer: thread_local_storage_pointer,
                kwasm_base_uri: document.baseURI,
            });
            worker.onmessage = function (e) {
                if (e.data.promise_inner_future_ptr) {
                    run_future(e.data.promise_inner_future_ptr);
                }
            }
        },
        kwasm_run_promise: function (promise_inner_future_ptr) {
            if (self.kwasm_is_worker) {
                // Ask the main thread to run this
                postMessage({
                    promise_inner_future_ptr: promise_inner_future_ptr
                });
            } else {
                run_future(promise_inner_future_ptr);
            }
        },
    };

    function run_future(promise_inner_future_ptr) {
        let function_to_run_index = self.kwasm_exports.kwasm_promise_begin(promise_inner_future_ptr);
        let function_to_run = self.kwasm_get_object(function_to_run_index);

        function_to_run.then((result) => {
            let result_js_object = self.kwasm_new_js_object(result);
            self.kwasm_exports.kwasm_promise_complete(promise_inner_future_ptr, result_js_object);
            self.kwasm_free_js_object(function_to_run);
        }, rejected => {
        });
        return;
    }

    // Load and setup the WebAssembly library.
    // This is called when using `kwasm` without wasm-bindgen.
    function initialize(wasm_library_path) {
        let imports = {
            env: {}
        };

        imports.env = Object.assign(imports.env, kwasm_import_functions);

        fetch(wasm_library_path).then(response =>
            response.arrayBuffer()
        ).then(bytes => {
            // 5 is arbitrary here
            let shared_memory_supported = typeof SharedArrayBuffer !== 'undefined';
            console.log("Shared memory supported: " + shared_memory_supported);

            // Start with a large amount of memory to avoid issues in Safari / Firefox with grow.
            // It seems grow fails if called from another thread, so this solution isn't exceptionally robust.
            // 5 is arbitrary here
            let starting_mem = Math.max(12800, (bytes.byteLength / 65536) + 5);

            if (shared_memory_supported) {
                self.kwasm_memory = new WebAssembly.Memory({ initial: starting_mem, maximum: 16384 * 4, shared: true });
            } else {
                self.kwasm_memory = new WebAssembly.Memory({ initial: starting_mem, maximum: 16384 * 4 });
            }
            imports.env.memory = self.kwasm_memory;
            return WebAssembly.instantiate(bytes, imports)
        }).then(results => {
            // If this module exports memory use that instead.
            if (results.instance.exports.memory) {
                self.kwasm_memory = results.instance.exports.memory;
            }
            self.kwasm_exports = results.instance.exports;
            self.kwasm_module = results.module;

            // Setup thread-local storage for the main thread
            if (kwasm_exports.kwasm_alloc_thread_local_storage) {
                const thread_local_storage = kwasm_exports.kwasm_alloc_thread_local_storage();
                self.kwasm_exports.__wasm_init_tls(thread_local_storage);
            }

            // Call our start function.
            results.instance.exports.main();
        });
    }

    // If we're a worker thread we'll use this to setup.
    onmessage = function (e) {
        self.kwasm_is_worker = true;
        self.kwasm_base_uri = e.data.kwasm_base_uri;
        let imports = {
            env: {}
        };
        imports.env = Object.assign(imports.env, kwasm_import_functions);

        let memory_assigned = false;

        // Fill in all wasm-bindgen functions with a placeholder.
        // This isn't great because it means that `wasm-bindgen` things
        // won't work in worker threads.
        WebAssembly.Module.imports(e.data.kwasm_module).forEach(item => {
            if (imports[item.module] === undefined) {
                imports[item.module] = {};
            }
            if (item.kind == "function" && !(item.name in imports[item.module])) {
                imports[item.module][item.name] = function () {
                    console.log(item.name + "is unimplemented in worker thread.");
                }
            }
            if (item.kind == "memory") {
                imports[item.module][item.name] = e.data.kwasm_memory;
                memory_assigned = true;
            }
        });

        if (!memory_assigned) {
            imports.env = {
                memory: e.data.kwasm_memory
            };
        }

        self.kwasm_memory = e.data.kwasm_memory;

        WebAssembly.instantiate(e.data.kwasm_module, imports).then(results => {
            self.kwasm_exports = results.exports;

            if (self.kwasm_exports.__wbindgen_start) {
                self.kwasm_exports.__wbindgen_start();
            } else {
                self.kwasm_exports.set_stack_pointer(e.data.stack_pointer);
                self.kwasm_exports.__wasm_init_tls(e.data.thread_local_storage_pointer);
            }
            if (e.data.entry_point) {
                self.kwasm_exports.kwasm_web_worker_entry_point(e.data.entry_point);
                console.error("FINISHED WASM WORKER THREAD");
            }
        });
    }

    kwasm_import_functions.initialize = initialize;

    return kwasm_import_functions;
}

const kwasm = kwasm_stuff();
var kwasm_stuff_blob = URL.createObjectURL(new Blob(
    ['(', kwasm_stuff.toString(), ')()'],
    { type: 'application/javascript' }
));

export default kwasm.initialize;

// The rest of the code here is to accommodate wasm-bindgen binding.
const kwasm_free_js_object = kwasm.kwasm_free_js_object;
const kwasm_new_string = kwasm.kwasm_new_string;
const kwasm_call_js_with_args_raw = kwasm.kwasm_call_js_with_args_raw;
const kwasm_call_js_with_args = kwasm.kwasm_call_js_with_args;
const kwasm_js_object_property = kwasm.kwasm_js_object_property;
const kwasm_get_js_object_value_u32 = kwasm.kwasm_get_js_object_value_u32;
const kwasm_get_js_object_value_f64 = kwasm.kwasm_get_js_object_value_f64;

const kwasm_new_worker = kwasm.kwasm_new_worker;
export {
    kwasm_free_js_object as kwasm_free_js_object,
    kwasm_new_string as kwasm_new_string,
    kwasm_call_js_with_args_raw as kwasm_call_js_with_args_raw,
    kwasm_call_js_with_args as kwasm_call_js_with_args,
    kwasm_js_object_property as kwasm_js_object_property,
    kwasm_get_js_object_value_u32 as kwasm_get_js_object_value_u32,
    kwasm_get_js_object_value_f64 as kwasm_get_js_object_value_f64,
    kwasm_new_worker as kwasm_new_worker
};
export function kwasm_initialize_wasmbindgen(module, memory) {
    self.kwasm_module = module;
    self.kwasm_memory = memory;
}
