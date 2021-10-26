use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.

        // Spawn a camera and make it look towards the origin.
        world.spawn((
            Transform::new()
                .with_position(Vec3::new(0.0, 4.0, 3.0))
                .look_at(Vec3::ZERO, Vec3::Y),
            Camera::new(),
            CameraControls::new(),
        ));

        // Spawn a light
        world.spawn((
            Transform::new()
                .with_position([0., 8.0, 8.0].into())
                .look_at(Vec3::ZERO, Vec3::Y),
            Light::new(LightMode::Directional, Color::WHITE, 5.0),
        ));

        let path = "assets/shiba/scene.gltf";

        // Begin loading a GlTf
        let worlds = world.get_single_component_mut::<Assets<World>>().unwrap();
        let gltf_world = worlds.load(&path);

        // Spawn a Handle<World> that will be replaced with the GlTf when it's loaded.
        world.spawn(gltf_world);

        |_, _| false
    });
}
