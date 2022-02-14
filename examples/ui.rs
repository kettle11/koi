use koi::*;

#[derive(Component, Clone)]
struct Counter(u32);

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // A camera is needed to display the UI
        world.spawn((Transform::new(), Camera::new(), CameraControls::new()));
        world.spawn((Transform::new(), Camera::new_for_user_interface()));

        // Spawn the [Counter] we'll edit with our UI
        world.spawn(Counter(0));

        let mut fonts = Fonts::empty();
        fonts.load_default_fonts();

        let mut standard_context = kui::StandardContext::new(
            kui::StandardStyle::default(),
            kui::StandardInput::default(),
            fonts,
        );

        let mut root_widget = stack((
            fill(|_, _| Color::WHITE),
            padding(
                |_| 50.,
                button(
                    |world: &mut World| world.get_singleton::<Counter>().0 += 1,
                    "Button",
                ),
            ),
            text(|world: &mut World| world.get_singleton::<Counter>().0.to_string()),
        ));

        let mut ui_manager = UIManager::new(world);

        move |event: Event, world| {
            match event {
                Event::FixedUpdate => {
                    ui_manager.update(world, &mut standard_context, &mut root_widget);
                }
                Event::Draw => {
                    ui_manager.layout_and_draw(world, &mut standard_context, &mut root_widget);
                }
                _ => {}
            }
            false
        }
    });
}
