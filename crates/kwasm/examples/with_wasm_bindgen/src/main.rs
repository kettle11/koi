fn main() {
    use web_sys::console;
    console::log_1(&"Hello using web-sys".into());
    kwasm::libraries::log("Hello using kwasm");
}
