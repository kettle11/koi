use koi::*;

#[derive(Component, Clone)]
struct Controlled;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.
        // Spawn a camera and make it look towards the origin.
        let mut transform = Transform::new_with_position(Vec3::new(0.0, 4.0, 3.0));
        transform.look_at(Vec3::ZERO, Vec3::Y);
        world.spawn((transform, Camera::new(), CameraControls::new()));

        // Spawn a cube that we can control
        world.spawn((Transform::new(), Mesh::CUBE, Material::UNLIT, Controlled));

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
                    .run(world)
                }
                Event::Draw => {
                    // Things that occur before rendering can go here.
                }
                _ => {}
            }
        }
    });
}
