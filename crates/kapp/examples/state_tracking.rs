/// Just display a window
use kapp::*;

fn main() {
    let (app, event_loop) = initialize();
    let _window = app.new_window().build().unwrap();

    event_loop.run(move |event| {
        if app.pointer_button(PointerButton::Primary) {
            println!("Mouse pressed");
        }

        match event {
            Event::WindowCloseRequested { .. } => app.quit(),
            _ => {}
        }
    });
}
