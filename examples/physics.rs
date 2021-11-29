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
            Transform::new().with_position(Vec3::Y * 3.0),
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

        move |event: Event, world: &mut World| {
            match event {
                Event::FixedUpdate => {
                    // Perform physics and game related updates here.
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
