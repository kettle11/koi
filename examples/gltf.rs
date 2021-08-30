use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        let camera = Camera::new();

        // Spawn a camera
        world.spawn((
            Transform::new_with_position(Vec3::new(0., 0., 4.0)),
            camera,
            CameraControls::new_with_mode(CameraControlsMode::Orbit { target: Vec3::ZERO }),
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
        let gltf_world = worlds.load(&"assets/tv/scene.gltf");
        let _ = world.spawn((Transform::new(), gltf_world));

        let mut camera_distance_scale = 1.0;
        move |event: Event, world: &mut World| match event {
            Event::FixedUpdate => {
                // Scale the camera to contain the scene's contents.
                let bounding_box = calculate_bounding_box_of_scene.run(world).unwrap();
                (|mut cameras: Query<(&mut Transform, &mut CameraControls)>| {
                    for (transform, camera_controls) in &mut cameras {
                        match &mut camera_controls.mode {
                            CameraControlsMode::Orbit { target } => {
                                *target = bounding_box.center();
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
