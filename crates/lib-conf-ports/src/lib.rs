#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]

#![doc = include_str!("../README.md")]

#[cfg(feature = "sqlx-postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx-postgres")))]
#[cfg_attr(feature = "sqlx-postgres", doc = "Contains `lib-conf` ports of [`sqlx`](::sqlx) types")]
pub mod sqlx;
