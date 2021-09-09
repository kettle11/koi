/// This is a really messy example of building a basic platformer.
use glow::*;
use kapp::*;

#[derive(Clone, Copy)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    pub fn bottom(&self) -> f32 {
        self.y
    }

    pub fn top(&self) -> f32 {
        self.y + self.height
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn left(&self) -> f32 {
        self.x
    }
}

struct Player {
    velocity: (f32, f32),
    rect: Rect,
    grounded: bool,
}

struct Block {
    rect: Rect,
    color: Color,
}
// Draw a rect using glScissor.
// This is not a good way to draw rectangles!
// Use a proper library with shaders!
fn draw_rect(gl: &Context, rect: &Rect, color: &Color, scale: f64) {
    unsafe {
        let scale = scale as f32;
        gl.scissor(
            (rect.x * scale) as i32,
            (rect.y * scale) as i32,
            (rect.width * scale) as i32,
            (rect.height * scale) as i32,
        );
        gl.clear_color(color.r, color.g, color.b, color.a);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
    }
}

// Make sure the rect doesn't leave the screen
// And bounce it off the walls if it does
// Also detect if the rect is touching the floor.
fn check_rect_bounds(
    rect: &mut Rect,
    velocity: &mut (f32, f32),
    screen_width: i32,
    screen_height: i32,
    grounded: &mut bool,
) {
    if rect.bottom() < 0. {
        rect.y = 0.;
        velocity.1 = 0.;
        *grounded = true;
    }

    if rect.left() < 0. {
        rect.x = 0.;
    }

    if rect.right() > screen_width as f32 {
        rect.x = screen_width as f32 - rect.width;
    }

    if rect.top() > screen_height as f32 {
        rect.y = screen_height as f32 - rect.height;
    }
}

// Check if two rectangles overlap and if so return penetration depths
fn rect_overlap(rect0: &Rect, rect1: &Rect) -> Option<(f32, f32)> {
    if rect0.left() < rect1.right()
        && rect0.right() > rect1.left()
        && rect0.bottom() < rect1.top()
        && rect0.top() > rect1.bottom()
    {
        let penetration_y0 = rect0.bottom() - rect1.top();
        let penetration_y1 = rect0.top() - rect1.bottom();
        let penetration_x0 = rect0.left() - rect1.right();
        let penetration_x1 = rect0.right() - rect1.left();
        // Find the direction along the y axis that penetrates the least.
        let penetration_y = if penetration_y0.abs() < penetration_y1.abs() {
            penetration_y0
        } else {
            penetration_y1
        };

        let penetration_x = if penetration_x0.abs() < penetration_x1.abs() {
            penetration_x0
        } else {
            penetration_x1
        };
        Some((penetration_x, penetration_y))
    } else {
        None
    }
}

