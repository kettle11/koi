function get_pointer_type(event) {
    switch (event.pointerType) {
        case "mouse": return 1
        case "pen": return 2
        case "touch": return 3
        default:
            return 0
    }
}


var canvas_last_width = 0;
var canvas_last_height = 0;

var animation_frame_requested = false;
function check_resize() {
    let width = canvas.clientWidth;
    let height = canvas.clientHeight;

    if (width != canvas_last_width || height != canvas_last_height) {
        var devicePixelRatio = window.devicePixelRatio || 1;
        canvas.width = width * devicePixelRatio;
        canvas.height = height * devicePixelRatio;
        canvas_last_width = width;
        canvas_last_height = height;
        self.kwasm_exports.kapp_on_window_resized(width * devicePixelRatio, height * devicePixelRatio);

    }
}

function request_animation_frame_callback(time) {
    let now = Date.now() - start_timestamp;
    for (const [key, time_stamp] of key_down_map) {
        if ((now - time_stamp) > 1000) {
            if (key != "OSLeft" &&
                key != "OSRight" &&
                key != "MetaLeft" &&
                key != "MetaRight" &&
                key != "ShiftLeft" &&
                key != "ShiftRight" &&
                key != "AltLeft" &&
                key != "AltRight" &&
                key != "ControlLeft" &&
                key != "ControlRight") {
                // Synthesize a keyup event when a keydown event hasn't occurred for one second.
                // Keydown events repeat for all the character keys.
                key_down_map.delete(key);
                self.kwasm_pass_string_to_client(key);
                self.kwasm_exports.kapp_on_key_up(now);
            }
        }
    }
    animation_frame_requested = false;
    check_resize();
    self.kwasm_exports.kapp_on_animation_frame(self.kwasm_exports.kapp_on_animation_frame);
}

function pass_f32_to_client(x) {
    let pointer = self.kwasm_exports.kwasm_reserve_space(4);
    let data_view = new Float32Array(kwasm_memory.buffer, pointer, 4);
    data_view[0] = x;
}

function pass_f32_f32_to_client(x, y) {
    let pointer = self.kwasm_exports.kwasm_reserve_space(8);
    let data_view = new Float32Array(kwasm_memory.buffer, pointer, 8);
    data_view[0] = x;
    data_view[1] = y;
}

var canvas = document
    .getElementById("canvas");

let previous_mouse_x;
let previous_mouse_y;

let start_timestamp = Date.now();
let key_down_map = new Map();

// When the window loses focus send a key up for all events.
window.addEventListener('blur', function () {
    let now = Date.now() - start_timestamp;
    for (const [key, time_stamp] of key_down_map) {
        key_down_map.delete(key);
        self.kwasm_pass_string_to_client(key);
        self.kwasm_exports.kapp_on_key_up(now);
    }
});

function check_for_synthetic_key_up(code) {
    if (key_down_map.has(code)) {
        let now = Date.now() - start_timestamp;
        key_down_map.delete(code);
        self.kwasm_pass_string_to_client(code);
        self.kwasm_exports.kapp_on_key_up(now);
    }
}

function check_special_key_status(event) {
    if (!event.shiftKey) {
        check_for_synthetic_key_up("ShiftRight");
        check_for_synthetic_key_up("ShiftLeft");
    }
    if (!event.metaKey) {
        check_for_synthetic_key_up("MetaLeft");
        check_for_synthetic_key_up("MetaRight");
        check_for_synthetic_key_up("OSRight");
        check_for_synthetic_key_up("OSLeft");
    }
    if (!event.ctrlKey) {
        check_for_synthetic_key_up("ControlLeft");
        check_for_synthetic_key_up("ControlRight");
    }
    if (!event.altKey) {
        check_for_synthetic_key_up("AltLeft");
        check_for_synthetic_key_up("AltRight");
    }
}

