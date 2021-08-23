//! Camera controls to be used by the editor or to quickly get a 3D camera up and running.
use crate::*;
use kapp::*;

pub fn camera_controls_plugin() -> Plugin {
    Plugin {
        fixed_update_systems: vec![update_camera_controls.system()],
        ..Default::default()
    }
}

#[derive(Clone, Component)]
pub struct CameraControls {
    velocity: Vec3,
    last_mouse_position: Option<Vec2>,
    pub max_speed: f32,
    pub acceleration: f32,
    pub friction: f32,
    pub rotation_sensitivity: f32,
}

impl CameraControls {
    pub fn new() -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: 4000.0,
            max_speed: 10.2,
            friction: 0.001,
            last_mouse_position: None,
            rotation_sensitivity: 1.5,
        }
    }
}

pub fn update_camera_controls(
    input: &Input,
    time: &Time,
    mut query: Query<(&mut CameraControls, &mut Camera, &mut Transform)>,
) {
    for (controls, camera, transform) in &mut query {
        // Handle rotation with the mouse
        if input.pointer_button(PointerButton::Secondary) {
            let position = input.pointer_position();
            let position = Vec2::new(position.0 as f32, position.1 as f32);

            if let Some(last_mouse_position) = controls.last_mouse_position {
                let (view_width, view_height) = camera.get_view_size();
                let (view_width, view_height) = (view_width as f32, view_height as f32);

                let difference = position - last_mouse_position;
                let rotation_pitch = Quat::from_yaw_pitch_roll(
                    0.,
                    -(difference[1] / view_height) * controls.rotation_sensitivity,
                    0.,
                );

                let rotation_yaw = Quat::from_yaw_pitch_roll(
                    -(difference[0] / view_width) * controls.rotation_sensitivity,
                    0.,
                    0.,
                );

                transform.rotation = rotation_yaw * transform.rotation * rotation_pitch;
            }

            controls.last_mouse_position = Some(position);
        } else {
            controls.last_mouse_position = None;
        }

        let mut direction = Vec3::ZERO;
        if input.key(Key::W) {
            direction += transform.forward();
        }

        if input.key(Key::S) {
            direction += transform.back();
        }

        if input.key(Key::A) {
            direction += transform.left();
        }

        if input.key(Key::D) {
            direction += transform.right();
        }

        // Up and down controls
        if input.key(Key::E) {
            direction += transform.up();
        }

        if input.key(Key::Q) {
            direction += transform.down();
        }

        if direction != Vec3::ZERO {
            controls.velocity +=
                direction.normalized() * controls.acceleration * time.delta_seconds_f64 as f32;
        } else {
            controls.velocity *= controls.friction.powf(time.delta_seconds_f64 as f32);
        }

        if controls.velocity.length() > controls.max_speed {
            controls.velocity = controls.velocity.normalized() * controls.max_speed;
        }

        transform.position += controls.velocity * time.delta_seconds_f64 as f32;
    }
}