#[allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]
mod apple;
mod application_mac;
mod events_mac;
mod keys_mac;
mod window_mac;

pub mod prelude {
    pub use super::application_mac::*;
    pub use kapp_platform_common::*;
}
