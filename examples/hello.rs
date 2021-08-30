use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.
        let mut camera = Camera::new();
        camera.clear_color = Some(Color::RED);
        let mut transform = Transform::new_with_position(Vec3::new(-3.0, 0.0, 0.0));
        transform.look_at(Vec3::ZERO, Vec3::Y);
        world.spawn((
            transform,
            camera,
            //  CameraControls::new(),
            CameraControls::new_with_mode(CameraControlsMode::Orbit { target: Vec3::ZERO }),
        ));
        world.spawn((Transform::new(), Mesh::CUBE, Material::PHYSICALLY_BASED));

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
