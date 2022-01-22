use koi::*;

#[derive(Component, Clone)]
struct Counter(u32);

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // A camera is needed to display the UI
        world.spawn((Transform::new(), Camera::new(), CameraControls::new()));

        world.spawn((Transform::new(), Camera::new_for_user_interface()));
        use kui::*;

        let mut fonts = Fonts::empty();
        fonts.load_default_fonts();

        let mut standard_context = kui::StandardContext::new(
            kui::StandardStyle::default(),
            kui::StandardInput::default(),
            fonts,
        );

        let mut data = 10;
        let mut root_widget = stack((
            fill(|_| Color::WHITE),
            padding(|_| 50., button(|data| *data += 1, text("Button"))),
            text(|data: &i32| data.to_string()),
        ));

        let mut ui_manager = UIManager::new(world);

        move |event: Event, world| {
            match event {
                Event::Draw => {
                    ui_manager.prepare(world, &mut standard_context);
                    ui_manager.update_layout_draw(
                        &mut data,
                        &mut standard_context,
                        &mut root_widget,
                    );
                    ui_manager.draw(world);
                }
                _ => {}
            }
            false
        }
    });
}
