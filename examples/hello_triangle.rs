use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.
        world.spawn((
            Transform::new().with_position(Vec3::Z),
            Camera::new().with_orthographic_projection(),
            CameraControls::new(),
        ));
        world.spawn((Transform::new(), Mesh::TRIANGLE, Material::UNLIT));

        // Run the World with this mutable closure.
        |_event: Event, _world: &mut World| false
    });
}
