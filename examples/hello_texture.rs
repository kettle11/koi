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

        let cube_maps = world.get_single_component_mut::<Assets<CubeMap>>().unwrap();
        let reflection_probe = cube_maps.load_reflection_probe("assets/shudu_lake_1k.hdr");

        let skybox_material = (|graphics: &mut Graphics,
                                shaders: &mut Assets<Shader>,
                                materials: &mut Assets<Material>| {
            let shader = shaders.add(
                graphics
                    .new_shader(
                        include_str!("../src/graphics/built_in_shaders/skybox.glsl"),
                        PipelineSettings::default(),
                    )
                    .unwrap(),
            );
            let mut material = Material::new(shader);
            material.set_cube_map("p_environment_map", reflection_probe.source.clone());
            materials.add(material)
        })
        .run(world);

        world.spawn((
            Transform::new(),
            Mesh::CUBE_MAP_CUBE,
            Color::WHITE,
            Texture::WHITE,
            skybox_material.clone(),
        ));

        world.spawn((Transform::new(), reflection_probe.clone()));

        // world.spawn((Mesh::CUBE, Material::DEFAULT, Color::RED, Transform::new()));

        let path = "assets/old_rowboat/scene.gltf";

        // Begin loading a GlTf
        let worlds = world.get_single_component_mut::<Assets<World>>().unwrap();
        let gltf_world = worlds.load(path);
        // Spawn a Handle<World> that will be replaced with the GlTf when it's loaded.
        let gltf_hierarchy = world.spawn(gltf_world);
        let scaled_down = world.spawn(Transform::new().with_scale(Vec3::fill(0.1)));
        set_parent(world, Some(scaled_down), gltf_hierarchy);

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

        let light_positions = [
            Vec3::new(-10.0, 10.0, 10.0),
            Vec3::new(10.0, 10.0, 10.0),
            Vec3::new(-10.0, -10.0, 10.0),
            Vec3::new(10.0, -10.0, 10.0),
        ];

        for i in 0..4 {
            world.spawn((
                Transform::new().with_position(light_positions[i]),
                Light::new(LightMode::Point { radius: 1.0 }, Color::WHITE, 300.),
                Mesh::SPHERE,
                Material::UNLIT,
                Color::RED,
            ));
        }

        /*
        world.spawn((
            Light::new(LightMode::Directional, Color::WHITE, 30.0),
            Transform::new().with_position(Vec3::new(0., 8.0, 8.0)),
            Mesh::SPHERE,
            Material::DEFAULT,
        ));
        */

        /*
        let sprite = Sprite::new(texture, Box2::new(Vec2::ZERO, Vec2::ONE));

        world.spawn((
            Transform::new(),
            Mesh::VERTICAL_QUAD,
            Material::UNLIT,
            sprite,
        ));
        */

        let mut toggle = true;

        move |event, world| {
            match event {
                Event::Draw => {
                    (|input: &Input, materials: &mut Assets<Material>| {
                        if input.key_down(Key::Space) {
                            toggle = !toggle;
                            println!("TOGGLING!");
                        }
                        if toggle {
                            materials
                                .get_mut(&skybox_material)
                                .set_cube_map("p_environment_map", reflection_probe.source.clone())
                        } else {
                            materials.get_mut(&skybox_material).set_cube_map(
                                "p_environment_map",
                                reflection_probe.diffuse_irradiance_map.clone(),
                            )
                        }
                    })
                    .run(world);
                }
                _ => {}
            }
            false
        }
    });
}
