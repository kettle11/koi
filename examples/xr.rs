use koi::*;

fn main() {
    App::new().setup_and_run(|world| {
        // Spawn a controllable camera
        let mut camera = Camera::new();
        camera.clear_color = Some(Color::BLUE);

        // The camera will be automatically updated to match the XR position.
        // So parent it to another entity that will be the thing moved around.
        let camera_parent = world.spawn(Transform::new().with_position([5.0, 0.0, 5.0].into()));
        let camera_entity = world.spawn((camera, Transform::new(), CameraControls::new()));
        set_parent(world, Some(camera_parent), camera_entity);

        let controller0 = world.spawn((
            XRController { id: 0 },
            Mesh::CUBE,
            Material::DEFAULT,
            Color::BLUE,
            Transform::new().with_scale(Vec3::fill(0.05)),
        ));
        let controller1 = world.spawn((
            XRController { id: 1 },
            Mesh::CUBE,
            Material::DEFAULT,
            Color::BLUE,
            Transform::new().with_scale(Vec3::fill(0.05)),
        ));
        set_parent(world, Some(camera_parent), controller0);
        set_parent(world, Some(camera_parent), controller1);

        // Spawn a light
        world.spawn((
            Transform {
                position: Vec3::new(27.708267, 57.67708, 35.69649),
                rotation: Quat::from_xyzw(0.28870586, -0.3600697, -0.11823557, -0.87920713),
                scale: Vec3::ONE,
            },
            Light::new(LightMode::Directional, Color::WHITE, 0.8),
            // ShadowCaster::new().with_ibl_shadowing(0.8),
        ));

        spawn_skybox_without_image_based_lighting(world, "assets/venice_sunset_1k.hdr");
        //spawn_skybox(world, "assets/venice_sunset_1k.hdr");

        let worlds = world.get_single_component_mut::<Assets<World>>().unwrap();
        let gltf_world = worlds.load("assets/silent_ash/scene.gltf");

        // Spawn a Handle<World> that will be replaced with the GlTf when it's loaded.
        let gltf_hierarchy = world.spawn(gltf_world);
        let scaled_down = world.spawn(Transform::new().with_scale(Vec3::fill(30.0)));
        set_parent(world, Some(scaled_down), gltf_hierarchy);

        // Spawn a cube
        //world.spawn((Transform::new(), Mesh::CUBE, Material::UNLIT, Color::RED));
        move |event: Event, world: &mut World| {
            match event {
                Event::Draw => {
                    let mut move_amount = 0.0;
                    let mut should_rotate = false;

                    (|input: &Input,
                      xr: &mut XR,
                      camera: &mut Camera,
                      controllers: Query<(&GlobalTransform, &XRController)>| {
                        if input.pointer_button_down(PointerButton::Primary) {
                            xr.start();
                        }

                        if xr.running() {
                            camera.clear_color = Some(Color::RED);
                        }

                        for (transform, controller) in &controllers {
                            if xr.button_state(controller.id, 0) {
                                move_amount = 0.5;
                            }
                            if xr.button_just_pressed(controller.id, 1) {
                                should_rotate = true;
                            }
                        }
                    })
                    .run(world);

                    let camera_forward = world
                        .get_component_mut::<GlobalTransform>(camera_entity)
                        .unwrap()
                        .forward();

                    let parent_transform =
                        world.get_component_mut::<Transform>(camera_parent).unwrap();
                    parent_transform.position += camera_forward * move_amount;

                    if should_rotate {
                        parent_transform.rotation = parent_transform.rotation
                            * Quat::from_angle_axis(std::f32::consts::TAU * 0.25, Vec3::Y);
                    }
                }
                _ => {}
            };
            false
        }
    })
}
