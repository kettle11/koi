use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.
        let mut camera = Camera::new();
        camera.clear_color = Some(Color::RED);
        world.spawn((
            Transform::new()
                .with_position(Vec3::new(-3.0, 0.0, 0.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            camera,
            CameraControls::new(),
        ));
        world.spawn((Transform::new(), Mesh::CUBE, Material::PHYSICALLY_BASED));

        // Spawn a light
        world.spawn((
            Light::new(LightMode::Directional, Color::WHITE, 1.0),
            ShadowCaster::new(),
            Transform::new()
                .with_position([0., 8.0, 8.0].into())
                .looking_at(Vec3::ZERO, Vec3::Y),
            Mesh::SPHERE,
            Material::UNLIT,
        ));

        world.spawn((
            Transform::new()
                .with_position(Vec3::new(0., 0.0, 0.))
                .with_scale(Vec3::fill(30.)),
            Mesh::CUBE,
            Material::PHYSICALLY_BASED,
        ));

        world.spawn((
            Transform::new()
                .with_position(Vec3::new(0., -50.0, 0.))
                .with_scale(Vec3::fill(100.)),
            Mesh::CUBE,
            Material::PHYSICALLY_BASED,
        ));

        // Run the World with this mutable closure.
        |_event: Event, _: &mut World| false
    });
}
