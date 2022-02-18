use kecs::*;

/// Time elapsed since last draw call
#[derive(NotCloneComponent)]
pub struct Time {
    pub delta_seconds_f64: f64,
    pub fixed_time_step: f64,
    pub discontinuity: bool,
}
