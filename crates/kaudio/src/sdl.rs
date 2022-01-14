use crate::*;
type AudioOutputFormat = f32;

pub fn begin_audio_thread(
    audio_callback: impl FnMut(&mut [AudioOutputFormat], StreamInfo) + Send + 'static,
) {
    let mut audio_spec_in = fermium::audio::SDL_AudioSpec {
        freq: SAMPLE_RATE as _,
        format: fermium::audio::AUDIO_F32SYS,
        channels: 2,
        silence: 0,
        size: 0,
        padding: 0,
        callback: Some(sdl_audio_callback),
        samples: 2048, // Number of frames in streaming buffer. Introduces 46 ms of latency.
        userdata: Box::leak(Box::new(CallbackData {
            callback: Box::new(audio_callback),
        })) as *mut _ as *mut _,
    };

    // These values will be overriden.
    let mut audio_spec_out = fermium::audio::SDL_AudioSpec {
        freq: SAMPLE_RATE as _,
        format: fermium::audio::AUDIO_F32SYS,
        channels: 2,
        silence: 0,
        size: 0,
        padding: 0,
        callback: None,
        samples: 2048, // Number of frames in streaming buffer. Introduces 46 ms of latency.
        userdata: std::ptr::null_mut(),
    };

    unsafe {
        fermium::audio::SDL_OpenAudio(&mut audio_spec_in, &mut audio_spec_out);
    }

    // Check that the audio backend matches kaudio's required settings.
    if audio_spec_out.channels != 2
        || !(audio_spec_out.format == fermium::audio::AUDIO_F32LSB
            || audio_spec_out.format == fermium::audio::AUDIO_F32MSB
            || audio_spec_out.format == fermium::audio::AUDIO_F32)
        || audio_spec_out.freq != SAMPLE_RATE as _
    {
        println!("kaudio error: Audio backend could not be initialized with requested settings. Audio will be disabled. Debug info:");
        println!("CHANNELS: {:?}", audio_spec_out.channels);
        println!("SAMPLES: {:?}", audio_spec_out.samples);
        println!("FORMAT: {:?}", audio_spec_out.format);
        println!("SAMPLE RATE: {:?}", audio_spec_out.freq);
    } else {
        // Actually play the audio
        unsafe { fermium::audio::SDL_PauseAudio(0) }
    }
}

struct CallbackData {
    callback: Box<dyn FnMut(&mut [AudioOutputFormat], StreamInfo) + Send + 'static>,
}

unsafe extern "C" fn sdl_audio_callback(
    userdata: *mut std::ffi::c_void,
    stream: *mut u8,
    len: i32,
) {
    let callback_data = userdata as *mut CallbackData;
    let stream: &mut [f32] = std::slice::from_raw_parts_mut(
        stream as *mut f32 as _,
        len as usize / std::mem::size_of::<f32>(),
    );

    // Always zero out data just in case.
    // But does SDL already do this?
    for v in stream.iter_mut() {
        *v = 0.0;
    }
    ((*callback_data).callback)(
        stream,
        StreamInfo {
            sample_rate: SAMPLE_RATE as u32,
            channels: 2,
        },
    )
}
