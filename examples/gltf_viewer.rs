use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        let camera = Camera::new();
        let mut camera_controls =
            CameraControls::new_with_mode(CameraControlsMode::Orbit { target: Vec3::ZERO });
        camera_controls.rotate_button = PointerButton::Primary;
        camera_controls.panning_mouse_button = Some(PointerButton::Secondary);

        // Spawn a camera
        world.spawn((
            Transform::new_with_position(Vec3::new(0., 0., 4.0)),
            camera,
            camera_controls,
        ));

        let mut light_transform = Transform::new_with_position([0., 8.0, 8.0].into());
        light_transform.look_at(Vec3::ZERO, Vec3::Y);

        // Spawn a light
        world.spawn((
            light_transform,
            Light::new(LightMode::Directional, Color::WHITE, 5.0),
            Material::UNLIT,
        ));

        // Spawn a loaded gltf
        let worlds = world.get_single_component_mut::<Assets<World>>().unwrap();

        #[cfg(target_arch = "wasm32")]
        let path = {
            let path = kwasm::libraries::eval(
                r#"
            let string = window.location.search;
            self.kwasm_pass_string_to_client(string.substring(1));
            "#,
            );
            kwasm::get_string_from_host()
        };
        #[cfg(not(target_arch = "wasm32"))]
        let path = "assets/silent_ash/scene.gltf".to_string();

        log!("GLTF PATH: {:?}", path);
        let gltf_world = worlds.load(&path);

        let mut loaded = false;
        let mut camera_distance_scale = 1.0;

        move |event: Event, world: &mut World| match event {
            Event::FixedUpdate => {
                if !loaded {
                    let new_world = (|worlds: &mut Assets<World>| {
                        if !worlds.is_placeholder(&gltf_world) {
                            loaded = true;
                            Some(worlds.get_mut(&gltf_world).clone_world())
                        } else {
                            None
                        }
                    })
                    .run(world)
                    .unwrap();

                    if let Some(mut new_world) = new_world {
                        world.add_world(&mut new_world);
                        update_global_transforms.run(world).unwrap();

                        let bounding_box = calculate_bounding_box_of_scene.run(world).unwrap();

                        // Once the object is loaded position the camera to keep the object in frame.
                        (|mut cameras: Query<(&mut Transform, &mut CameraControls)>| {
                            for (transform, camera_controls) in &mut cameras {
                                match &mut camera_controls.mode {
                                    CameraControlsMode::Orbit { target } => {
                                        *target = bounding_box.center();
                                        let max_axis = bounding_box.size().max_component();

                                        transform.position = Vec3::new(1.0, 1.0, -1.0).normalized()
                                            * max_axis
                                            * 1.2
                                            * camera_distance_scale
                                            + *target;
                                    }
                                    _ => {}
                                }
                            }
                        })
                        .run(world)
                        .unwrap();
                    }
                }

                let bounding_box = calculate_bounding_box_of_scene.run(world).unwrap();

                // Scale the camera to contain the scene's contents.
                (|mut cameras: Query<(&mut Transform, &mut CameraControls)>| {
                    for (transform, camera_controls) in &mut cameras {
                        camera_controls.panning_scale = camera_distance_scale;
                        match &mut camera_controls.mode {
                            CameraControlsMode::Orbit { target } => {
                                let max_axis = bounding_box.size().max_component();
                                let diff = transform.position - *target;
                                transform.position =
                                    diff.normalized() * max_axis * 1.2 * camera_distance_scale
                                        + *target;
                            }
                            _ => {}
                        }
                    }
                })
                .run(world)
                .unwrap();
            }
            Event::KappEvent(event) => match event {
                kapp::Event::Scroll { delta_y, .. } => {
                    let delta_y = delta_y as f32;
                    let change = -delta_y / 300.0 + 1.0;
                    camera_distance_scale *= change;
                }
                kapp::Event::PinchGesture { delta, .. } => {
                    camera_distance_scale *= (-delta + 1.0) as f32;
                }
                _ => {}
            },
            _ => {}
        }
    });
}

fn calculate_bounding_box_of_scene(
    meshes: &Assets<Mesh>,
    entities: Query<(&Transform, &Handle<Mesh>)>,
) -> BoundingBox<f32, 3> {
    entities.iter().fold(
        BoundingBox::ZERO,
        |bounding_box, (transform, mesh_handle)| {
            let mesh = meshes.get(mesh_handle);
            if let Some(mesh_bounding_box) = mesh.bounding_box {
                let transformed_bounding_box = transform
                    .global_transform
                    .transform_bounding_box(mesh_bounding_box);
                return bounding_box.join(transformed_bounding_box);
            }

            bounding_box
        },
    )
}
