pub use crate::*;
use std::cell::RefCell;
use kwasm::*;

type AudioOutputFormat = f32;

struct ThreadLocalData {
    callback: Box<dyn FnMut(&mut [AudioOutputFormat], StreamInfo) + Send + 'static>,
    // JavaScript will read directly from these.
    audio_scratch_buffers: Vec<Vec<f32>>,
    interleaved_buffer: Vec<f32>,
}

impl ThreadLocalData {
    /// Reserve enough space for an audio frame.
    pub fn resize(&mut self, channels: u32, frame_size: u32) {
        self.audio_scratch_buffers
            .resize_with(channels as usize, || {
                Vec::with_capacity(frame_size as usize)
            });
        for v in self.audio_scratch_buffers.iter_mut() {
            v.resize(frame_size as usize, 0.);
        }

        self.interleaved_buffer
            .resize((frame_size * channels) as usize, 0.);
    }
}

// This callback is set and used on the audio worklet thread.
thread_local! {
    static THREAD_AUDIO_CALLBACK: RefCell<Option<ThreadLocalData>> = RefCell::new(None);
}

pub fn begin_audio_thread(
    audio_callback: impl FnMut(&mut [AudioOutputFormat], StreamInfo) + Send + 'static,
) {
    #[cfg(target_feature = "atomics")]
    {
        let worklet_js_code = JSObjectFromString::new(include_str!("web_worklet.js").into());

        let (entry_point, stack_pointer, thread_local_storage_memory) = unsafe {
            kwasm::web_worker::create_worker_data(move || {
                THREAD_AUDIO_CALLBACK.with(|f| {
                    *f.borrow_mut() = Some(ThreadLocalData {
                        callback: Box::new(audio_callback),
                        audio_scratch_buffers: Vec::new(),
                        interleaved_buffer: Vec::new(),
                    });
                })
            })
        };

        // Construct a Box with a Box with a function pointer inside
        // The function pointer is a fat pointer, which JS can't handle.
        // So the external Box makes a thin pointer that's passed to JS
        // The pointer is passed to JS and then passed into the other thread when it initializes.
        worklet_js_code.call_raw(
            &JSObject::NULL,
            &[entry_point, stack_pointer, thread_local_storage_memory],
        );
    }

    #[cfg(not(target_feature = "atomics"))] {
        let _ = audio_callback;
    }

}

/*
#[no_mangle]
pub extern "C" fn kaudio_thread_initialize(callback: u32) {
    THREAD_AUDIO_CALLBACK.with(|f| unsafe {
        let callback_box_ptr = callback as *mut std::ffi::c_void as *mut _;
        let b: Box<Box<dyn FnMut(&mut [AudioOutputFormat], StreamInfo) + Send + 'static>> =
            Box::from_raw(callback_box_ptr);
        *f.borrow_mut() = Some(ThreadLocalData {
            callback: *b,
            audio_scratch_buffers: Vec::new(),
            interleaved_buffer: Vec::new(),
        });
    });
}
*/

// Returns a pointer to the memory location.
#[no_mangle]
pub extern "C" fn kaudio_audio_buffer_location(channel: u32) -> u32 {
    THREAD_AUDIO_CALLBACK.with(|f| {
        let thread_local_data = f.borrow();
        let thread_local_data = thread_local_data.as_ref().unwrap();
        thread_local_data.audio_scratch_buffers[channel as usize].as_ptr() as u32
    })
}

#[no_mangle]
pub extern "C" fn kaudio_run_callback(channels: u32, frame_size: u32, sample_rate: u32) {
    THREAD_AUDIO_CALLBACK.with(|f| {
        let mut thread_local_data = f.borrow_mut();

        let thread_local_data = thread_local_data.as_mut().unwrap();

        thread_local_data.resize(channels, frame_size);

        let stream_info = StreamInfo {
            channels,
            sample_rate,
        };

        (thread_local_data.callback)(&mut thread_local_data.interleaved_buffer, stream_info);

        // There has got to be better code (using iterators or something) for deinterleaving an audio buffer.
        // It's kinda lame that this needs to be done, but other backends require interleaved data.
        // Unfortunately the web browser will just interleave this again later.

        let mut index_in_frame = 0;
        let mut index_in_interleaved_buffer = 0;
        let channels = channels as usize;
        for _ in 0..frame_size {
            for i in 0..channels {
                thread_local_data.audio_scratch_buffers[i][index_in_frame] =
                    thread_local_data.interleaved_buffer[index_in_interleaved_buffer];
                index_in_interleaved_buffer += 1;
            }
            index_in_frame += 1;
        }

        //  let thing = Box::new(true);

        /*
        unsafe {
            let thread_local_data = CALLBACK.as_mut().unwrap();
            let a = &mut thread_local_data.audio_scratch_buffers;
            // for b in thread_local_data.audio_scratch_buffers.iter() {}
            // for b in thread_local_data.audio_scratch_buffers.iter_mut() {}
            //    panic!("HI THERE");
            //  b.iter_mut().for_each(|i| *i = 0.);
            //  }
        }
        */
    })
}
