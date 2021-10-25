use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.
        world.spawn((
            Transform::new(),
            Camera::new_orthographic(),
            CameraControls::new(),
        ));
        world.spawn((Transform::new(), Mesh::TRIANGLE, Material::UNLIT));

        // Run the World with this mutable closure.
        |event: Event, _: &mut World| {
            match event {
                _ => {}
            }
            false
        }
    });
}
