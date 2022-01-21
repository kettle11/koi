use koi::*;

#[derive(Component, Clone)]
struct Counter(u32);

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // A camera is needed to display the UI
        world.spawn((Transform::new(), Camera::new(), CameraControls::new()));

        world.spawn((Transform::new(), Camera::new_for_user_interface()));
        use kui::*;

        let mut standard_context =
            kui::StandardContext::new(kui::StandardStyle::default(), kui::StandardInput::default());

        let mut root_widget = stack((
            fill(|_| Color::WHITE),
            padding(|_| 50., button(|_| println!("CLICKED!"), text("BUTTON"))),
        ));

        let mut ui_manager = UIManager::new(world);

        move |event: Event, world| {
            match event {
                Event::Draw => {
                    ui_manager.update_input(
                        world,
                        std::rc::Rc::get_mut(&mut standard_context.input).unwrap(),
                    );
                    ui_manager.update_size(world, &mut standard_context);
                    ui_manager.update(world, &mut standard_context, &mut root_widget);
                    ui_manager.draw(world);
                }
                _ => {}
            }
            false
        }
    });
}
