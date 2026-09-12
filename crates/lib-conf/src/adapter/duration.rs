use std::time::Duration;

// !- Seconds adapter

/// Handles mapping a [`Duration`] from runtime config, provided as seconds
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

// !- Mins adapter

/// Handles mapping a [`Duration`] from runtime config, provided as minutes
pub struct MinutesAdapter(u64);

impl From<u64> for MinutesAdapter {
    fn from(val: u64) -> Self {
        Self(val)
    }
}
impl From<MinutesAdapter> for Duration {
    fn from(adapter: MinutesAdapter) -> Self {
        Self::from_mins(adapter.0)
    }
}

// !- Hours adapter

/// Handles mapping a [`Duration`] from runtime config, provided as hours
pub struct HoursAdapter(u64);

impl From<u64> for HoursAdapter {
    fn from(val: u64) -> Self {
        Self(val)
    }
}
impl From<HoursAdapter> for Duration {
    fn from(adapter: HoursAdapter) -> Self {
        Self::from_hours(adapter.0)
    }
}
