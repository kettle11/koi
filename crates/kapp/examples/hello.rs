/// Just display a window
use kapp::*;

fn main() {
    let (app, event_loop) = initialize();
    let _window = app.new_window().minimum_size(1000, 1000).build().unwrap();

    event_loop.run(move |event| match event {
        Event::WindowCloseRequested { .. } => app.quit(),
        Event::Draw { .. } => {
            // Render something here.
        }
        _ => {}
    });
}
