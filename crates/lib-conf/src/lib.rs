#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use lib_conf_derive::LibConfig;