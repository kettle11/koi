use oddio::Frame;

pub struct FixedGain<T> {
    gain: f32,
    inner: T,
}

impl<T> FixedGain<T> {
    pub fn new(signal: T, gain: f32) -> Self {
        Self {
            gain,
            inner: signal,
        }
    }
}

impl<T: oddio::Signal> oddio::Signal for FixedGain<T>
where
    T::Frame: oddio::Frame,
{
    type Frame = T::Frame;

    fn sample(&self, interval: f32, out: &mut [Self::Frame]) {
        self.inner.sample(interval, out);
        for frame in out {
            for v in frame.channels_mut() {
                *v *= self.gain
            }
        }
    }
}

impl<T> oddio::Seek for FixedGain<T>
where
    T: oddio::Signal + oddio::Seek,
    T::Frame: Frame,
{
    fn seek(&self, seconds: f32) {
        self.inner.seek(seconds);
    }
}
