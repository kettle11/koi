use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.

        // Spawn a camera and make it look towards the origin.
        world.spawn((
            Transform::new()
                .with_position(Vec3::new(0.0, 4.0, 3.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            Camera::new(),
            CameraControls::new(),
        ));

        // Spawn a light
        // world.spawn((
        //     Transform::new()
        //         .with_position([0., 8.0, 8.0].into())
        //         .looking_at(Vec3::ZERO, Vec3::Y),
        //     Light::new(LightMode::Directional, Color::WHITE, 10.0),
        // ));

        // Spawn an HDRI and shadow-caster with primary light direction
        // inferred from the brightest point in the HDRI.
        spawn_skybox(world, "assets/field_1k.hdr");

        let path = "assets/VC/glTF/VC.gltf";

        // Begin loading a glTF
        let worlds = world.get_single_component_mut::<Assets<World>>().unwrap();
        let gltf_world = worlds.load(path);

        // Spawn a Handle<World> that will be replaced with the glTF when it's loaded.
        world.spawn(gltf_world);

        |_, _| false
    });
}
