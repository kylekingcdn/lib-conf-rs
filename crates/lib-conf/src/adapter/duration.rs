use std::time::Duration;

// Alternatively, could use a single struct that handles all units using a phantom generic field (for the unit)
//
// Each variation would only require a single `From<DurationAdapter<Unit>>` impl
//
// However, opted to KISS here instead
//
// Might revise in the future, but the current impl is far simpler and a good resource to use as an example for users

// !- Seconds adapter

/// Handles mapping a [`Duration`] from runtime config, provided as seconds
pub struct Sec(u64);

impl<F: Into<u64>> From<F> for Sec {
    fn from(override_field: F) -> Self {
        Self(override_field.into())
    }
}
impl From<Sec> for Duration {
    fn from(adapter: Sec) -> Self {
        Self::from_secs(adapter.0)
    }
}

// !- Mins adapter

/// Handles mapping a [`Duration`] from runtime config, provided as minutes
pub struct Min(u64);

impl<F: Into<u64>> From<F> for Min {
    fn from(override_field: F) -> Self {
        Self(override_field.into())
    }
}
impl From<Min> for Duration {
    fn from(adapter: Min) -> Self {
        Self::from_mins(adapter.0)
    }
}

// !- Hours adapter

/// Handles mapping a [`Duration`] from runtime config, provided as hours
pub struct Hour(u64);

impl<F: Into<u64>> From<F> for Hour {
    fn from(override_field: F) -> Self {
        Self(override_field.into())
    }
}
impl From<Hour> for Duration {
    fn from(adapter: Hour) -> Self {
        Self::from_hours(adapter.0)
    }
}
