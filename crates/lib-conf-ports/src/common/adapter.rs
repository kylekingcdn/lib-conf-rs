use std::time::Duration;

/// Handles mapping durations provided at runtime
pub struct SecondsAdapter(u64);

impl From<u64> for SecondsAdapter {
    fn from(val: u64) -> Self {
        Self(val)
    }
}
impl From<SecondsAdapter> for Duration {
    fn from(adapter: SecondsAdapter) -> Self {
        Self::from_secs(adapter.0)
    }
}
