let result = function (string_index, callback_pointer) {
    console.log("JS: FETCHING BYTES");
    let path = self.kwasm_get_object(string_index);

    // This could fail, but for now don't handle those cases.
    fetch(path)
        .then(response => {
            console.log("JS: HERE IN FETCH");
            response.arrayBuffer()
        }).then(result => {
            let pointer = self.kwasm_exports.kwasm_reserve_space(result.byteLength);
            let destination = new Uint8Array(kwasm_memory.buffer, pointer, result.byteLength);
            destination.set(new Uint8Array(result));
            console.log("ABOUT TO CALL BACK INTO RUST AFTER FETCH");
            self.kwasm_exports.kwasm_complete_fetch(callback_pointer);
        }).catch((error) => {
            console.error('Error:', error);
        });
};

function run_on_worker() {
    onmessage = e => {
        if (e.data.message_type === "setup") {
            return;
            let imports = {};
            let memory_assigned = false;

            // Fill worker imports with placeholder values.
            // None of these functions will be called on this worker anyways.
            WebAssembly.Module.imports(e.data.kwasm_module).forEach(item => {
                if (imports[item.module] === undefined) {
                    imports[item.module] = {};
                }

                if (item.kind == "function") {
                    imports[item.module][item.name] = function () {
                        console.log(item.name + "is unimplemented on fetch worker");
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
            this.kwasm_memory = e.data.kwasm_memory;

            WebAssembly.instantiate(e.data.kwasm_module, imports).then(results => {
                let exports = results.exports;
                if (exports.__wbindgen_start) {
                    exports.__wbindgen_start();
                } else {
                    exports.set_stack_pointer(e.data.stack_pointer);
                    exports.__wasm_init_tls(e.data.thread_local_storage_pointer);
                }

                // exports.kwasm_web_worker_entry_point(e.data.entry_point);
                this.kwasm_exports = exports;
            });
        } else {
            // This is a fetch
            console.log("RUNNING PROMISE CODE ON SUB WORKER");
            e.data.callback();
        }
    }
}

function fetch_inner(path, callback_pointer) {
    // This could fail, but for now don't handle those cases.
    fetch(path)
        .then(response => {
            console.log("JS: HERE IN FETCH");
            response.arrayBuffer()
        }).then(result => {
            let pointer = self.kwasm_exports.kwasm_reserve_space(result.byteLength);
            let destination = new Uint8Array(kwasm_memory.buffer, pointer, result.byteLength);
            destination.set(new Uint8Array(result));
            console.log("ABOUT TO CALL BACK INTO RUST AFTER FETCH");
            self.kwasm_exports.kwasm_complete_fetch(callback_pointer);
        })
}

/*
var fetch_object = {
    setup_worker(stack_pointer, thread_local_storage_pointer) {
        console.log("SETTING UP WORKER");
        let blobURL = URL.createObjectURL(new Blob(
            ['(', run_on_worker.toString(), ')()'],
            { type: 'application/javascript' }
        ));
        let fetch_worker = new Worker(blobURL);

        URL.revokeObjectURL(blobURL);
        let message = {
            kwasm_memory: self.kwasm_memory,
            kwasm_module: self.kwasm_module,
            // entry_point: entry_point,
            stack_pointer: stack_pointer,
            thread_local_storage_pointer: thread_local_storage_pointer,
            message_type: "setup"
        };
        fetch_worker.postMessage(message);
        return fetch_worker;
    },
    fetch_on_worker(string_index, callback_pointer) {
        console.log("TRYING TO FETCH ON WORKER----------");
        let path = self.kwasm_get_object(string_index);
        // kwasm_promise_worker is assigned when the worker thread is initialized.
        self.kwasm_promise_worker.postMessage({
            callback: function () {
                fetch_inner(path, callback_pointer)
            }
        });
    },
    fetch_local(string_index, callback_pointer) {
        console.log("JS: FETCHING LOCALLY");
        let path = self.kwasm_get_object(string_index);
        fetch_inner(path, callback_pointer);
    }
};
*/
fetch_inner