function receive_message(command, data) {

    switch (command) {
        case 0:
            // RequestAnimationFrame
            // Request an animation frame
            if (!animation_frame_requested) {
                animation_frame_requested = true;
                request_animation_frame_client_callback = data;
                window.requestAnimationFrame(request_animation_frame_callback)
            }
            break;
        case 1:
            // GetCanvasSize
            // Unused presently.
            break;
        case 2:
            // SetCallbacks

            // Hook up callbacks
            window.onresize = function (event) {
                check_resize();
            }
            canvas.onpointermove = function (event) {
                check_special_key_status(event);
                let pointer_type = get_pointer_type(event);
                self.kwasm_exports.kapp_on_pointer_move(event.clientX * window.devicePixelRatio, event.clientY * window.devicePixelRatio, pointer_type, event.timeStamp, event.pointerId);
            }
            canvas.onmousemove = function (event) {
                check_special_key_status(event);
                // Calculate delta instead to make it more consistent between browsers. 
                let movement_x = (previous_mouse_x ? event.screenX - previous_mouse_x : 0)
                let movement_y = (previous_mouse_y ? event.screenY - previous_mouse_y : 0)
                previous_mouse_x = event.screenX;
                previous_mouse_y = event.screenY;
                self.kwasm_exports.kapp_on_mouse_move(movement_x * window.devicePixelRatio, movement_y * window.devicePixelRatio, event.timeStamp);
            }
            canvas.onpointerdown = function (event) {
                check_special_key_status(event);
                canvas.setPointerCapture(event.pointerId);
                let pointer_type = get_pointer_type(event);
                self.kwasm_exports.kapp_on_pointer_down(event.clientX * window.devicePixelRatio, event.clientY * window.devicePixelRatio, pointer_type, event.button, event.timeStamp, event.pointerId);
            }
            canvas.onpointerup = function (event) {
                check_special_key_status(event);
                let pointer_type = get_pointer_type(event);
                self.kwasm_exports.kapp_on_pointer_up(event.clientX * window.devicePixelRatio, event.clientY * window.devicePixelRatio, pointer_type, event.button, event.timeStamp, event.pointerId);

            }
            canvas.onmouseup = function (event) {
                if (event.detail == 2) {
                    self.kwasm_exports.kapp_on_double_click(event.clientX * window.devicePixelRatio, event.clientY * window.devicePixelRatio, event.button, event.timeStamp);
                }
            }
            canvas.onpointercancel = function (event) {
                check_special_key_status(event);
                let pointer_type = get_pointer_type(event);
                self.kwasm_exports.kapp_on_pointer_up(event.clientX * window.devicePixelRatio, event.clientY * window.devicePixelRatio, pointer_type, event.button, event.timeStamp, event.pointerId);
            }

            // This is a hack to prevent the iPad's "scribble" feature from messing up PointerDown events in Safari.
            canvas.ontouchmove = function (event) {
                event.preventDefault();
            }

            // Prevent backswipe gesture on Safari
            canvas.ontouchstart = function (event) {
                event.preventDefault();
            }

            document.onkeydown = function (event) {
                check_special_key_status(event);

                key_down_map.set(event.code, event.timeStamp);

                self.kwasm_pass_string_to_client(event.code);
                if (event.repeat) {
                    self.kwasm_exports.kapp_on_key_repeat(event.timeStamp);
                } else {
                    self.kwasm_exports.kapp_on_key_down(event.timeStamp);
                }

                // Perhaps these character received events should only be sent if text input has been enabled.

                // Ignore keys pressed while composing an IME character.
                // Also ignore keys that are longer than 1 character.
                // This is incorrect for some non-English key combos, but is an OK heuristic for now
                // to reject non-textual character inputs.
                // A more robust solution may watch a text field for changes instead.
                if (!event.is_composing && event.key.length == 1) {
                    self.kwasm_pass_string_to_client(event.key);
                    self.kwasm_exports.kapp_character_received(event.timeStamp);
                }

                // This prevents everything else on the page from receiving an event. 
                // It's probably OK for now.
                // It fixes a Safari issue where Ctrl+Z and Shift+Ctrl+Z are used for browser navigation.
                event.preventDefault();
            }

            document.onkeyup = function (event) {
                if (key_down_map.has(event.code)) {
                    key_down_map.delete(event.code);
                    self.kwasm_pass_string_to_client(event.code);
                    self.kwasm_exports.kapp_on_key_up(event.timeStamp);
                    check_special_key_status(event);
                }
            }

            canvas.onwheel = function (event) {
                if (event.ctrlKey) {
                    // This is a bit weird, but if a pinch gesture is performed
                    // the ctrl modifier is set.
                    // This is the simplest way to disambiguate it.

                    // 0.02 is a completely arbitrary number to make this value more similar
                    // to what native MacOS produces.
                    // Is this a good idea at all?
                    // Should this library even make such adjustments?
                    // Is there a way to find an actual scale factor instead of a guess?
                    event.preventDefault();
                    self.kwasm_exports.kapp_on_pinch(-event.deltaY * 0.02, event.timeStamp);
                } else {
                    self.kwasm_exports.kapp_on_scroll(-event.deltaX, -event.deltaY, event.timeStamp);
                }

                // Prevent scrolling horizontally from going back on Safari
                event.preventDefault();
            }

            window.addEventListener("unload", function (event) {
                this.self.kwasm_exports.kapp_on_unload();
            });

            window.addEventListener("beforeunload", function (event) {
                this.self.kwasm_exports.kapp_on_before_unload();
            });

            break;
        case 3:
            // GetDevicePixelRatio
            // This will be sent to Rust as an integer.
            // So this will be incorrect if non-integer values are expected here.
            pass_f32_to_client(window.devicePixelRatio);
            break;
        case 4:
            // GetWindowSize

            let width = canvas.clientWidth * window.devicePixelRatio;
            let height = canvas.clientHeight * window.devicePixelRatio;

            // This will be sent to Rust as an integer.
            // So this will be incorrect if non-integer values are expected here.
            pass_f32_f32_to_client(width, height);
            break;
        case 5:
            // LockCursor
            canvas.requestPointerLock();
            break;
        case 6:
            // UnlockCursor
            document.exitPointerLock();
            break;
    }
    return 0;
}

receive_message