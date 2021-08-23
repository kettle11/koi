pub struct Sound {
    /// Sounds are stored internally as an i16 at 44100 HZ
    pub data: Vec<f32>,
    /// Channels are interleaved if greater than 1
    pub channels: u8,
}

impl Sound {
    pub fn new(data: Vec<f32>, channels: u8) -> Self {
        Self { data, channels }
    }
}

/// Resample interleaved audio.
pub fn resample(data: &Vec<f32>, channels: usize, old_rate: f32, new_rate: f32) -> Vec<f32> {
    let step = old_rate / new_rate;

    let mut samples = Vec::with_capacity(new_rate as usize * channels);
    let last_sample = data.len() - 1;
    for i in 0..data.len() / channels {
        for j in 0..channels {
            // Find the offset for this single channel
            let start_position = i as f32 * step;

            let start_index = (start_position as usize * channels + j).min(last_sample);
            let end_index = (start_index + channels).min(last_sample);

            let start = data[start_index];
            let end = data[end_index];

            let value = (end - start) * start_position.fract() + start;
            samples.push(value);
        }
    }
    samples
}
