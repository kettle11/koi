use koi::*;

#[derive(Component, Clone)]
struct Rotator;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        world.spawn((
            Camera::new(),
            Transform::new().with_position(Vec3::new(0.0, 0.0, 10.0)),
            CameraControls::new(),
        ));

        world.spawn((
            Transform::new().with_position(Vec3::new(0., 10.0, 8.0)),
            Light::new(LightMode::Directional, Color::WHITE, 300.),
        ));
        let mut parent_cube =
            world.spawn((Rotator, Mesh::CUBE, Material::DEFAULT, Transform::new()));

        for _ in 0..10 {
            let child_cube = world.spawn((
                Mesh::CUBE,
                Material::DEFAULT,
                Transform::new().with_position(Vec3::Y * 6.0),
            ));

            set_parent(world, Some(parent_cube), child_cube);
            parent_cube = child_cube;
        }

        // Run the World with this mutable closure.
        move |event: Event, world: &mut World| {
            match event {
                Event::FixedUpdate => {
                    (|mut query: Query<(&mut Rotator, &mut Transform)>| {
                        {
                            // Rotate the parent cube per frame.
                            for (_rotator, transform) in &mut query {
                                transform.rotation =
                                    Quat::from_angle_axis(0.05, Vec3::X) * transform.rotation
                            }
                        }
                    })
                    .run(world)
                }
                _ => {}
            }
            false
        }
    });
}
