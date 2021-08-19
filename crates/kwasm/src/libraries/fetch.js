let result = function (string_index, callback_pointer) {
    let path = self.kwasm_get_object(string_index);

    // This could fail, but for now don't handle those cases.
    fetch(path)
        .then(async response => {
            let result = await response.arrayBuffer();
            let pointer = self.kwasm_exports.kwasm_reserve_space(result.byteLength);
            let destination = new Uint8Array(kwasm_memory.buffer, pointer, result.byteLength);
            destination.set(new Uint8Array(result));
            self.kwasm_exports.kwasm_complete_fetch(callback_pointer);
        });
};

result