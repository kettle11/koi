use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        let textures = world.get_single_component_mut::<Assets<Texture>>().unwrap();
        let texture = textures.load("examples/assets/tiles.png");

        world.spawn((Transform::new(), Camera::new_orthographic()));
        world.spawn((
            Transform::new(),
            Mesh::VERTICAL_QUAD,
            Material::UNLIT,
            texture,
        ));

        |_, _| {}
    });
}
