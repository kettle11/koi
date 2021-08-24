//! A minimalist serialization / deserialization crate.
//! **WORK IN PROGRESS**

mod deserialize_trait;
mod serialize_trait;
mod thing;

pub use deserialize_trait::*;
pub use serialize_trait::*;
pub use thing::*;

mod json {
    mod json_deserialize;
    mod json_serialize;
    pub use json_deserialize::*;
    pub use json_serialize::*;
}

pub use json::*;

#[cfg(feature = "kserde_derive")]
pub use kserde_derive::*;
