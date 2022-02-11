use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.

        let mut camera = Camera::new();
        camera.clear_color = Some(Color::BLUE.with_lightness(0.8));

        // Spawn a camera and make it look towards the origin.
        world.spawn((
            Transform::new()
                .with_position(Vec3::new(3.0, 3.0, 3.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            camera,
            CameraControls::new(),
        ));

        let mut light = Light::new(LightMode::Directional, Color::WHITE, 8.);
        light.ambient_light_amount = 0.01;
        world.spawn((
            Transform::new()
                .with_position(Vec3::new(3.0, 3.0, 3.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            light,
            ShadowCaster::new(),
        ));

        let worlds = world.get_singleton::<Assets<World>>();

        let tiles = [
            worlds.load("examples/assets/city_builder/square_forest.gltf.glb"),
            worlds.load("examples/assets/city_builder/square_rock.gltf.glb"),
            worlds.load("examples/assets/city_builder/square_water.gltf.glb"),
        ];

        world.spawn((Transform::new(), tiles[0].clone()));

        let mut random = Random::new();

        move |event: Event, world: &mut World| {
            match event {
                Event::FixedUpdate => {
                    let mut commands = Commands::new();

                    (|input: &Input, cameras: Query<(&Camera, &GlobalTransform)>| {
                        if input.pointer_button_down(PointerButton::Primary) {
                            let (x, y) = input.pointer_position();
                            let (camera, camera_global_transform) = cameras.iter().next().unwrap();
                            let ray =
                                camera.view_to_ray(camera_global_transform, x as f32, y as f32);
                            let plane = Plane::new(Vec3::Y, Vec3::ZERO);
                            let intersection = koi::intersections::ray_with_plane(ray, plane);
                            if let Some(intersection) = intersection {
                                commands.spawn((
                                    Transform::new().with_position(ray.get_point(intersection)),
                                    random.select_from_slice(&tiles).clone(),
                                ))
                            }
                        }
                    })
                    .run(world);
                    commands.apply(world);
                }
                Event::Draw => {
                    // Things that occur before rendering can go here.
                }
                _ => {}
            }

            // Do not consume the event and allow other systems to respond to it.
            false
        }
    });
}
