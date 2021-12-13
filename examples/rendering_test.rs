use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Spawn a camera
        world.spawn((
            Transform::new()
                .with_position(Vec3::new(0.0, 4.0, 3.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            Camera::new(),
            CameraControls::new(),
        ));

        spawn_skybox(world, "assets/venice_sunset_1k.hdr");

        /*
        let worlds = world.get_single_component_mut::<Assets<World>>().unwrap();
        let gltf_world = worlds.load("assets/SciFiHelmet/glTF/SciFiHelmet.gltf");

        // Spawn a Handle<World> that will be replaced with the GlTf when it's loaded.
        let gltf_hierarchy = world.spawn(gltf_world);
        let scaled_down = world.spawn(Transform::new().with_scale(Vec3::fill(10.0)));
        set_parent(world, Some(scaled_down), gltf_hierarchy);
        */

        // Spawn a series of balls with different material properties.
        // Up is more metallaic
        // Right is more more rough
        let spacing = 2.5;
        let mut commands = Commands::new();
        (|materials: &mut Assets<Material>| {
            let rows = 6;
            let columns = 6;
            for i in 0..rows {
                for j in 0..columns {
                    let new_material = materials.add(new_pbr_material(
                        Shader::PHYSICALLY_BASED,
                        PBRProperties {
                            base_color: Color::new(0.5, 0.0, 0.0, 1.0),
                            metallic: i as f32 / rows as f32,
                            roughness: (j as f32 / columns as f32).clamp(0.05, 1.0),
                            ..Default::default()
                        },
                    ));
                    commands.spawn((
                        Transform::new().with_position(Vec3::new(
                            j as f32 * spacing,
                            i as f32 * spacing,
                            -2.0,
                        )),
                        new_material,
                        Mesh::SPHERE,
                    ))
                }
            }
        })
        .run(world);
        commands.apply(world);

        |_, _| false
    });
}
