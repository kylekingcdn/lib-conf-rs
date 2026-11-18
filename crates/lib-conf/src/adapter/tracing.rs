use std::fmt::Display;
use std::str::FromStr;

// !- Tracing level adapter

/// Handles mapping a [`tracing::Level`] from runtime config, provided as a string
///
/// **Warning:** panics if the provided level is invalid.
pub struct Level(String);

impl<O: Display> From<O> for Level {
    fn from(override_field: O) -> Self {
        Self(override_field.to_string())
    }
}

/// **Warning:** panics if the provided level is invalid
impl From<Level> for tracing::Level {
    fn from(adapter: Level) -> Self {
        match tracing::Level::from_str(&adapter.0) {
            Ok(level) => level,
            Err(error) => {
                panic!("Failed to parse tracing level: {error}");
            }
        }
    }
}
