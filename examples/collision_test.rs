use koi::*;

#[derive(Component, Clone)]
struct Controlled;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.

        // Spawn a light
        world.spawn((
            Light::new(LightMode::Directional, Color::WHITE, 1.0),
            Transform::new().with_position([0., 8.0, 8.0].into()),
            Material::UNLIT,
        ));

        // Spawn a camera and make it look towards the origin.
        world.spawn((
            Transform::new()
                .with_position(Vec3::new(0.0, 4.0, 3.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            Camera::new(),
            CameraControls::new(),
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

        // Spawn a cube that we can control
        let object_a = world.spawn((
            Transform::new().with_position(Vec3::X * 1.5),
            Mesh::CUBE,
            Controlled,
            Color::WHITE,
            Material::DEFAULT,
        ));
        let object_b = world.spawn((
            Transform::new(), //.with_rotation(Random::new().quaternion()),
            Mesh::CUBE,
            Color::WHITE,
            Material::DEFAULT,
        ));

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

                        println!("position a: {:?}", transform_a.position);
                        println!("position b: {:?}", transform_b.position);

                        let mesh_a = meshes.get(a_mesh);
                        let mesh_b = meshes.get(a_mesh);

                        let points_a = &mesh_a.mesh_data.as_ref().unwrap().positions;
                        let points_b = &mesh_b.mesh_data.as_ref().unwrap().positions;
                        let collision_info = kphysics::gjk(
                            transform_a.model(),
                            transform_b.model(),
                            points_a,
                            points_b,
                        );
                        let new_color = if collision_info.collided {
                            Color::RED
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
