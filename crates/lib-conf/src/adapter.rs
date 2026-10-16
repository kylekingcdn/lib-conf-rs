/// Provides adapters for deserializing [`Duration`](std::time::Duration)
#[cfg_attr(docsrs, doc(cfg(feature = "adapter")))]
pub mod duration;

/// Provides an adapter for deserializing [`tracing::Level`]
#[cfg(feature = "adapter-tracing-level")]
#[cfg_attr(docsrs, doc(cfg(feature = "adapter-tracing-level")))]
pub mod tracing_level;
