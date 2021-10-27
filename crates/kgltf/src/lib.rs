//! A minimal crate for loading glTF.
//!
//! This crate is auto-generated from the specification's Json Schema,
//! so some comments may not exactly match the Rust names.

mod glb;
#[allow(clippy::redundant_field_names, clippy::redundant_clone)]
mod gltf_json;

pub use glb::*;
pub use gltf_json::*;

pub use kserde::{FromJson, ToJson};