fn main() {
    // Create a new application with default settings.
    let (app, event_loop) = initialize();

    let mut screen_width = 500;
    let mut screen_height = 500;

    let mut gl_context = GLContext::new().build().unwrap(); // Create a gl_context for the app
    #[cfg(target_arch = "wasm32")]
    let gl = glow::Context::from_webgl1_context(gl_context.webgl1_context().unwrap());
    #[cfg(not(target_arch = "wasm32"))]
    let gl = glow::Context::from_loader_function(|s| gl_context.get_proc_address(s));

    unsafe {
        gl.enable(SCISSOR_TEST);
    }

    let window = app
        .new_window()
        .title("Platformer")
        .size(screen_width, screen_height)
        .build()
        .unwrap();

    gl_context.set_window(Some(&window)).unwrap();

    // ---------------- Level Data -------------------
    let black = (0.0, 0.0, 0.0, 1.0);

    let murky_grey = Color {
        r: 0.05,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    let background_blocks = vec![
        Block {
            rect: Rect {
                x: 0.,
                y: 0.,
                width: 150.,
                height: 200.,
            },
            color: murky_grey,
        },
        Block {
            rect: Rect {
                x: 130.,
                y: 0.,
                width: 150.,
                height: 300.,
            },
            color: murky_grey,
        },
        Block {
            rect: Rect {
                x: 180.,
                y: 0.,
                width: 200.,
                height: 470.,
            },
            color: murky_grey,
        },
        Block {
            rect: Rect {
                x: 250.,
                y: 0.,
                width: 300.,
                height: 140.,
            },
            color: murky_grey,
        },
    ];

    let interactive_color = Color {
        r: 0.05,
        g: 0.7,
        b: 0.15,
        a: 1.0,
    };
    let interactive_blocks = vec![
        Block {
            rect: Rect {
                x: 90.,
                y: 0.,
                width: 139.,
                height: 15.,
            },
            color: interactive_color,
        },
        Block {
            rect: Rect {
                x: 220.,
                y: 0.,
                width: 40.,
                height: 50.,
            },
            color: interactive_color,
        },
        Block {
            rect: Rect {
                x: 170.,
                y: 100.,
                width: 40.,
                height: 50.,
            },
            color: interactive_color,
        },
        Block {
            rect: Rect {
                x: 230.,
                y: 140.,
                width: 80.,
                height: 40.,
            },
            color: interactive_color,
        },
        Block {
            rect: Rect {
                x: 400.,
                y: 30.,
                width: 40.,
                height: 80.,
            },
            color: interactive_color,
        },
        Block {
            rect: Rect {
                x: 340.,
                y: 40.,
                width: 30.,
                height: 30.,
            },
            color: interactive_color,
        },
        Block {
            rect: Rect {
                x: 320.,
                y: 140.,
                width: 30.,
                height: 20.,
            },
            color: interactive_color,
        },
    ];

    let moody_foreground_waterfall = Color {
        r: 0.01,
        g: 0.02,
        b: 0.8,
        a: 1.0,
    };
    let foreground_blocks = vec![
        Block {
            rect: Rect {
                x: 260.,
                y: 176.,
                width: 20.,
                height: 300.,
            },
            color: moody_foreground_waterfall,
        },
        Block {
            rect: Rect {
                x: 220.,
                y: 170.,
                width: 100.,
                height: 10.,
            },
            color: moody_foreground_waterfall,
        },
        Block {
            rect: Rect {
                x: 220.,
                y: 0.,
                width: 20.,
                height: 170.,
            },
            color: moody_foreground_waterfall,
        },
        Block {
            rect: Rect {
                x: 300.,
                y: 0.,
                width: 20.,
                height: 170.,
            },
            color: moody_foreground_waterfall,
        },
    ];
    // ---------------- End level data -------------------

    let mut player = Player {
        rect: Rect {
            x: 0.,
            y: 0.,
            width: 20.,
            height: 30.,
        },
        velocity: (0., 0.),
        grounded: false,
    };

    // Various player parameters.
    let player_color = Color {
        r: 0.0,
        g: 0.3,
        b: 0.6,
        a: 1.0,
    };

    let player_ground_acceleration = 0.5;
    let player_air_acceleration = 0.15;
    let ground_friction = 0.2;
    let air_friction = 0.05;
    let gravity = 0.4;
    let jump_power = 7.0;

    let mut right_held = false;
    let mut left_held = false;

    event_loop.run(move |event| unsafe {
        match event {
            Event::WindowCloseRequested { .. } => app.quit(),
            Event::WindowResized { width, height, .. } => {
                gl_context.resize(); // Resizes the window buffer
                screen_width = width;
                screen_height = height;
            }
            Event::KeyDown { key, .. } => match key {
                Key::Left => {
                    left_held = true;
                    // This gives the player a little extra velocity when they hit a button.
                    // It helps make the controls feel more responsive.
                    player.velocity.1 -= player_ground_acceleration * 0.2;
                }
                Key::Right => {
                    right_held = true;
                    player.velocity.1 += player_ground_acceleration * 0.2;
                }
                Key::Space => {
                    if player.grounded {
                        player.velocity.1 += jump_power
                    }
                } // Jump!
                Key::V => {
                    gl_context.set_vsync(VSync::On).unwrap();
                }
                Key::A => {
                    gl_context.set_vsync(VSync::Adaptive).unwrap();
                }
                Key::O => {
                    gl_context.set_vsync(VSync::Off).unwrap();
                }
                Key::F => window.fullscreen(),
                Key::Escape => window.restore(),
                _ => {}
            },
            Event::KeyUp { key, .. } => match key {
                Key::Left => left_held = false,
                Key::Right => right_held = false,
                _ => {}
            },
            Event::Draw { .. } => {
                // First update the world

                // It feels incorrect if the player can control too tightly while jumping, so change air acceleration here.
                // We also want less friction in the air so the player can 'fling' themselves with jumps.
                let (friction, player_acceleration) = if player.grounded {
                    (ground_friction, player_ground_acceleration)
                } else {
                    (air_friction, player_air_acceleration)
                };

                // This is where we respond to player input by changing velocity.
                if left_held {
                    player.velocity.0 -= player_acceleration;
                }

                if right_held {
                    player.velocity.0 += player_acceleration;
                }

                // Apply friction and gravity.
                player.velocity.0 -= player.velocity.0 * friction;
                player.velocity.1 -= gravity;

                // Here we move the player's position based on velocity.
                player.rect.x += player.velocity.0;
                player.rect.y += player.velocity.1;

                // This function bounds the player within the screen, and sets their Y-velocity to 0 if they touch the bottom of the screen.
                // If the player touches the bottom of the screen they're 'grounded'.
                player.grounded = false;
                check_rect_bounds(
                    &mut player.rect,
                    &mut player.velocity,
                    screen_width as i32,
                    screen_height as i32,
                    &mut player.grounded,
                );

                // Check if the player is colliding with any blocks:
                for block in interactive_blocks.iter() {
                    // This function returns the penetration depth of the player with a block along the x and y axis.
                    if let Some(collision) = rect_overlap(&player.rect, &block.rect) {
                        if collision.0.abs() < collision.1.abs() {
                            player.rect.x -= collision.0;
                            player.velocity.0 = 0.;
                        } else {
                            // If the player collides downwards along the y axis they're on the ground and can jump.
                            if collision.1 < 0. {
                                player.grounded = true;
                            }
                            player.rect.y -= collision.1;
                            player.velocity.1 = 0.;
                        }
                    }
                }

                // Now begin drawing!
                // First clear the screen
                gl.scissor(0, 0, screen_width as i32, screen_height as i32);
                gl.clear_color(black.0, black.1, black.2, black.3);
                gl.clear(COLOR_BUFFER_BIT);

                let scale = 1.0; //window.backing_scale();
                                 // Draw the background!
                for block in background_blocks.iter() {
                    draw_rect(&gl, &block.rect, &block.color, scale);
                }

                // Then draw the blocks the player can interact with
                for block in interactive_blocks.iter() {
                    draw_rect(&gl, &block.rect, &block.color, scale);
                }
                // Then draw the player
                draw_rect(&gl, &player.rect, &player_color, scale);

                // Draw the foreground!
                for block in foreground_blocks.iter() {
                    draw_rect(&gl, &block.rect, &block.color, scale);
                }

                // When we're done rendering swap the window buffers to display to the screen.
                gl_context.swap_buffers();

                window.request_redraw();
            }
            _ => {}
        }
    });
}
