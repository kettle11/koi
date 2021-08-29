use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        let mut camera = Camera::new();
        // camera.clear_color = Some(Color::new(0.5, 0.0, 0.0, 1.0));
        camera.clear_color = Some(Color::WHITE);

        // Spawn a camera
        world.spawn((
            Transform::new_with_position(Vec3::new(0., 0., 4.0)),
            camera,
            CameraControls::new(),
        ));

        let mut light_transform = Transform::new_with_position([0., 8.0, 8.0].into());
        light_transform.look_at(Vec3::ZERO, Vec3::Y);

        // Spawn a light
        world.spawn((
            light_transform,
            Light::new(LightMode::Directional, Color::WHITE, 10.0),
            Material::UNLIT,
        ));

        world.spawn((
            Transform::new_with_position(Vec3::new(0.0, 0.0, -3.0)),
            Mesh::CUBE,
            Material::PHYSICALLY_BASED,
        ));

        // Spawn a loaded gltf
        let worlds = world.get_single_component_mut::<Assets<World>>().unwrap();
        let gltf_world = worlds.load(&"assets/porsche/scene.gltf");
        let gltf = world.spawn((Transform::new(), gltf_world));

        // Set a parent to scale the thing down.
        // Why doesn't this work?
        let parent = world.spawn(Transform::new_with_scale(Vec3::fill(3.0)));
        HierarchyNode::set_parent(world, Some(parent), gltf);

        |_, _| {}
    });
}
