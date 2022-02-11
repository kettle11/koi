use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Spawn a camera

        world.spawn((
            Transform {
                position: Vec3::new(-21.868414, 1.0, 23.094368),
                rotation: Quat::from_xyzw(-0.4105159, 0.38509429, -0.1969354, -0.8027361),
                scale: Vec3::ONE,
            },
            Camera::new(),
            CameraControls::new(),
        ));

        world.spawn((
            Transform {
                position: Vec3::new(27.708267, 57.67708, 35.69649),
                rotation: Quat::from_xyzw(0.28870586, -0.3600697, -0.11823557, -0.87920713),
                scale: Vec3::ONE,
            },
            Light::new(LightMode::Directional, Color::WHITE, 0.0),
            //  ShadowCaster::new().with_ibl_shadowing(0.8),
        ));

        world.spawn((
            Transform::new()
                .with_position(Vec3::new(0., -50.0, 0.))
                .with_scale(Vec3::fill(100.)),
            Mesh::SPHERE,
            Material::PHYSICALLY_BASED,
        ));

        spawn_skybox(world, "assets/venice_sunset_1k.hdr");

        /*
        let worlds = world.get_single_component_mut::<Assets<World>>().unwrap();
        let gltf_world = worlds.load("assets/huge_medieval_battle_scene/scene.gltf");

        // Spawn a Handle<World> that will be replaced with the GlTf when it's loaded.
        let gltf_hierarchy = world.spawn(gltf_world);
        let scaled_down = world.spawn(Transform::new().with_scale(Vec3::fill(5.0)));
        set_parent(world, Some(scaled_down), gltf_hierarchy);
        */
        // Spawn a series of balls with different material properties.
        // Up is more metallaic
        // Right is more more rough
        let spacing = 2.0;
        let mut commands = Commands::new();
        (|materials: &mut Assets<Material>| {
            let rows = 6;
            let columns = 6;
            for i in 0..rows {
                for j in 0..columns {
                    let new_material = materials.add(new_pbr_material(
                        Shader::PHYSICALLY_BASED,
                        PBRProperties {
                            base_color: Color::AZURE,
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

        |event, _world| {
            match event {
                Event::Draw => {}
                _ => {}
            }
            false
        }
    });
}
