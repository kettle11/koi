//! Camera controls to be used by the editor or to quickly get a 3D camera up and running.

use crate::*;
use kapp::*;

pub fn camera_controls_plugin() -> Plugin {
    Plugin {
        fixed_update_systems: vec![update_camera_controls.system()],
        ..Default::default()
    }
}

#[derive(Clone, SerializeDeserialize)]
pub enum CameraControlsMode {
    Fly,
    Orbit { target: Vec3 },
}

#[derive(Clone, Component)]
pub struct CameraControls {
    velocity: Vec3,
    pub max_speed: f32,
    pub acceleration: f32,
    pub friction: f32,
    pub rotation_sensitivity: f32,
    pub mode: CameraControlsMode,
    pub rotate_button: PointerButton,
    pub panning_mouse_button: Option<PointerButton>,
    pub panning_scale: f32,
}

impl Default for CameraControls {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraControls {
    pub fn new() -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: 4000.0,
            max_speed: 15.2,
            friction: 0.01,
            rotation_sensitivity: 1.5,
            mode: CameraControlsMode::Fly,
            rotate_button: PointerButton::Secondary,
            panning_mouse_button: None,
            panning_scale: 1.0,
        }
    }

    pub fn new_with_mode(mode: CameraControlsMode) -> Self {
        let mut camera_controls = Self::new();
        camera_controls.mode = mode;
        camera_controls
    }
}

pub fn update_camera_controls(
    input: &Input,
    time: &Time,
    mut query: Query<(&mut CameraControls, &mut Camera, &mut Transform)>,
) {
    for (controls, camera, transform) in &mut query {
        let (x, y) = input.mouse_motion();
        let difference: Vec2 = Vec2::new(x as f32, y as f32) / 1000.;

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

        // Rotation
        let (mut pitch, mut yaw) = if input.pointer_button(controls.rotate_button) {
            let scale = 4.0;

            (-difference[1] * scale, -difference[0] * scale)
        } else {
            (0.0, 0.0)
        };

        let mut pan = Vec2::ZERO;

        // Panning
        if input.key(Key::LeftShift) || input.key(Key::RightShift) || input.key(Key::Shift) {
            let scale = 0.005;
            pitch = -input.scroll().1 as f32 * scale;
            yaw = -input.scroll().0 as f32 * scale;
        } else {
            let scale = 0.0125;
            pan.x -= -input.scroll().0 as f32 * scale;
            pan.y -= -input.scroll().1 as f32 * scale;
        };

        if let Some(panning_mouse_button) = controls.panning_mouse_button {
            if input.pointer_button(panning_mouse_button) {
                let scale = controls.panning_scale * 3.0;
                pan += difference * scale;
            }
        }

        let left = transform.left();
        let up = transform.up();
        let offset = left * pan.x + up * pan.y;

        match &mut controls.mode {
            CameraControlsMode::Orbit { target } => {
                *target += offset;
                transform.position += offset;
            }
            _ => {
                transform.position += offset;
            }
        };

        if input.key(Key::Space) {
            println!("TRANSFORM: {:#?}", transform);
        }

        match &mut controls.mode {
            CameraControlsMode::Fly => {
                let pointer_position = input.pointer_position();
                let zoom_direction = camera.view_to_ray(
                    transform,
                    pointer_position.0 as f32,
                    pointer_position.1 as f32,
                );
                transform.position += -zoom_direction.direction * input.pinch() as f32 * 5.;

                let rotation_pitch = Quat::from_yaw_pitch_roll(0., pitch, 0.);
                let rotation_yaw = Quat::from_yaw_pitch_roll(yaw, 0., 0.);

                transform.rotation = rotation_yaw * transform.rotation * rotation_pitch;
                transform.position += controls.velocity * time.delta_seconds_f64 as f32;
            }
            CameraControlsMode::Orbit { target } => {
                transform.position += transform.forward() * input.pinch() as f32 * 5.;

                let rotation_pitch = Quat::from_yaw_pitch_roll(0., pitch, 0.);
                let rotation_yaw = Quat::from_yaw_pitch_roll(yaw, 0., 0.);

                let diff = transform.position - *target;
                let diff_length = diff.length();

                let rotation = rotation_yaw * transform.rotation * rotation_pitch;

                let new_direction = rotation * -Vec3::Z;
                let new_up = rotation * Vec3::Y;

                *target += controls.velocity * time.delta_seconds_f64 as f32;

                transform.position = *target - new_direction * diff_length;
                transform.rotation = Quat::from_forward_up(new_direction, new_up);
            }
        }
    }
}
