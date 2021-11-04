use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Spawn a camera
        let mut camera = Camera::new_orthographic();
        camera.set_orthographic_height(1.0);
        world.spawn((Transform::new(), camera));

        let textures = world.get_single_component_mut::<Assets<Texture>>().unwrap();
        let texture =
            textures.load_with_options("assets/royal_esplanade_1k.hdr", TextureSettings::default());

        let sprite = Sprite::new(texture, BoundingBox::new(Vec2::ZERO, Vec2::ONE));

        world.spawn((
            Transform::new(),
            Mesh::VERTICAL_QUAD,
            Material::UNLIT,
            sprite,
        ));

        |_, _| false
    });
}
