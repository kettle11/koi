use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.
        world.spawn((
            Transform::new().with_position(Vec3::Z),
            Camera::new().with_orthographic_projection(),
        ));
        world.spawn((
            Transform::new(),
            Mesh::TRIANGLE,
            Material::UNLIT,
            Color::MAJORELLE_BLUE,
        ));

        // Run the World with this mutable closure.
        |_event: Event, _world: &mut World| false
    });
}
