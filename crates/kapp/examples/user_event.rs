/// Just display a window
use kapp::*;

fn main() {
    let (app, event_loop) = initialize();
    let _window = app.new_window().minimum_size(1000, 1000).build().unwrap();

    let user_event_sender = app.get_user_event_sender();
    event_loop.run(move |event| match event {
        Event::WindowCloseRequested { .. } => app.quit(),
        Event::Draw { .. } => {
            // Render something here.
        }
        Event::UserEvent { .. } => {
            println!("Received custom event: {:?}", event);
        }
        Event::KeyDown { key: Key::A, .. } => {
            user_event_sender.send(2, 120);
        }
        _ => {}
    });
}
