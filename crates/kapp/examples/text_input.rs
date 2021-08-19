extern crate kapp;
use kapp::*;

fn main() {
    let (app, event_loop) = initialize();
    let _window = app.new_window().title("Log Events").build().unwrap();

    app.start_text_input();
    event_loop.run(move |event| match event {
        // Just log text input related events.
        Event::IMEComposition { .. }
        | Event::IMEEndComposition
        | Event::CharacterReceived { .. } => println!("{:?}", event),
        Event::WindowCloseRequested { .. } => app.quit(),
        _ => {}
    });
}
