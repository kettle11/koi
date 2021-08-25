use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        let mut camera = Camera::new();
        // camera.clear_color = Some(Color::new(0.5, 0.0, 0.0, 1.0));
        // Spawn a camera
        world.spawn((Transform::new(), camera, CameraControls::new()));

        let mut light_transform = Transform::new_with_position([0., 8.0, 8.0].into());
        light_transform.look_at(Vec3::ZERO, Vec3::Y);

        // Spawn a light
        world.spawn((
            light_transform,
            Light::new(LightMode::Directional, Color::WHITE, 10.0),
            Material::UNLIT,
        ));

        // Spawn a loaded gltf
        let worlds = world.get_single_component_mut::<Assets<World>>().unwrap();
        let gltf_world = worlds.load(&"../koi/assets/one_angery_dragon_boi/scene.gltf");
        let gltf = world.spawn(gltf_world);

        // Set a parent to scale the thing down.
        // Why doesn't this work?
        // let parent = world.spawn(Transform::new_with_scale(Vec3::fill(0.001)));
        // HierarchyNode::set_parent(world, Some(parent), gltf);

        |_, _| {}
    });
}
