use koi::*;

#[derive(Component, Clone)]
struct Counter(u32);

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // A camera is needed to display the UI
        world.spawn((Transform::new(), Camera::new(), CameraControls::new()));

        world.spawn((Transform::new(), Camera::new_for_user_interface()));

        let mut standard_context = kui::StandardContext {
            style: kui::StandardStyle::default(),
            input: kui::StandardInput::default(),
        };
        use kui::*;

        let mut root_widget = button(|_| println!("CLICKED"), text("Hello"));
        // let mut root_widget = stack((
        //     rectangle(Vec2::new(200., 200.), Color::RED),
        //     rectangle(Vec2::new(100., 100.), Color::BLUE),
        // ));
        let mut ui_manager = UIManager::new(world, kui::StandardConstraints::default());

        move |event: Event, world| {
            match event {
                Event::Draw => {
                    ui_manager.update_input(world, &mut standard_context.input);
                    ui_manager.update_size(world);
                    ui_manager.update(world, &mut standard_context, &mut root_widget);
                    ui_manager.draw(world);
                }
                _ => {}
            }
            false
        }
    });
}
