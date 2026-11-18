#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]

#![doc = include_str!("../README.md")]

// !- Crate modules

/// Common types and dependenxy re-exports that may be referenced in derived code
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
mod derive;

// Re-export all from derive module (keeps feature gates clean)
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use derive::*;

/// Exposes a small set of conversion handlers for `override_via`
pub mod adapter;

// !- Sub-crate re-exports

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use lib_conf_derive::LibConfig;

/// Exposes traits from [`lib-conf-traits`](lib_conf_traits)
pub use lib_conf_traits as traits;
