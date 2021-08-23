
var audio_running = false;

function run_on_worklet() {
    class KAudioProcessor extends AudioWorkletProcessor {
        constructor(...args) {
            super(...args)
            this.port.onmessage = (e) => {
                let imports = {};
                let memory_assigned = false;

                // Fill audio worklet imports with placeholder values.
                // None of these functions will be called on this worklet thread anyways.
                WebAssembly.Module.imports(e.data.kwasm_module).forEach(item => {
                    if (imports[item.module] === undefined) {
                        imports[item.module] = {};
                    }

                    if (item.kind == "function") {
                        imports[item.module][item.name] = function () {
                            console.log(item.name + "is unimplemented in audio worklet");
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

                    exports.kwasm_web_worker_entry_point(e.data.entry_point);
                    this.kwasm_exports = exports;

                });
            }
        }

        process(inputs, outputs, parameters) {
            if (this.kwasm_exports) {
                let channel_count = outputs[0].length;
                let frame_size = outputs[0][0].length; // It's probably fine to assume all channels have the same frame size. 
                this.kwasm_exports.kaudio_run_callback(channel_count, frame_size, sampleRate);

                for (let i = 0; i < channel_count; i++) {
                    let location = this.kwasm_exports.kaudio_audio_buffer_location(i);
                    let data = new Float32Array(this.kwasm_memory.buffer, location, frame_size);
                    outputs[0][i].set(data);
                }
            }
            return true;
        }
    }

    registerProcessor('kaudio-processor', KAudioProcessor);
}


function setup_worklet(entry_point, stack_pointer, thread_local_storage_pointer) {

    document.onpointerdown = (event) => {
        if (!audio_running) {
            setup_worklet();
            audio_running = true;
        }

        async function setup_worklet() {
            const audioContext = new AudioContext({ sampleRate: 44100 });

            var blobURL = URL.createObjectURL(new Blob(
                ['(', run_on_worklet.toString(), ')()'],
                { type: 'application/javascript' }
            ));

            await audioContext.audioWorklet.addModule(blobURL);
            URL.revokeObjectURL(blobURL);

            const worklet = new AudioWorkletNode(audioContext, 'kaudio-processor', {
                outputChannelCount: [2],
            });
            worklet.connect(audioContext.destination);
            let message = {};

            // Smuggling these values via document properties
            // is hack for now, but it requires a specific index.html setup
            // and should be replaced.
            message.kwasm_memory = self.kwasm_memory;
            message.kwasm_module = self.kwasm_module;
            message.entry_point = entry_point;
            message.stack_pointer = stack_pointer;
            message.thread_local_storage_pointer = thread_local_storage_pointer;

            worklet.port.postMessage(message);
        }

    };

}
setup_worklet