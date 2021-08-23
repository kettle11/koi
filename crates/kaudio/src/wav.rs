use crate::{resample, Sound};

pub fn load_wav_from_bytes(bytes: &[u8]) -> Result<crate::Sound, hound::Error> {
    let mut reader = hound::WavReader::new(bytes)?;

    let spec = reader.spec();
    let mut samples = match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().map(|x| x.unwrap()).collect(),
        hound::SampleFormat::Int => match spec.bits_per_sample {
            8 => reader
                .samples::<i8>()
                .map(|x| (x.unwrap() as f32 / i8::MAX as f32))
                .collect(),
            16 => reader
                .samples::<i16>()
                .map(|x| (x.unwrap() as f32 / i16::MAX as f32))
                .collect(),
            24 => reader
                .samples::<i32>()
                .map(|x| (x.unwrap() as f32 / 8388607.))
                .collect(),
            32 => reader
                .samples::<i32>()
                .map(|x| (x.unwrap() as f32 / i32::MAX as f32))
                .collect(),
            _ => unimplemented!(),
        },
    };

    // Resample audio if it doesn't match our desired sample rate.
    if spec.sample_rate != 44100 {
        samples = resample(
            &samples,
            spec.channels as usize,
            spec.sample_rate as f32,
            44100.,
        );
    }

    Ok(Sound::new(samples, spec.channels as u8))
}
pub fn load_wav(path: &str) -> Result<crate::Sound, hound::Error> {
    let file = std::fs::File::open(path)?;
    let mut reader = hound::WavReader::new(file)?;

    let spec = reader.spec();
    let mut samples = match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().map(|x| x.unwrap()).collect(),
        hound::SampleFormat::Int => match spec.bits_per_sample {
            8 => reader
                .samples::<i8>()
                .map(|x| (x.unwrap() as f32 / i8::MAX as f32))
                .collect(),
            16 => reader
                .samples::<i16>()
                .map(|x| (x.unwrap() as f32 / i16::MAX as f32))
                .collect(),
            24 => reader
                .samples::<i32>()
                .map(|x| (x.unwrap() as f32 / 8388607.))
                .collect(),
            32 => reader
                .samples::<i32>()
                .map(|x| (x.unwrap() as f32 / i32::MAX as f32))
                .collect(),
            _ => unimplemented!(),
        },
    };

    // Resample audio if it doesn't match our desired sample rate.
    if spec.sample_rate != 44100 {
        samples = resample(
            &samples,
            spec.channels as usize,
            spec.sample_rate as f32,
            44100.,
        );
    }

    Ok(Sound::new(samples, spec.channels as u8))
}
