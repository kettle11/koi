use kaudio::*;

fn main() {
    // This example assumes a wav file with a single channel.
    let bytes = include_bytes!("bell.wav");
    let mut sound = load_wav_from_bytes(bytes).unwrap();

    if sound.channels > 1 {
        let channels = sound.channels as f32;
        // Reduce the sound to mono by taking the average of the channels.
        sound.data = sound
            .data
            .chunks(sound.channels as usize)
            .map(|d| d.iter().sum::<f32>() / channels)
            .collect();
    }

    //  let sound = load_wav("examples/bell.wav").unwrap();

    let mut offset = 0;
    begin_audio_thread(move |samples, stream_info| {
        if stream_info.sample_rate() != 44100 {
            panic!();
        }

        let channels = stream_info.channels() as usize;

        for i in (0..samples.len()).step_by(channels) {
            for j in 0..channels {
                samples[i + j] = sound.data[offset]
            }
            offset += 1;

            if offset >= sound.data.len() {
                offset = 0;
            }
        }
    });
    #[cfg(not(target_arch = "wasm32"))]
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
