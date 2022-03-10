use ktasks::*;
use std::task::Poll;

fn main() {
    create_workers();

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

    if let Some(result) = task0.get_result() {
        println!("RESULT: {:?}", result);
    }
}
