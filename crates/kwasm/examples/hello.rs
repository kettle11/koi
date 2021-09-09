use kwasm::libraries::*;
use kwasm::*;

fn main() {
    setup_panic_hook();
    console::log("HELLO WORLD!");

    let log_function = JS_SELF.get_property("console.log");

    let message = JSString::new("HI WORLD!!!");
    log_function.call_1_arg(&message);

    console::log("LOGGING FROM THE CONSOLE");

    eval("console.log('EVAL SEEMS TO WORK')");
    let _ = kwasm::libraries::Instant::now();
}
