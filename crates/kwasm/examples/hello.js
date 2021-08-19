function receive_message(command, data) {
    // Each of these commands could do something different.
    // The data is an ArrayBuffer that is a view into the raw WebAssembly bytes.
    // Data could be used to pass a string, or multiple parameters.
    if (command == 0) {
        console.log("COMMAND 0");
    }
    if (command == 1) {
        console.log("COMMAND 1");
    }
    if (command == 2) {
        kwasm_helpers.pass_string_to_client("HI CLIENT");
    }
    return 0;
}

return receive_message;