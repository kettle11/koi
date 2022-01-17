use koi::*;

#[derive(Component, Clone)]
struct Counter(u32);

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // A camera is needed to display the UI
        world.spawn((Transform::new(), Camera::new(), CameraControls::new()));
        world.spawn((Transform::new(), Camera::new_for_user_interface()));

        let ui = kui::row(kui::for_each(
            |data: &mut World, f| {
                (|mut transforms: Query<&mut Transform>| {
                    for transform in &mut transforms {
                        f(transform)
                    }
                })
                .run(data);
            },
            || kui::rectangle(Vec2::fill(100.), Color::RED),
        ));
        let mut ui = UI::new(world, kui::StandardConstraints::default(), ui);

        move |event: Event, world: &mut World| {
            match event {
                Event::Draw => {
                    ui.draw(world);
                    ui.update(world);
                }
                _ => {}
            }
            false
        }
    });
}
