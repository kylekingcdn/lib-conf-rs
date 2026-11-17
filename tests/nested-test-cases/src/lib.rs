#![allow(clippy::pedantic, clippy::allow_attributes, reason = "expansion tests")]

pub mod nested;
pub use nested::*;

pub mod parent;
pub use parent::*;

pub mod shared;
// pub(crate) use shared::*;

//pub mod examples;
