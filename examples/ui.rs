use koi::*;

#[derive(Component, Clone)]
struct Counter(u32);

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // A camera is needed to display the UI
        world.spawn((Transform::new(), Camera::new_for_user_interface()));
        world.spawn((Transform::new(), Camera::new(), CameraControls::new()));

        world.spawn((
            Transform::new_with_position(Vec3::Z * -2.0),
            Mesh::CUBE,
            Material::UNLIT,
            Color::WHITE,
        ));

        let mut style = StandardStyle::new();

        // Load a default font.
        style
            .new_font(include_bytes!("../Inter-Regular.otf"))
            .unwrap();

        let root = column(
            |world: &mut World, mut child_creator: ChildrenCreator<_, _>| {
                (|q: Query<(Option<&HierarchyNode>)>| {
                    for (e, t) in q.entities_and_components() {
                        child_creator.child(&mut (), || button("Press me", |data| {}));
                    }
                })
                .run(world);
            },
        );

        let mut ui = UI::new(world, root);

        move |event: Event, world: &mut World| match event {
            Event::FixedUpdate => {}
            Event::Draw => {
                // Update and draw the UI.
                ui.draw(world, &mut style);
            }
            _ => {}
        }
    });
}
