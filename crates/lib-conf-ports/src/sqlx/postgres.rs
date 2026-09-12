/// Re-export of [`log::LevelFilter`](https://docs.rs/log/latest/log/enum.LevelFilter.html).
///
pub use log::LevelFilter;

use lib_conf::{
    adapter::duration::SecondsAdapter,
    LibConfig,
};
use secrecy::{ExposeSecret, SecretString};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

/// `lib-conf` port of [`sqlx::postgres::PgPoolOptions`].
///
/// ---
///
/// Configuration at compile-time is done via [`PgPoolConfigBuilder`].
///
/// Configuration at run-time is done via [`PgPoolOverrideConfig`].
///
/// # Example
///
/// ```rust
/// use serde::Deserialize;
/// use lib_conf_ports::sqlx::postgres::{PgPoolConfig, PgPoolOverrideConfig};
///
/// #[derive(Debug, Clone, Default, Deserialize)]
/// struct MyAppConfig {
///     #[serde(default)]
///     db: PgPoolOverrideConfig,
/// }
/// # let my_app_config = MyAppConfig::default();
///
/// // ..
///
/// let pg_conf = PgPoolConfig::builder()
///    .migrate(false) // prevent migrating unless explicitly enabled at runtime
///    .max_connections(15)
///    .with_override(my_app_config.db.clone())
///    .build();
///
/// if let Some(db_url) = pg_conf.url_exposed().as_ref() {
///     let migrate = pg_conf.migrate();
///     let pool_options = pg_conf.into_pool_options();
///
/// #   #[allow(unused)]
///     if let Ok(pool) = pool_options.connect_lazy(db_url) {
///         if migrate {
///             println!("Running migrations");
///             // would probably run this next...
///             // sqlx::migrate!().run(&pool).await.expect("Migrations run");
///         } else {
///             println!("Skipping migrations");
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone, LibConfig)]
pub struct PgPoolConfig {
    /// URL of the database
    ///
    /// This is a supplementary field added by `lib-conf-ports` for convenience.
    ///
    /// **Note**: this field can only be set at run-time.
    #[config(builder_skip)]
    pub(crate) url: Option<SecretString>,

    /// Indicates to the application whether or not migrations should run.
    ///
    /// This is a supplementary field added by `lib-conf-ports` for convenience.
    ///
    /// Out of the box, this setting has no impact. It is intended to be used as
    /// runtime-adjustable control-flow for migration handling.
    ///
    /// See the [`PgPoolConfig`] struct-level docs for example usage.
    #[config(copy, default = true)]
    pub(crate) migrate: bool,

    /// Maximum number of connections that this pool should maintain
    ///
    /// Be mindful of the connection limits for your database as well as other applications
    /// which may want to connect to the same database (or even multiple instances of the same
    /// application in high-availability deployments).
    #[config(copy, default = 10)]
    pub(crate) max_connections: u32,

    /// Minimum number of connections to maintain at all times.
    ///
    /// When the pool is built, this many connections will be automatically spun up.
    ///
    /// If any connection is reaped by [`max_lifetime`](Self::max_lifetime)
    /// or [`idle_timeout`](Self::idle_timeout) or explicitly closed,
    /// and it brings the connection count below this amount, a new connection will be opened to
    /// replace it.
    ///
    /// This is only done on a best-effort basis, however. The routine that maintains this value
    /// has a deadline so it doesn't wait forever if the database is being slow or returning errors.
    ///
    /// This value is clamped internally to not
    /// exceed [`max_connections`](Self::max_connections).
    ///
    /// We've chosen not to assert `min_connections <= max_connections` anywhere
    /// because it shouldn't break anything internally if the condition doesn't hold,
    /// and if the application allows either value to be dynamically set
    /// then it should be checking this condition itself and returning
    /// a nicer error than a panic anyway.
    #[config(copy, default = 0)]
    pub(crate) min_connections: u32,

    /// Enable logging of time taken to acquire a connection from the connection pool via
    /// [`sqlx::Pool::acquire()`].
    ///
    /// If slow acquire logging is also enabled, this level is used for acquires that are not
    /// considered slow.
    #[config(copy, default = LevelFilter::Off)]
    pub(crate) acquire_time_level: LevelFilter,

    /// Level used for logging excessive time taken to acquire a connection
    /// for faster connection acquires via [`sqlx::Pool::acquire()`].
    #[config(copy, default = LevelFilter::Warn)]
    pub(crate) acquire_slow_level: LevelFilter,

