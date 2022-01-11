use koi::*;

#[derive(Component, Clone)]
struct Controlled;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.

        // Spawn a light
        world.spawn((
            Light::new(LightMode::Directional, Color::WHITE, 1.0),
            Transform::new()
                .with_position([0., 8.0, 8.0].into())
                .looking_at(Vec3::ZERO, Vec3::Y),
        ));

        world.spawn((
            Light::new(LightMode::Directional, Color::WHITE, 1.0),
            Transform::new()
                .with_position([0., -8.0, -8.0].into())
                .looking_at(Vec3::ZERO, Vec3::Y),
        ));

        let mut camera = Camera::new();
        camera.clear_color = Some(Color::WHITE);
        // Spawn a camera and make it look towards the origin.
        world.spawn((
            Transform::new()
                .with_position(Vec3::new(0.0, 4.0, 3.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            CameraControls::new(),
            camera,
        ));

        let collision_marker_a = world.spawn((
            Transform::new().with_scale(Vec3::fill(0.1)),
            Mesh::SPHERE,
            Material::UNLIT,
            Color::BLUE,
        ));
        let collision_marker_b = world.spawn((
            Transform::new().with_scale(Vec3::fill(0.1)),
            Mesh::SPHERE,
            Material::UNLIT,
            Color::GREEN,
        ));

        let mut contact_markers = Vec::new();
        for _ in 0..4 {
            let e = world.spawn((
                Transform::new().with_scale(Vec3::fill(0.1)),
                Mesh::SPHERE,
                Material::UNLIT,
                Color::from_srgb_hex(0xFFFF00, 1.0),
            ));
            contact_markers.push(e);
        }

        // Spawn a cube that we can control
        let object_a = world.spawn((
            Transform::new().with_position(Vec3::new(0.600000024, 0.927636206, 0.0)),
            // Transform::new().with_position(Vec3::Y * 0.75 + Vec3::X * 0.9),
            Mesh::CUBE,
            Controlled,
            Color::WHITE,
            Material::PHYSICALLY_BASED_TRANSPARENT,
        ));
        let object_b = world.spawn((
            Transform::new(),
            // Transform::new().with_rotation(Random::new_with_seed(2).quaternion()),
            // .with_rotation(Random::new_with_seed(2).quaternion()),
            /*
            .with_rotation(Quat::from_yaw_pitch_roll(
                0.0,
                std::f32::consts::TAU * 0.125,
                std::f32::consts::TAU * 0.125,
            )), //.with_position(Vec3::Y * 0.8 + Vec3::X * 0.6),
            */
            Mesh::CUBE,
            Color::WHITE,
            Material::PHYSICALLY_BASED_TRANSPARENT,
        ));

        let mut rotation_angle = 0.0;
        move |event: Event, world: &mut World| {
            match event {
                Event::FixedUpdate => {
                    // Perform physics and game related updates here.

                    // Control the cube.
                    (|input: &Input, mut things_to_move: Query<(&mut Transform, &Controlled)>| {
                        for (transform, _) in &mut things_to_move {
                            if input.key(Key::Left) {
                                transform.position -= Vec3::X * 0.1;
                            }
                            if input.key(Key::Right) {
                                transform.position += Vec3::X * 0.1;
                            }
                            if input.key(Key::Up) {
                                transform.position -= Vec3::Z * 0.1;
                            }
                            if input.key(Key::Down) {
                                transform.position += Vec3::Z * 0.1;
                            }
                            if input.key(Key::NumPad2) {
                                transform.position -= Vec3::Y * 0.1;
                            }
                            if input.key(Key::NumPad8) {
                                transform.position += Vec3::Y * 0.1;
                            }
                            if input.key(Key::O) {
                                rotation_angle -= 0.01;
                                transform.rotation = Quat::from_angle_axis(rotation_angle, Vec3::Y);
                            }
                            if input.key(Key::P) {
                                rotation_angle += 0.01;
                                transform.rotation = Quat::from_angle_axis(rotation_angle, Vec3::Y);
                            }
                        }
                    })
                    .run(world);

                    (|meshes: &Assets<Mesh>,
                      mut objects: Query<(
                        &mut Transform,
                        &GlobalTransform,
                        &mut Color,
                        &Handle<Mesh>,
                    )>| {
                        let (_, transform_a, _, a_mesh) =
                            objects.get_entity_components(object_a).unwrap();
                        let (_, transform_b, _, b_mesh) =
                            objects.get_entity_components(object_b).unwrap();

                        // println!("position a: {:?}", transform_a.position);
                        // println!("position b: {:?}", transform_b.position);

                        let mesh_a = meshes.get(a_mesh);
                        let mesh_b = meshes.get(b_mesh);

                        let points_a = &mesh_a.mesh_data.as_ref().unwrap().positions;
                        let points_b = &mesh_b.mesh_data.as_ref().unwrap().positions;
                        let model_a = transform_a.model();
                        let model_b = transform_b.model();
                        let collision_info = kphysics::collision::gjk(
                            transform_a.model(),
                            transform_b.model(),
                            points_a,
                            points_b,
                        );

                        println!("COLLISION INFO: {:?}", collision_info);
                        if collision_info.collided {
                            let mesh_data_a = kphysics::create_mesh_data(
                                &mesh_a.mesh_data.as_ref().unwrap().positions,
                                &mesh_a.mesh_data.as_ref().unwrap().normals,
                                &mesh_a.mesh_data.as_ref().unwrap().indices,
                            );

                            let mesh_data_b = kphysics::create_mesh_data(
                                &mesh_b.mesh_data.as_ref().unwrap().positions,
                                &mesh_b.mesh_data.as_ref().unwrap().normals,
                                &mesh_b.mesh_data.as_ref().unwrap().indices,
                            );

                            let contact_points = kphysics::collision::find_contact_points_on_plane(
                                Plane {
                                    normal: -Vec3::Y,
                                    distance_along_normal: -0.427636206,
                                },
                                &mesh_data_a.planes,
                                &mesh_data_b.planes,
                                &model_a,
                                &model_b,
                            );
                            println!("CONTACT POINTS: {:?}", contact_points);

                            for (i, e) in contact_markers.iter().enumerate() {
                                if i < contact_points.len() {
                                    objects.get_entity_components_mut(*e).unwrap().0.position =
                                        contact_points[i];
                                } else {
                                    objects.get_entity_components_mut(*e).unwrap().0.position =
                                        Vec3::fill(1000.);
                                }
                            }
                        }
                        let new_color = if collision_info.collided {
                            let mut color = Color::RED;
                            color.alpha = 0.7;
                            color
                        } else {
                            Color::WHITE
                        };
                        *objects.get_entity_components_mut(object_a).unwrap().2 = new_color;
                        *objects.get_entity_components_mut(object_b).unwrap().2 = new_color;

                        objects
                            .get_entity_components_mut(collision_marker_a)
                            .unwrap()
                            .0
                            .position = collision_info.closest_point_a;

                        println!("POINT: {:?}", collision_info.closest_point_b);
                        objects
                            .get_entity_components_mut(collision_marker_b)
                            .unwrap()
                            .0
                            .position = collision_info.closest_point_b;
                    })
                    .run(world)
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
