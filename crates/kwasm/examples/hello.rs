use kwasm::libraries::*;
use kwasm::*;

fn main() {
    setup_panic_hook();
    console::log("HELLO WORLD!");

    let console = JS_SELF.get_property("console");
    let log_function = console.get_property("log");

    let message = JSString::new("HI WORLD!!!");
    log_function.call_1_arg(&console, &message);

    console::log("LOGGING FROM THE CONSOLE");

    eval("console.log('EVAL SEEMS TO WORK')");
    let v = kwasm::libraries::Instant::now();
}
