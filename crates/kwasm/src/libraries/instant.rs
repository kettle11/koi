use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

use crate::*;
thread_local! {
    static PERFORMANCE_NOW: JSObjectFromString = JSObjectFromString::new("function now() { return performance.now() }; now");
    static DATE_NOW: JSObjectFromString = JSObjectFromString::new("function now() { return Date.now() }; now");

}

pub fn now() -> f64 {
    let result = PERFORMANCE_NOW.with(|f| f.call()).unwrap();
    result.get_value_f64()
}

pub fn date_now() -> f64 {
    let result = DATE_NOW.with(|f| f.call()).unwrap();
    result.get_value_f64()
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Instant(pub Duration);

impl Ord for Instant {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("an instant should never be NaN or Inf.")
    }
}
impl Eq for Instant {}

impl Instant {
    #[inline]
    pub fn now() -> Self {
        Instant(duration_from_f64(now()))
    }

    #[inline]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        assert!(
            earlier.0 <= self.0,
            "`earlier` cannot be later than `self`."
        );
        self.0 - earlier.0
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }

    #[inline]
    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
        self.0.checked_add(duration).map(Instant)
    }

    #[inline]
    pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
        self.0.checked_sub(duration).map(Instant)
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Duration) -> Self {
        Instant(self.0 + rhs)
    }
}

impl AddAssign<Duration> for Instant {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Duration) -> Self {
        Instant(self.0 - rhs)
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Instant) -> Duration {
        self.duration_since(rhs)
    }
}

impl SubAssign<Duration> for Instant {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 -= rhs
    }
}

fn duration_from_f64(millis: f64) -> Duration {
    Duration::from_secs_f64(millis / 1000.)
}
