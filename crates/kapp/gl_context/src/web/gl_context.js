function create_webgl1_context() {
    let canvas = document
        .getElementById("canvas");

    // There are other potentially useful flags as well.
    canvas.getContext('webgl', {
        alpha: false,
        desynchronized: false,
        antialias: true,
        depth: true
    });
}

function create_webgl2_context() {
    console.log("CREATING WEBGL2 CONTEXT");

    let canvas = document
        .getElementById("canvas");

    // There are other potentially useful flags as well.
    canvas.getContext('webgl2', {
        alpha: false,
        desynchronized: false,
        antialias: true,
        depth: true
    });
}

function command(command, memory_buffer, data, data_length) {
    switch (command) {
        case 0:
            create_webgl1_context();
            break;
        case 1:
            create_webgl2_context();
            break;
    }
    return 0;
}
command