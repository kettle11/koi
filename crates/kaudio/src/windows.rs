extern crate winapi;
use crate::windows::winapi::Interface;
use crate::*;

pub trait AudioSource {
    fn provide_samples(&mut self, samples: &mut [f32]);
    fn handle_event() {}
}

fn check_result(result: winapi::um::winnt::HRESULT) -> Result<(), std::io::Error> {
    if result < 0 {
        println!("ERROR: {:?}", result);
        Err(std::io::Error::from_raw_os_error(result))
    } else {
        Ok(())
    }
}

type AudioOutputFormat = f32;

// A backend for Windows Audio Session Application Programming Interface (WASAPI)
// Inspired by sokol_audio.h:
// https://github.com/floooh/sokol/blob/master/sokol_audio.h
// And cpal: https://github.com/RustAudio/cpal/blob/master/src/host/wasapi/device.rs
pub fn begin_audio_thread(
    audio_callback: impl FnMut(&mut [AudioOutputFormat], StreamInfo) + Send + 'static,
) {
    let sample_rate = 44100;
    unsafe {
        let hresult = winapi::um::combaseapi::CoInitializeEx(
            std::ptr::null_mut(),
            winapi::um::objbase::COINIT_MULTITHREADED,
        );
        check_result(hresult).unwrap();

        // First find an audio device to play to
        let mut enumerator: *mut winapi::um::mmdeviceapi::IMMDeviceEnumerator =
            std::mem::MaybeUninit::uninit().assume_init();
        let hresult = winapi::um::combaseapi::CoCreateInstance(
            &winapi::um::mmdeviceapi::CLSID_MMDeviceEnumerator,
            std::ptr::null_mut(),
            winapi::um::combaseapi::CLSCTX_ALL,
            &winapi::um::mmdeviceapi::IMMDeviceEnumerator::uuidof(),
            &mut enumerator as *mut *mut winapi::um::mmdeviceapi::IMMDeviceEnumerator as *mut _,
        );
        check_result(hresult).unwrap();

        // Select the default audio device.
        let mut device = std::mem::MaybeUninit::uninit().assume_init();
        let hresult = (*enumerator).GetDefaultAudioEndpoint(
            winapi::um::mmdeviceapi::eRender,
            winapi::um::mmdeviceapi::eConsole,
            &mut device,
        );
        check_result(hresult).unwrap();

        // Create an audio client.
        let mut audio_client = std::mem::MaybeUninit::uninit().assume_init();
        let hresult = (*device).Activate(
            &winapi::um::audioclient::IID_IAudioClient,
            winapi::um::combaseapi::CLSCTX_ALL,
            std::ptr::null_mut(),
            &mut audio_client,
        );
        check_result(hresult).unwrap();

        let audio_client = audio_client as *mut winapi::um::audioclient::IAudioClient;

        // Setup the streaming format for the audio.
        let wFormatTag = winapi::shared::mmreg::WAVE_FORMAT_PCM;
        let nChannels = 2;
        let nSamplesPerSec = sample_rate;
        let wBitsPerSample = 16;

        let nBlockAlign = (nChannels as u32 * wBitsPerSample as u32) / 8; // Size of a sample. Required equation. See below link
        let nAvgBytesPerSec = nSamplesPerSec as u32 * nBlockAlign as u32; // Required equation. See below link

        let cbSize = 0; // Denotes space for extra data appended to the end.

        // https://docs.microsoft.com/en-us/windows/win32/api/mmeapi/ns-mmeapi-waveformatex
        let format = winapi::shared::mmreg::WAVEFORMATEX {
            wFormatTag,
            nChannels,
            nSamplesPerSec: nSamplesPerSec as u32,
            nAvgBytesPerSec: nAvgBytesPerSec as u32,
            nBlockAlign: nBlockAlign as u16,
            wBitsPerSample,
            cbSize,
        };
        use winapi::um::audiosessiontypes::*;

        // https://docs.microsoft.com/en-us/windows/win32/api/audioclient/nf-audioclient-iaudioclient-initialize
        let buffer_frames = 2048; // Number of frames in streaming buffer

        let duration = (buffer_frames as f64) / (sample_rate as f64 * (1.0 / 10000000.0));
        let hresult = (*audio_client).Initialize(
            AUDCLNT_SHAREMODE_SHARED,          // ShareMode
            AUDCLNT_STREAMFLAGS_EVENTCALLBACK, // StreamFlgs
            duration as i64,                   // hnsBufferDuration
            0,                                 // hnsPeriodicity
            &format,                           // *pFormat
            std::ptr::null(),                  // AudioSessionGuid
        );
        check_result(hresult).unwrap();

        let mut pNumBufferFrames = std::mem::MaybeUninit::uninit().assume_init();
        let hresult = (*audio_client).GetBufferSize(&mut pNumBufferFrames);
        check_result(hresult).unwrap();
        println!("Buffer frames: {:?}", pNumBufferFrames);

        let mut render_client: *mut winapi::um::audioclient::IAudioRenderClient =
            std::mem::MaybeUninit::uninit().assume_init();

        let hresult = (*audio_client).GetService(
            &winapi::um::audioclient::IID_IAudioRenderClient,
            &mut render_client as *mut *mut winapi::um::audioclient::IAudioRenderClient as *mut _,
        );
        check_result(hresult).unwrap();

        // Create the event
        // https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventa
        let event = winapi::um::synchapi::CreateEventA(
            std::ptr::null_mut(), // lpEventAttributes
            0,                    // bManualReset (BOOL)
            0,                    // bInitialState (BOOL)
            std::ptr::null(),     // lpName
        );

        let hresult = (*audio_client).SetEventHandle(event);
        check_result(hresult).unwrap();

        // This wrapper is a workaround because Rust designates raw pointers as unsafe to send
        // 'unsafe impl Send' is implemented for ThreadData
        let thread_data = ThreadData {
            event,
            audio_client,
            render_client,
        };

        // Run forever requesting audio
        std::thread::spawn(move || loop {
            let mut buffer_position: u32 = 0;
            loop {
                let result = winapi::um::synchapi::WaitForSingleObject(
                    thread_data.event,
                    winapi::um::winbase::INFINITE,
                );

                let mut padding = std::mem::MaybeUninit::uninit().assume_init();
                let hresult = (*thread_data.audio_client).GetCurrentPadding(&mut padding);
                check_result(hresult).unwrap();

                // println!("PADDING: {:?}", padding);
                let frames_to_write = pNumBufferFrames - padding;

                let mut buffer: *mut winapi::shared::minwindef::BYTE =
                    std::mem::MaybeUninit::uninit().assume_init();
                let hresult = (*thread_data.render_client)
                    .GetBuffer(frames_to_write, &mut buffer as *mut *mut _);
                check_result(hresult).unwrap();

                let buffer_len = frames_to_write as usize * nChannels as usize;
                let samples_slice: &mut [i16] =
                    std::slice::from_raw_parts_mut(buffer as *mut i16, buffer_len);
                samples_slice.fill(0);

                // Don't provide audio for now
                //  audio_source.provide_samples(samples_slice);
                let hresult = (*thread_data.render_client).ReleaseBuffer(frames_to_write, 0);
                check_result(hresult).unwrap();

                //println!("REQUESTING AUDIO");
            }
        });

        let hresult = (*audio_client).Start();
        check_result(hresult).unwrap();

        // These should be stored in some sort of structure and released when the structure is dropped.
        // (*enumerator).Release();
        // (*device).Release();
        // (*render_client).Release();
    }
}

struct ThreadData {
    event: *mut winapi::ctypes::c_void,
    audio_client: *mut winapi::um::audioclient::IAudioClient,
    render_client: *mut winapi::um::audioclient::IAudioRenderClient,
}

unsafe impl Send for ThreadData {}
