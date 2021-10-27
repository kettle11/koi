use koi::*;

#[derive(Component, Clone)]
struct Counter(u32);

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // A camera is needed to display the UI
        world.spawn((Transform::new(), Camera::new(), CameraControls::new()));
        world.spawn((Transform::new(), Camera::new_for_user_interface()));

        world.spawn((
            Transform::new().with_position(Vec3::Z * -2.0),
            Mesh::CUBE,
            Material::UNLIT,
            Color::WHITE,
        ));

        let mut style = StandardStyle::new();

        // Load a default font.
        style
            .new_font(include_bytes!("../Inter-Regular.otf"))
            .unwrap();

        let root = scroll_view(column(|world, mut child_creator| {
            (|q: Query<Option<&Transform>>| {
                for (e, transform) in q.entities_and_components() {
                    if let Some(_transform) = transform {
                        let mut e = *e;
                        child_creator.child(&mut e, || {
                            button(|data: &mut _| format!("{:?}", data), |_| {})
                        });
                    }
                }
            })
            .run(world);
        }));

        let mut ui = UI::new(world, root);

        move |event: Event, world: &mut World| {
            match event {
                Event::FixedUpdate => {}
                Event::KappEvent(event) => {
                    if ui.handle_event(world, event) {
                        return true;
                    }
                }
                Event::Draw => {
                    // Update and draw the UI.
                    ui.draw(world, &mut style);
                }
            }
            false
        }
    });
}
