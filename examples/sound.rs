use koi::*;

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        let sounds = world.get_single_component_mut::<Assets<Sound>>().unwrap();

        // Create a sound that plays a 440 hz sine wave.
        let sin_wav_iter =
            (0..44100).map(|i| ((i as f32 / 44100.0) * std::f32::consts::TAU * 440.0).sin() * 0.6);
        let sound = Sound::new_from_iter(sin_wav_iter);
        // let sound = Sound::load_immediate_bytes(include_bytes!("assets/bell.wav"), Some("wav"), 1.0).unwrap();
        let sound = sounds.add(sound);

        // Setup things here.
        world.spawn((
            Transform::new().with_position(Vec3::Z * 3.),
            Camera::new(),
            CameraControls::new(),
            Listener::new(),
        ));

        // Spawn a sphere playing a sound
        let mut audio_source = AudioSource::new();
        audio_source.play(&sound, true);
        world.spawn((
            Transform::new(),
            Mesh::SPHERE,
            Material::UNLIT,
            audio_source,
        ));

        // Run the World with this mutable closure.
        |_: Event, _: &mut World| {
            {}
            false
        }
    });
}
