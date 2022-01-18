use koi::*;

#[derive(Component, Clone)]
struct Counter(u32);

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // A camera is needed to display the UI
        world.spawn((Transform::new(), Camera::new(), CameraControls::new()));
        world.spawn((Transform::new(), Camera::new_for_user_interface()));

        let mut style = kui::StandardStyle::default();

        // Load a default font.
        style
            .new_font(include_bytes!("../Inter-Regular.otf"))
            .unwrap();

        let mut root_widget = kui::column((
            kui::heading(|_d: &_| "Hello".to_string()),
            kui::heading(|_d: &_| "Hi there!".to_string()),
        ));

        let mut ui_manager = UIManager::new(world, kui::StandardConstraints::default());

        move |event: Event, world| {
            match event {
                Event::Draw => {
                    ui_manager.draw(world);
                    ui_manager.update(world, &mut style, &mut root_widget);
                }
                _ => {}
            }
            false
        }
    });
}
