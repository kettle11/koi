use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Spawn a camera
        world.spawn((Transform::new(), Camera::new(), CameraControls::new()));

        // Spawn a light
        world.spawn((
            Light::new(LightMode::Directional, Color::WHITE, 1.0),
            Transform::new_with_position([0., 8.0, 8.0].into()),
            Material::UNLIT,
        ));

        // Spawn a loaded gltf
        let worlds = world.get_single_component_mut::<Assets<World>>().unwrap();
        let gltf_world = worlds.load(&"../koi/assets/one_angery_dragon_boi/scene.gltf");
        world.spawn(gltf_world);

        |_, _| {}
    });
}
