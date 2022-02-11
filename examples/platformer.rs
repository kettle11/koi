use koi::*;

#[derive(Component, Clone)]
struct Character {
    running: bool,
    current_sprite: usize,
    sprites: [Sprite; 2],
    sprite_timer: f32,
}

#[derive(Component, Clone)]
struct Controlled;

impl Character {
    pub fn new(sprites: [Sprite; 2]) -> Self {
        Self {
            running: false,
            current_sprite: 0,
            sprites,
            sprite_timer: 0.,
        }
    }
}

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Spawn a camera
        let mut camera = Camera::new().with_orthographic_projection();
        camera.set_orthographic_height(10.0);

        // Use lightness to make a nice shader of blue
        camera.clear_color = Some(Color::BLUE.with_lightness(0.8));

        // Back up the camera so we can see the sprites.
        world.spawn((Transform::new().with_position(Vec3::Z * 3.0), camera));

        let textures = world.get_single_component_mut::<Assets<Texture>>().unwrap();

        let characters_texture = textures.load_with_options(
            "examples/assets/characters.png",
            // Load the texture with filtering more appropriate for pixel art
            TextureSettings {
                minification_filter: FilterMode::Nearest,
                magnification_filter: FilterMode::Nearest,
                ..Default::default()
            },
        );
        let tiles_texture = textures.load_with_options(
            "examples/assets/tiles.png",
            // Load the texture with filtering more appropriate for pixel art
            TextureSettings {
                minification_filter: FilterMode::Nearest,
                magnification_filter: FilterMode::Nearest,
                ..Default::default()
            },
        );
        // A [SpriteMap] is a helper to make getting sprites from a texture easier.
        let tiles_sprite_map = SpriteMap::new(tiles_texture, 18, 2, 398, 178);

        let character_sprite_map = SpriteMap::new(characters_texture, 24, 2, 232, 76);

        // Enter the tile of the sprite.
        let snow_man_sprite = tiles_sprite_map.get_sprite(5, 7);
        let middle_platform_sprite = tiles_sprite_map.get_sprite(2, 0);

        let character_sprites = [
            character_sprite_map.get_sprite(0, 0),
            character_sprite_map.get_sprite(1, 0),
        ];

        for i in 0..5 {
            world.spawn(sprite_bundle(
                Transform::new().with_position(Vec3::X * i as f32),
                middle_platform_sprite.clone(),
            ));
        }

        world.spawn((
            Transform::new().with_position(Vec3::X * 4.0 as f32),
            Mesh::VERTICAL_QUAD,
            Material::UNLIT_TRANSPARENT,
            character_sprites[0].clone(),
            Character::new(character_sprites),
            Controlled,
        ));

        |event, world| {
            match event {
                Event::FixedUpdate => {
                    (|time: &Time, mut characters: Query<(&mut Character, &mut Sprite)>| {
                        // Animate sprites
                        for (character, sprite) in characters.iter_mut() {
                            if character.running {
                                let animate_speed = 0.3;
                                character.sprite_timer += time.fixed_time_step as f32;
                                if character.sprite_timer > animate_speed {
                                    character.current_sprite += 1;
                                    if character.current_sprite >= character.sprites.len() {
                                        character.current_sprite = 0;
                                    }
                                    *sprite = character.sprites[character.current_sprite].clone();
                                    character.sprite_timer -= animate_speed;
                                }
                            } else {
                                character.sprite_timer = 0.5;
                                *sprite = character.sprites[1].clone();
                            }
                        }
                    })
                    .run(world);

                    // Move controlled characters
                    (|time: &Time,
                      input: &mut Input,
                      mut characters: Query<(
                        &mut Transform,
                        Option<&mut Character>,
                        &Controlled,
                    )>| {
                        let speed = 2.0;
                        for (transform, character, _) in characters.iter_mut() {
                            let mut input_pressed = false;
                            if input.key(Key::Left) {
                                transform.position.x -= speed * time.fixed_time_step as f32;
                                transform.rotation =
                                    Quat::from_angle_axis(0.0 * std::f32::consts::TAU, Vec3::Y);
                                input_pressed = true;
                            }
                            if input.key(Key::Right) {
                                transform.position.x += speed * time.fixed_time_step as f32;
                                transform.rotation =
                                    Quat::from_angle_axis(0.5 * std::f32::consts::TAU, Vec3::Y);
                                input_pressed = true;
                            }
                            if let Some(character) = character {
                                character.running = input_pressed;
                            }
                        }
                    })
                    .run(world);
                }
                _ => {}
            }
            false
        }
    });
}

fn sprite_bundle(
    transform: Transform,
    sprite: Sprite,
) -> (Transform, Handle<Mesh>, Handle<Material>, Sprite) {
    (
        transform,
        Mesh::VERTICAL_QUAD,
        Material::UNLIT_TRANSPARENT,
        sprite,
    )
}
