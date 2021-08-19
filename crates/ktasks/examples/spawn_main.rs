use ktasks::*;
use std::task::Poll;

fn main() {
    create_workers(3);
    let main_thread_id = std::thread::current().id();

    let task0 = spawn(async move {
        println!("HERE IN TASK0");
        let result = spawn_main(async move {
            println!("SUBTASK ON MAIN THREAD");
            assert!(main_thread_id == std::thread::current().id());
            println!("DONE");
            10
        })
        .await;

        std::thread::sleep(std::time::Duration::from_millis(1));
        result * 2
    });

    task0.run();

    std::thread::sleep(std::time::Duration::from_millis(50));
    run_current_thread_tasks();
    std::thread::sleep(std::time::Duration::from_millis(50));

    if let Poll::Ready(result) = task0.is_complete() {
        println!("RESULT: {:?}", result);
    }
}