    /// Threshold for reporting excessive time taken to acquire a connection from
    /// the connection pool via [`sqlx::Pool::acquire()`]. When the threshold is exceeded, a
    /// warning is logged.
    ///
    /// **Note**: [`PgPoolOverrideConfig`] deserializes this as a `u64` (in seconds).
    ///
    /// Defaults to a value that should not typically be exceeded by the pool enlarging
    /// itself with an additional new connection.
    #[config(
        copy, default = Duration::from_secs(2),
        override_from = u64, override_via = SecondsAdapter,
    )]
    pub(crate) acquire_slow_threshold: Duration,

    /// Maximum amount of time to spend waiting for a connection in [`sqlx::Pool::acquire()`].
    ///
    /// **Note**: [`PgPoolOverrideConfig`] deserializes this as a `u64` (in seconds).
    ///
    /// Caps the total amount of time `Pool::acquire()` can spend waiting across multiple phases:
    ///
    /// * First, it may need to wait for a permit from the semaphore, which grants it the privilege
    ///   of opening a connection or popping one from the idle queue.
    /// * If an existing idle connection is acquired, by default it will be checked for liveness
    ///   and integrity before being returned, which may require executing a command on the
    ///   connection. This can be disabled with [`test_before_acquire(false)`][PgPoolOptions::test_before_acquire].
    ///     * If [`before_acquire`][PgPoolOptions::before_acquire] is set, that will also
    ///       be executed.
    /// * If a new connection needs to be opened, that will obviously require I/O, handshaking,
    ///   and initialization commands.
    ///     * If [`after_connect`][PgPoolOptions::after_connect] is set, that will also be executed.
    #[config(
        copy, default = Duration::from_secs(30),
        override_from = u64, override_via = SecondsAdapter,
    )]
    pub(crate) acquire_timeout: Duration,

    /// Maximum lifetime of individual connections.
    ///
    /// **Note**: [`PgPoolOverrideConfig`] deserializes this as a `u64` (in seconds).
    ///
    /// Any connection with a lifetime greater than this will be closed.
    ///
    /// When set to `None`, all connections live until either reaped by [`idle_timeout`](Self::idle_timeout)
    /// or explicitly disconnected.
    ///
    /// Infinite connections are not recommended due to the unfortunate reality of memory/resource
    /// leaks on the database-side. It is better to retire connections periodically
    /// (even if only once daily) to allow the database the opportunity to clean up data structures
    /// (parse trees, query metadata caches, thread-local storage, etc.) that are associated with a
    /// session.
    #[config(
        copy,
        override_from = u64, override_via = SecondsAdapter,
    )]
    pub(crate) max_lifetime: Option<Duration>,

    /// Maximum idle duration for individual connections.
    ///
    /// **Note**: [`PgPoolOverrideConfig`] deserializes this as a `u64` (in seconds).
    ///
    /// Any connection that remains in the idle queue longer than this will be closed.
    ///
    /// For usage-based database server billing, this can be a cost saver.
    #[config(
        copy,
        override_from = u64, override_via = SecondsAdapter,
    )]
    pub(crate) idle_timeout: Option<Duration>,
}
impl PgPoolConfig {
    /// Returns the [`url`](Self::url) as a `String`, if one is set.
    ///
    /// This is identical to calling [`secrecy::ExposeSecret::expose_secret`], and is provided
    /// to help reduce imports & avoid polluting your Cargo.toml with one-off dependencies.
    #[must_use]
    pub fn url_exposed(&self) -> Option<String> {
        self.url.as_ref().map(|url| url.expose_secret().to_string())
    }

    #[must_use]
    /// Converts the `PgPoolConfig` ported type into `sqlx`'s native [`PgPoolOptions`].
    pub fn as_pool_options(&self) -> PgPoolOptions {
        self.clone().into()
    }
    #[must_use]
    /// Converts the `PgPoolConfig` ported type into `sqlx`'s native [`PgPoolOptions`].
    pub fn into_pool_options(self) -> PgPoolOptions {
        self.into()
    }
}
impl From<PgPoolConfig> for PgPoolOptions {
    fn from(conf: PgPoolConfig) -> Self {
        Self::new()
            .max_connections(conf.max_connections)
            .min_connections(conf.min_connections)
            .acquire_time_level(conf.acquire_time_level)
            .acquire_slow_level(conf.acquire_slow_level)
            .acquire_slow_threshold(conf.acquire_slow_threshold)
            .acquire_timeout(conf.acquire_timeout)
            .max_lifetime(conf.max_lifetime)
            .idle_timeout(conf.idle_timeout)
    }
}
impl PgPoolOverrideConfig {
    /// Returns the [`url`](Self::url) as a `String`, if one is set.
    ///
    /// This is identical to calling [`secrecy::ExposeSecret::expose_secret`], and is provided
    /// to help reduce imports & avoid polluting your Cargo.toml with one-off dependencies.
    #[must_use]
    pub fn url_exposed(&self) -> Option<String> {
        self.url.as_ref().map(|url| url.expose_secret().to_string())
    }
}
