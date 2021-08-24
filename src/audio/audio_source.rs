use super::*;

#[derive(Component)]
pub struct AudioSource {
    // Remove this Mutex if the inner type becomes `Sync`.
    pub(super) to_play: Vec<(Handle<Sound>, bool)>,
    pub(super) playing: Vec<Box<dyn SpatialHandle>>,
    pub(super) last_position: Option<Vec3>,
    pub(super) volume: f32,
    pub teleported: bool,
}

// For now `AudioSource`'s clone with nothing playing.
// Should it be this way?
impl Clone for AudioSource {
    fn clone(&self) -> Self {
        AudioSource {
            to_play: Vec::new(),
            playing: Vec::new(),
            last_position: self.last_position,
            volume: self.volume,
            teleported: self.teleported,
        }
    }
}

impl AudioSource {
    pub fn new() -> Self {
        Self {
            to_play: Vec::new(),
            playing: Vec::new(),
            last_position: None,
            volume: 1.0,
            teleported: false,
        }
    }
    pub fn play(&mut self, sound: &Handle<Sound>, looped: bool) {
        // Sounds are played at the end of fixed update.
        // This shouldn't cause any major lag because frame calculations should generally be fast.
        // And if ultraminimal latency requirements are necessary then special scheduling to run
        // earlier in the frame is needed anyways.
        self.to_play.push((sound.clone(), looped));
    }

    pub fn set_position_and_velocity(
        &mut self,
        position: Vec3,
        velocity: Vec3,
        discontinuity: bool,
    ) {
        // Since AudioSource keeps its own position and velocity care must be taken to ensure
        // it's always up to date.

        let mut i = 0;
        while i < self.playing.len() {
            if self.playing[i].is_stopped() {
                self.playing.swap_remove(i);
            } else {
                self.playing[i].set_motion(position, velocity, discontinuity);
                i += 1;
            }
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        for sound in &mut self.playing {
            sound.set_volume(volume)
        }
    }
}

pub fn move_sources(
    mut listener: Query<(&mut Listener, Option<&Transform>)>,
    mut sources: Query<(&mut AudioSource, &Transform)>,
    sounds: &Assets<Sound>,
    audio: &mut AudioManager,
) {
    // For now only one `Listener` is supported.
    if let Some((listener, listener_transform)) = listener.iter_mut().next() {
        let listener_transform = listener_transform.cloned().unwrap_or(Transform::new());

        let q: [f32; 4] = listener_transform.rotation.into();
        let mut spatial_scene_control = audio.scene_handle.control::<oddio::SpatialScene, _>();
        spatial_scene_control.set_listener_rotation(q.into());

        // Calculate relative postion and velocity.
        let last_position = listener
            .last_position
            .unwrap_or(listener_transform.position);
        listener.last_position = Some(listener_transform.position);
        let listener_velocity = (listener_transform.position - last_position) * 60.;

        for (source, source_transform) in &mut sources {
            let last_position = source.last_position.unwrap_or(source_transform.position);
            let velocity = if source.teleported {
                // If the source begins moving immediately this will be slightly incorrect.
                Vec3::ZERO
            } else {
                (source_transform.position - last_position) * 60.
            };

            source.last_position = Some(source_transform.position);

            let relative_position = source_transform.position - listener_transform.position;
            let relative_velocity = -(listener_velocity - velocity);

            source.set_position_and_velocity(
                relative_position,
                relative_velocity,
                source.teleported,
            );
            source.teleported = false;

            let velocity: [f32; 3] = relative_velocity.into();
            let position: [f32; 3] = relative_velocity.into();

            let position = position.into();
            let velocity = velocity.into();

            // Play sounds that are queued up to be played.
            let mut i = 0;
            while i < source.to_play.len() {
                let (sound_handle, looped) = &source.to_play[i];

                // Skip playing placeholder sounds until they're ready.
                // This has the effect of delaying a sound.
                // A fancier scheme could skip the placeholder sound ahead by the appropriate amount.
                if sounds.is_placeholder(&sound_handle) {
                    i += 1;
                    continue;
                }

                let sound = sounds.get(&sound_handle);
                let spatial_options = oddio::SpatialOptions {
                    position,
                    velocity,
                    radius: 1.0,
                    max_distance: 1000.,
                };
                let mut frames_source = if *looped {
                    let source = oddio::Gain::new(oddio::Cycle::new(sound.frames.clone()), 1.0);
                    Box::new(spatial_scene_control.play(source, spatial_options))
                        as Box<dyn SpatialHandle>
                } else {
                    let source =
                        oddio::Gain::new(oddio::FramesSignal::new(sound.frames.clone(), 0.), 1.0);
                    Box::new(spatial_scene_control.play(source, spatial_options))
                        as Box<dyn SpatialHandle>
                };
                frames_source.set_volume(source.volume);
                source.playing.push(frames_source);
                source.to_play.swap_remove(i);
            }
        }
    }
}

pub(super) trait SpatialHandle: Send + Sync {
    fn set_motion(&mut self, position: Vec3, velocity: Vec3, discontinuity: bool);
    fn set_volume(&mut self, volume: f32);
    fn is_stopped(&mut self) -> bool;
}

impl<T> SpatialHandle for oddio::Handle<oddio::Spatial<oddio::Stop<oddio::Gain<T>>>> {
    fn set_motion(&mut self, position: Vec3, velocity: Vec3, discontinuity: bool) {
        let position: [f32; 3] = position.into();
        let velocity: [f32; 3] = velocity.into();

        let position = position.into();
        let velocity = velocity.into();
        self.control::<oddio::Spatial<_>, _>()
            .set_motion(position, velocity, discontinuity);
    }

    fn set_volume(&mut self, volume: f32) {
        self.control::<oddio::Gain<_>, _>().set_gain(volume);
    }

    fn is_stopped(&mut self) -> bool {
        self.control::<oddio::Stop<_>, _>().is_stopped()
    }
}
