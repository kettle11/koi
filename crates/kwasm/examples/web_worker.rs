// For SharedArrayBuffers (and this example) to the correct cross-origin flags must be set.
// This is how to set those flags for a server hosted locally (like `devserver`)
// devserver --header Cross-Origin-Opener-Policy='same-origin' --header Cross-Origin-Embedder-Policy='require-corp'
use kwasm::libraries::*;
use kwasm::*;

fn main() {
    kwasm::web_worker::spawn(|| {
        console::log("In worker");
    });
}
