use koi::*;

#[derive(Component, Clone)]
struct Controlled;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // A camera is needed to display the UI
        world.spawn((Transform::new(), Camera::new_for_user_interface()));

        let mut style = StandardStyle::new();

        // Load a default font.
        style
            .new_font(include_bytes!("../Inter-Regular.otf"))
            .unwrap();

        let root = button("Hello", |data| {
            *data += 1;
        });
        let mut ui = UI::new(world, root);

        let mut counter = 0;
        move |event: Event, world: &mut World| match event {
            Event::FixedUpdate => {}
            Event::Draw => {
                // Update and draw the UI.
                ui.draw(world, &mut style, &mut counter);
            }
            _ => {}
        }
    });
}
