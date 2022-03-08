//! This file is a testbed for interpolation code.
//! It shows how a simple generic interpolator system could be implemented and used.
//!
use koi::*;

#[derive(Component, Clone)]
struct Interpolator<T: InterpolateTrait + ComponentTrait + Clone> {
    from: T,
    to: T,
    value: f32,
    rate: f32,
    function: fn(f32) -> f32,
}

fn move_interpolators<T: ComponentTrait + InterpolateTrait + Clone>(
    mut interpolators: Query<(&mut T, &mut Interpolator<T>)>,
    time: &Time,
) {
    for (transform, interpolator) in &mut interpolators {
        *transform = interpolator.from.interpolate(
            &interpolator.to,
            (interpolator.function)(interpolator.value),
        );
        interpolator.value += interpolator.rate * time.delta_seconds_f64 as f32;
        interpolator.value = interpolator.value.clamp(0.0, 1.0);
    }
}

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        // Setup things here.

        // Spawn a camera and make it look towards the origin.
        world.spawn((
            Transform::new()
                .with_position(Vec3::new(0.0, 4.0, 3.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            Camera::new(),
            CameraControls::new(),
        ));

        // Spawn a cube
        let parent = world.spawn((
            Transform::new(),
            Mesh::CUBE,
            Material::UNLIT,
            Interpolator {
                from: Transform::new(),
                to: Transform::new()
                    // .with_position(Vec3::X)
                    .with_rotation(Quat::from_angle_axis(std::f32::consts::TAU, Vec3::Y)),
                value: 0.0,
                rate: 1.0,
                function: smooth_step,
            },
        ));

        // Spawn a child to make the rotation easier to notice.
        let child = world.spawn((
            Transform::new()
                .with_position(Vec3::X)
                .with_scale(Vec3::fill(0.2)),
            Mesh::CUBE,
            Material::UNLIT,
        ));

        set_parent(world, Some(parent), child);

        move |event: Event, world: &mut World| {
            match event {
                Event::FixedUpdate => {
                    // Run the interpolator system.
                    move_interpolators::<Transform>.run(world);
                }
                _ => {}
            }

            false
        }
    });
}
