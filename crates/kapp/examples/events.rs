extern crate kapp;
use kapp::*;

fn main() {
    let (app, event_loop) = initialize();
    let _window = app.new_window().title("Log Events").build().unwrap();

    event_loop.run(move |event| match event {
        // EventsCleared and MouseMoved log too much, so ignore them.
        Event::EventsCleared | Event::PointerMoved { .. } => {}
        Event::WindowCloseRequested { .. } => app.quit(),
        _ => {
            println!("{:?}", event);
        }
    });
}
