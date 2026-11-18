//! This is an example library for [`lib_conf`] and does not provide actual functionality.
#![allow(unused, clippy::pedantic, clippy::allow_attributes, clippy::allow_attributes_without_reason)]

/// User-configuration types
pub mod config;

pub use config::{Example00Config, Example00OverrideConfig};
