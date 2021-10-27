use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.
        let mut camera = Camera::new();
        camera.clear_color = Some(Color::RED);
        let mut transform = Transform::new().with_position(Vec3::new(-3.0, 0.0, 0.0));
        transform.look_at(Vec3::ZERO, Vec3::Y);
        world.spawn((transform, camera, CameraControls::new()));
        world.spawn((Transform::new(), Mesh::CUBE, Material::PHYSICALLY_BASED));

        // Spawn a light
        world.spawn((
            Light::new(LightMode::Directional, Color::WHITE, 1.0),
            Transform::new().with_position([0., 8.0, 8.0].into()),
            Mesh::SPHERE,
            Material::UNLIT,
        ));

        let size = 8;
        for i in 0..size {
            for j in 0..size {
                for k in 0..size {
                    world.spawn((
                        Transform::new()
                            .with_position(Vec3::new(i as f32, j as f32, k as f32) * 3.0),
                        Mesh::CUBE,
                        Material::PHYSICALLY_BASED,
                    ));
                }
            }
        }

        // Run the World with this mutable closure.
        |event: Event, _: &mut World| false
    });
}
