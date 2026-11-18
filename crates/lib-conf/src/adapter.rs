/// Provides adapters for deserializing [`Duration`](std::time::Duration)
pub mod duration;

/// Provides adapters for [`tracing`], e.g. for deserializing `tracing::Level`
#[cfg(feature = "adapter-tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "adapter-tracing")))]
pub mod tracing;
