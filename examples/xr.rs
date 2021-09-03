use koi::*;

fn main() {
    App::new().setup_and_run(setup_and_run)
}

fn setup_and_run(world: &mut World) -> impl FnMut(Event, &mut koi::World) {
    // Spawn a controllable camera
    let mut camera = Camera::new();
    camera.clear_color = Some(Color::BLUE);

    world.spawn((
        camera,
        Transform::new_with_position([0.0, 0.0, 20.0].into()),
        CameraControls::new(),
    ));

    let light = Light::new(LightMode::Directional, Color::WHITE, 1.0);

    // Spawn a light
    world.spawn((
        light,
        Transform::new_with_position([0., 8.0, 8.0].into()),
        Mesh::SPHERE,
        Material::UNLIT,
    ));

    world.spawn((
        Light::new(LightMode::Directional, Color::WHITE, 1.0),
        Transform::new_with_position([-4., 4.0, -4.0].into()),
        Mesh::SPHERE,
        Material::DEFAULT,
    ));

    // Spawn a cube
    world.spawn((Transform::new(), Mesh::CUBE, Material::UNLIT, Color::RED));

    |event: Event, world: &mut World| match event {
        Event::Draw => {
            (|input: &Input, xr: &mut XR, camera: &mut Camera| {
                if input.pointer_just_pressed(PointerButton::Primary) {
                    xr.start();
                }

                if xr.running() {
                    camera.clear_color = Some(Color::RED);
                }
            })
            .run(world)
            .unwrap();
        }
        _ => {}
    }
}
