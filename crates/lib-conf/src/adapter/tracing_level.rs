use std::str::FromStr;

// !- Tracing level adapter

/// Handles mapping a [`tracing::Level`] from runtime config, provided as a string
///
/// **Warning:** panics if the provided level is invalid.
/// This mimics standard deserialize behaviour.
pub struct TracingLevelAdapter(String);

impl From<String> for TracingLevelAdapter {
    fn from(val: String) -> Self {
        Self(val)
    }
}
impl From<TracingLevelAdapter> for tracing::Level {
    fn from(adapter: TracingLevelAdapter) -> Self {
        match tracing::Level::from_str(&adapter.0) {
            Ok(level) => level,
            Err(error) => {
                panic!("Failed to parse tracing level: {error}");
            }
        }
    }
}
