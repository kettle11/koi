use crate::*;
use std::sync::Arc;

pub struct Sound {
    pub(super) frames: Arc<oddio::Frames<f32>>,
}

impl Sound {
    pub fn new_from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = f32>,
        I::IntoIter: ExactSizeIterator,
    {
        let frames = oddio::Frames::from_iter(SAMPLE_RATE, iter);
        Sound { frames }
    }

    pub fn new_from_slice(slice: &[f32]) -> Self {
        let frames = oddio::Frames::from_slice(SAMPLE_RATE, slice);
        Sound { frames }
    }

    pub fn load_immediate_bytes(bytes: &[u8], extension: Option<&str>, scale: f32) -> Option<Self> {
        match extension {
            Some("wav") => {
                let mut sound = kaudio::load_wav_from_bytes(bytes).unwrap();

                // Apply scale
                sound.data.iter_mut().for_each(|s| *s *= scale);

                if sound.channels > 1 {
                    let channels = sound.channels as f32;
                    // Reduce the sound to mono by taking the average of the channels.
                    sound.data = sound
                        .data
                        .chunks(sound.channels as usize)
                        .map(|d| d.iter().sum::<f32>() / channels)
                        .collect();
                }
                Some(Sound::new_from_iter(sound.data.into_iter()))
            }
            _ => panic!("Unsupported audio format"),
        }
    }
    pub fn load_immediate(path: &str, scale: f32) -> Option<Self> {
        let bytes = std::fs::read(path).ok()?;
        let extension = std::path::Path::new(&path)
            .extension()
            .and_then(std::ffi::OsStr::to_str);

        Self::load_immediate_bytes(&bytes, extension, scale)
    }
}

use std::sync::mpsc;

struct SoundLoadMessage {
    handle: Handle<Sound>,
    sound: Sound,
}
pub struct SoundAssetLoader {
    sender: SyncGuard<mpsc::Sender<SoundLoadMessage>>,
    receiver: SyncGuard<mpsc::Receiver<SoundLoadMessage>>,
}

impl LoadableAssetTrait for Sound {
    type Options = ();
    type AssetLoader = SoundAssetLoader;
}

impl SoundAssetLoader {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender: SyncGuard::new(sender),
            receiver: SyncGuard::new(receiver),
        }
    }
}

impl AssetLoader<Sound> for SoundAssetLoader {
    fn load_with_options(
        &mut self,
        path: &str,
        handle: Handle<Sound>,
        _options: <Sound as LoadableAssetTrait>::Options,
    ) {
        let path = path.to_owned();
        let sender = self.sender.inner().clone();

        ktasks::spawn(async move {
            let extension = std::path::Path::new(&path)
                .extension()
                .and_then(std::ffi::OsStr::to_str);

            // Todo: This should produce a meaningful error instead of an unwrap
            let bytes = crate::fetch_bytes(&path).await.unwrap();
            let sound = Sound::load_immediate_bytes(&bytes, extension, 1.0).unwrap();

            sender.send(SoundLoadMessage { handle, sound }).unwrap();
        })
        .run();
    }
}

pub fn load_sounds(sounds: &mut Assets<Sound>) {
    // A Vec doesn't need to be allocated here.
    // This is just a way to not borrow the AssetLoader and Assets at
    // the same time.
    let messages: Vec<SoundLoadMessage> = sounds.asset_loader.receiver.inner().try_iter().collect();
    for message in messages.into_iter() {
        println!("SOUND LOADED");
        sounds.replace_placeholder(&message.handle, message.sound);
    }
}
