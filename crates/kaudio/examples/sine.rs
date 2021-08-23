use std::usize;

use kaudio::*;

fn main() {
    let mut t = 0.0;

    begin_audio_thread(move |samples, stream_info| {
        // Play a middle C
        let step_size = (std::f64::consts::PI * 2.0 * 261.63) / stream_info.sample_rate() as f64;

        /* */
        for i in (0..samples.len()).step_by(stream_info.channels() as usize) {
            let v = f64::sin(t);

            for j in 0..stream_info.channels() as usize {
                //stream_info.channels() as usize {
                samples[i + j] = v as f32 * 0.2;
            }

            t += step_size;

            if t > std::f64::consts::PI * 2.0 {
                t -= std::f64::consts::PI * 2.0
            }
        }
    });
    #[cfg(not(target_arch = "wasm32"))]
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
