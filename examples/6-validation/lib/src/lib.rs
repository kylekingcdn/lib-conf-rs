//! This is an example library for [`lib_conf`] and does not provide actual functionality.
#![allow(unused, clippy::pedantic)]

/// User-configuration types
pub mod config;

pub use config::{Example6Config, Example6OverrideConfig};

use serde::{Deserialize, Serialize};

/// Backend runtime executor
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeBackend {
    Tokio,
    Futures,
}
impl RuntimeBackend {
    pub fn supports_threading_strategy(self, strategy: RuntimeThreading) -> bool {
        match self {
            Self::Tokio => true,
            Self::Futures => strategy == RuntimeThreading::Current,
        }
    }
}

/// Backend threading strategy
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeThreading {
    Current,
    Multi,
}
