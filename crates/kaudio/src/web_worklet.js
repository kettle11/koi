
var audio_running = false;

function run_on_worklet() {
    class KAudioProcessor extends AudioWorkletProcessor {
        constructor(options) {
            super(options);

            let setup_data = options.processorOptions;

            let imports = {};
            let memory_assigned = false;

            // Fill audio worklet imports with placeholder values.
            // None of these functions will be called on this worklet thread anyways.
            WebAssembly.Module.imports(setup_data.kwasm_module).forEach(item => {
                if (imports[item.module] === undefined) {
                    imports[item.module] = {};
                }

                if (item.kind == "function") {
                    imports[item.module][item.name] = function () {
                        console.log(item.name + "is unimplemented in audio worklet");
                    }
                }
                if (item.kind == "memory") {
                    imports[item.module][item.name] = setup_data.kwasm_memory;
                    memory_assigned = true;
                }
            });

            if (!memory_assigned) {
                imports.env = {
                    memory: setup_data.kwasm_memory
                };
            }
            this.kwasm_memory = setup_data.kwasm_memory;

            WebAssembly.instantiate(setup_data.kwasm_module, imports).then(results => {
                this.port.postMessage("INSTANTIATED MODULE");

                let exports = results.exports;
                if (exports.__wbindgen_start) {
                    exports.__wbindgen_start();
                } else {
                    exports.set_stack_pointer(setup_data.stack_pointer);
                    exports.__wasm_init_tls(setup_data.thread_local_storage_pointer);
                }

                exports.kwasm_web_worker_entry_point(setup_data.entry_point);
                this.kwasm_exports = exports;

            });
            this.port.postMessage("Successfully initialized Audio Worklet");
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
    let callback = (event) => {
        if (!audio_running) {
            setup_worklet();
            audio_running = true;
        }

        async function setup_worklet() {
            console.log("SETTING UP AUDIO WORKLET!");
            const audioContext = new AudioContext({ sampleRate: 44100 });

            let blobURL = URL.createObjectURL(new Blob(
                ['(', run_on_worklet.toString(), ')()'],
                { type: 'application/javascript' }
            ));

            await audioContext.audioWorklet.addModule(blobURL);
            URL.revokeObjectURL(blobURL);

            const worklet = new AudioWorkletNode(audioContext, 'kaudio-processor', {
                outputChannelCount: [2],
                processorOptions: {
                    kwasm_memory: self.kwasm_memory,
                    kwasm_module: self.kwasm_module,
                    entry_point: entry_point,
                    stack_pointer: stack_pointer,
                    thread_local_storage_pointer: thread_local_storage_pointer,
                }
            });
            worklet.connect(audioContext.destination);
            worklet.port.onmessage = (e) => {
                console.log("Worklet message: ", e.data);
            };

            this.kaudio_audio_worklet = worklet;
            audioContext.resume();
        }

    };

    document.onclick = callback;
}
setup_worklet