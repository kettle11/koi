use koi::*;

// Custom components need to derive "Component".
#[derive(Component, Clone)]
struct Thingy;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.
        let mut camera = Camera::new();
        camera.clear_color = Some(Color::RED);
        world.spawn((Transform::new(), camera, CameraControls::new()));
        world.spawn((
            Transform::new_with_position(Vec3::new(0.0, 0.0, -3.0)),
            Mesh::CUBE,
            Material::PHYSICALLY_BASED,
        ));

        // Spawn a light
        world.spawn((
            Light::new(LightMode::Directional, Color::WHITE, 1.0),
            Transform::new_with_position([0., 8.0, 8.0].into()),
            Mesh::SPHERE,
            Material::UNLIT,
        ));

        // Run the World with this mutable closure.
        move |event: Event, _: &mut World| {
            match event {
                Event::FixedUpdate => {
                    //println!("Hello!: {:?}", thingy)
                }
                Event::Draw => {
                    // Things that occur before rendering can go here.
                }
            }
        }
    });
}
