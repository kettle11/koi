use koi::*;

#[derive(Component, Clone)]
struct Controlled;

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

        // Spawn a cube that we can control
        let object_a = world.spawn((Transform::new(), Mesh::CUBE, Material::UNLIT, Controlled, Color::WHITE));
        let object_b = world.spawn((Transform::new(), Mesh::CUBE, Material::UNLIT, Color::WHITE));

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
                        }
                    })
                    .run(world);

                    (|meshes: &Assets<Mesh>, mut objects: Query<(&GlobalTransform, &mut Color, &Handle<Mesh>)>| {
                       let (transform_a, _, a_mesh) = objects.get_entity_components(object_a).unwrap();
                       let (transform_b, _, b_mesh) = objects.get_entity_components(object_b).unwrap();
                       let mesh_a = meshes.get(a_mesh);
                       let mesh_b = meshes.get(a_mesh);

                        let intersects = check_intersection(transform_a, transform_b, &mesh_a.mesh_data.as_ref().unwrap().positions, &mesh_b.mesh_data.as_ref().unwrap().positions);
                        let new_color = if intersects {
                            Color::RED
                        } else {
                            Color::WHITE
                        };
                        for (_, color, _) in &mut objects {
                            *color = new_color;
                        }
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

fn check_intersection(
    transform_a: &GlobalTransform,
    transform_b: &GlobalTransform,
    points_a: &[Vec3],
    points_b: &[Vec3],
) -> bool {
    let inverse_a = transform_a.model() * transform_b.model().inversed();

    // Transform b into the space of a.
    let points_b: Vec<_> = points_b
        .iter()
        .map(|p| inverse_a.transform_point(*p))
        .collect();

    kphysics::gjk(&points_a, &points_b)
}
