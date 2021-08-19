use ktasks::*;
use std::task::Poll;

fn main() {
    create_workers(3);

    let task0 = spawn(async {
        println!("HERE IN TASK0");
        let result = spawn(async {
            println!("HERE IN SUBTASK --------");
            4
        })
        .await;

        println!("RESULT OF SUBTASK: {:?}", result);
        std::thread::sleep(std::time::Duration::from_millis(1));

        100 + result
    });

    task0.run();

    std::thread::sleep(std::time::Duration::from_millis(50));
    run_current_thread_tasks();

    if let Poll::Ready(result) = task0.is_complete() {
        println!("RESULT: {:?}", result);
    }
}
