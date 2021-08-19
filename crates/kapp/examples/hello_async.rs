/// This example demonstrates using kapp's built in async support
use kapp::*;
fn main() {
    run_async(run);
}

async fn run(app: Application, events: Events) {
    let mut _window = app.new_window().build().unwrap();

    // Loop forever!
    loop {
        match events.next().await {
            Event::WindowCloseRequested { .. } => app.quit(),
            Event::Draw { .. } => {}
            _ => {}
        }
    }
}
