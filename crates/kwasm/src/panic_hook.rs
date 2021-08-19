use std::panic;

fn hook_impl(info: &panic::PanicInfo) {
    let message = info.to_string();
    crate::libraries::console::error(&message);
}

/// Sets up a panic hook to print a slightly more useful error-message to the console.
pub fn setup_panic_hook() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(hook_impl));
    });
}
