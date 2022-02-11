use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Spawn a camera
        let mut camera = Camera::new().with_orthographic();
        camera.set_orthographic_height(10.0);

        // Back up the camera so we can see the sprites.
        world.spawn((Transform::new().with_position(Vec3::Z * 3.0), camera));

        let textures = world.get_single_component_mut::<Assets<Texture>>().unwrap();
        let texture = textures.load_with_options(
            "examples/assets/tiles.png",
            // Load the texture with filtering more appropriate for pixel art
            TextureSettings {
                minification_filter: FilterMode::Nearest,
                magnification_filter: FilterMode::Nearest,
                ..Default::default()
            },
        );
        // A [SpriteMap] is a helper to make getting sprites from a texture easier.
        let sprite_map = SpriteMap::new(texture, 18, 2, 398, 178);

        // Enter the tile of the sprite.
        let snow_man_sprite = sprite_map.get_sprite(5, 7);

        for i in 0..5 {
            world.spawn((
                Transform::new().with_position(Vec3::X * i as f32),
                Mesh::VERTICAL_QUAD,
                Material::UNLIT,
                snow_man_sprite.clone(),
            ));
        }

        |_, _| false
    });
}
