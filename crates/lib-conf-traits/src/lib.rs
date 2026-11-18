#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]

#![doc = include_str!("../README.md")]

/// Traits for building `Config` structs
pub mod build;

/// Traits implemented by config `Error` enums
pub mod error;

/// Traits for merging `Override` config structs
pub mod merge;

pub use build::{BuildConfig, TryBuildConfig};
pub use error::ConfigError;
pub use merge::{MergeOverride, TryMergeOverride};
