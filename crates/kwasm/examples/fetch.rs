use kwasm::libraries::*;
use kwasm::*;

fn main() {
    setup_panic_hook();
    ktasks::create_workers(kwasm::available_threads());
    ktasks::spawn(async {
        log("Made it here!0");
        let result = kwasm::libraries::fetch::fetch("README.md").await.unwrap();
        log("Made it here!1");
        let result_string = std::str::from_utf8(&result).unwrap();
        log(result_string);
    })
    .run();
    ktasks::run_current_thread_tasks();
}
