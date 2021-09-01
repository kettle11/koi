use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.

        // Spawn a camera and make it look towards the origin.
        let mut transform = Transform::new_with_position(Vec3::new(-3.0, 0.0, 0.0));
        transform.look_at(Vec3::ZERO, Vec3::Y);
        world.spawn((transform, Camera::new(), CameraControls::new()));

        // Spawn a cube
        world.spawn((Transform::new(), Mesh::CUBE, Material::PHYSICALLY_BASED));

        // Spawn a light
        world.spawn((
            Light::new(LightMode::Directional, Color::WHITE, 1.0),
            Transform::new_with_position([0., 8.0, 8.0].into()),
            Mesh::SPHERE,
            Material::UNLIT,
        ));

        // Run the World with this mutable closure.
        move |event: Event, _: &mut World| {
            match event {
                Event::FixedUpdate => {
                    // Perform physics and game related updates here.
                }
                Event::Draw => {
                    // Things that occur before rendering can go here.
                }
                _ => {}
            }
        }
    });
}
