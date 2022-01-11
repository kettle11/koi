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

        // Spawn a cube that we can control
        world.spawn((
            Transform::new().with_position(Vec3::Y * 1.3 + Vec3::X * 0.8),
            Mesh::CUBE,
            Material::DEFAULT,
            RigidBody::new(1.0),
            Collider::new(),
        ));

        // Spawn a cube that we can control
        world.spawn((
            Transform::new(),
            Mesh::CUBE,
            Material::DEFAULT,
            RigidBody::new(f32::INFINITY),
            Collider::new(),
        ));

        let mut step_physics = false;
        move |event: Event, world: &mut World| {
            match event {
                Event::FixedUpdate => {
                    let mut immediate_drawer = ImmediateDrawer::new();
                    immediate_drawer.set_material(&Material::UNLIT);
                    immediate_drawer.set_color(Color::RED);
                    (|physics_world: &PhysicsWorld| {
                        let contact_points = &physics_world.contact_points;
                        for p in contact_points {
                            immediate_drawer.draw_sphere(
                                Transform::new()
                                    .with_position(*p)
                                    .with_scale(Vec3::fill(0.1)),
                            )
                        }
                    })
                    .run(world);
                    immediate_drawer.apply(world);
                    // Perform physics and game related updates here.
                }
                Event::Draw => {
                    (|input: &Input, physics_world: &mut koi::PhysicsWorld| {
                        if physics_world.collision_occurred {
                            physics_world.paused = true;
                        }

                        if step_physics {
                            physics_world.paused = true;
                            step_physics = false;
                        }

                        if input.key_down(Key::P) {
                            println!("STEP");

                            physics_world.paused = false;
                            step_physics = true;
                        }
                    })
                    .run(world);

                    // Things that occur before rendering can go here.
                }
                _ => {}
            }

            // Do not consume the event and allow other systems to respond to it.
            false
        }
    });
}
