use ktasks::*;
use std::task::Poll;

fn main() {
    create_workers();

    let thread_id = std::thread::current().id();
    println!("Main thread ID: {:?}", thread_id);
    let task0 = spawn(async move {
        println!(
            "Running on thread with ID: {:?}",
            std::thread::current().id()
        );
    });
    task0.run();

    let task1 = spawn(async move {
        println!(
            "Likely on another thread. Thread ID: {:?}",
            std::thread::current().id()
        );
    });
    task1.run();

    std::thread::sleep(std::time::Duration::from_millis(50));
    run_current_thread_tasks();

    if let Some(result) = task0.get_result() {
        println!("RESULT: {:?}", result);
    }
}
