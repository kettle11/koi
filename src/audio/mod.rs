use crate::*;

mod sound;
pub use sound::*;

mod listener;
pub use listener::*;

mod audio_source;
pub use audio_source::*;

pub(crate) const SAMPLE_RATE: u32 = 44100;

pub fn audio_plugin() -> Plugin {
    Plugin {
        setup_systems: vec![setup_audio.system()],
        end_of_frame_systems: vec![move_sources.system()],
        ..Default::default()
    }
}

pub fn setup_audio(world: &mut World) {
    let placeholder_sound = Sound::new_from_slice(&[0.0]);
    let sound_assets = Assets::new(placeholder_sound);

    const QUIET_AMPLITUDE: f32 = 0.001;

    let spatial_scene = oddio::SpatialScene::new(SAMPLE_RATE, 0.1);
    let mixer = oddio::Adapt::new(
        spatial_scene,
        QUIET_AMPLITUDE / 2.0f32.sqrt(),
        oddio::AdaptOptions {
            tau: 0.1,
            max_gain: 1.0,
            low: 0.1 / 2.0f32.sqrt(),
            high: 0.5 / 2.0f32.sqrt(),
        },
    );

    let (scene_handle, scene) = oddio::split(mixer);

    let mut audio_thread = AudioThread { scene };

    kaudio::begin_audio_thread(move |samples, _info| {
        audio_thread.provide_samples(samples);
    });
    world.spawn(sound_assets);
    world.spawn(AudioManager { scene_handle });
}

struct AudioThread {
    scene: oddio::SplitSignal<oddio::Adapt<oddio::SpatialScene>>,
}

impl AudioThread {
    fn provide_samples(&mut self, samples: &mut [f32]) {
        let frames = oddio::frame_stereo(samples);
        oddio::run(&self.scene, 44100, frames);
    }
}

#[derive(NotCloneComponent)]
pub struct AudioManager {
    scene_handle: oddio::Handle<oddio::Adapt<oddio::SpatialScene>>,
}