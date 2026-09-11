#[derive(Debug, Clone)]
pub struct CrabLogConfig {
    /// Enables the Logger library
    pub(crate) enabled: bool,

    /// Enables verbose logging
    pub(crate) verbose: bool,

    /// Enables file logging at the given path
    ///
    /// If no path is provided, file logging is disabled.
    pub(crate) log_file_path: Option<String>,
}
impl CrabLogConfig {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            enabled: true,
            verbose: false,
            log_file_path: None,
        }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder() -> CrabLogConfigBuilder {
        CrabLogConfigBuilder::new()
    }
    /// Enables the Logger library
    ///
    /// ---
    ///
    /// - Library default: **`true`**
    #[must_use]
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    /// Enables verbose logging
    ///
    /// ---
    ///
    /// - Library default: **`false`**
    #[must_use]
    pub fn verbose(&self) -> bool {
        self.verbose
    }
    /// Enables file logging at the given path
    ///
    /// If no path is provided, file logging is disabled.
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub fn log_file_path(&self) -> Option<&str> {
        self.log_file_path.as_ref().map(|s| s.as_str())
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(&mut self, override_conf: CrabLogOverrideConfig) {
        let val = override_conf.enabled.clone();
        if override_conf.enabled_unset {
            self.enabled = true
        } else if let Some(val) = val {
            self.enabled = val;
        }
        let val = override_conf.verbose.clone();
        if override_conf.verbose_unset {
            self.verbose = false
        } else if let Some(val) = val {
            self.verbose = val;
        }
        let val = override_conf.log_file_path.clone();
        if override_conf.log_file_path_unset {
            self.log_file_path = None
        } else if let Some(val) = val {
            self.log_file_path = Some(val);
        }
    }
}
impl Default for CrabLogConfig {
    fn default() -> Self {
        Self::new()
    }
}
impl std::ops::Add<CrabLogOverrideConfig> for CrabLogConfig {
    type Output = Self;
    fn add(self, other: CrabLogOverrideConfig) -> Self {
        let mut merged = self;
        merged.merge_with_override(other);
        merged
    }
}

/// Provides a builder used to configure [`CrabLogConfig`] at compile-time.
///
/// Can be constructed using either [`CrabLogConfig::builder()`] or [`CrabLogConfigBuilder::new()`].
///
/// For run-time congiguration of `CrabLogConfig`, see [`CrabLogOverrideConfig`].
#[derive(Debug, Clone)]
pub struct CrabLogConfigBuilder {
    pub(crate) inner: CrabLogConfig,
    pub(crate) override_conf: Option<CrabLogOverrideConfig>,
}
impl CrabLogConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(clippy::redundant_field_names)]
    pub fn new() -> Self {
        Self {
            override_conf: None,
            inner: <CrabLogConfig>::new(),
        }
    }
    /// Enables the Logger library
    ///
    /// ---
    ///
    /// - Library default: **`true`**
    pub fn enabled(mut self, val: bool) -> Self {
        self.inner.enabled = val;
        self
    }
    /// Enables verbose logging
    ///
    /// ---
    ///
    /// - Library default: **`false`**
    pub fn verbose(mut self, val: bool) -> Self {
        self.inner.verbose = val;
        self
    }
    /// Enables file logging at the given path
    ///
    /// If no path is provided, file logging is disabled.
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    pub fn log_file_path(mut self, val: Option<String>) -> Self {
        self.inner.log_file_path = val;
        self
    }
    /// If supplied, will overwrite any values present in the provided
    /// override config.
    ///
    /// This happens as the last step within the
    ///[`build()`](Self::build) method.
    ///
    /// Any previous calls to [`with_override()`](Self::with_override)
    /// in the builder chain will have no effect on the resulting
    /// config.
    ///
    /// The order in which this is chained with the builder's setter
    /// methods does not matter.
    #[must_use]
    pub fn with_override(mut self, override_conf: CrabLogOverrideConfig) -> Self {
        self.override_conf = Some(override_conf);
        self
    }
    /// Clears the override config previously set via
    /// [`with_override()`](Self::with_override).
    #[must_use]
    pub fn clear_override(mut self) -> Self {
        self.override_conf = None;
        self
    }
    /// Builds the [`CrabLogConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(self) -> CrabLogConfig {
        if let Some(override_conf) = self.override_conf {
            self.inner + override_conf
        } else {
            self.inner
        }
    }
}
impl Default for CrabLogConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CrabLogOverrideConfig {
    /// Enables the Logger library
    ///
    /// ---
    ///
    /// - Library default: **`true`**
    pub(crate) enabled: Option<bool>,
    #[serde(
        default,
        alias = "enabled_reset",
        alias = "enabled_revert",
        alias = "enabled_clear",
        alias = "enabled_default"
    )]
    /// flag allowing for reverting a builder-configured
    /// setting at runtime
    pub(crate) enabled_unset: bool,
    /// Enables verbose logging
    ///
    /// ---
    ///
    /// - Library default: **`false`**
    pub(crate) verbose: Option<bool>,
    #[serde(
        default,
        alias = "verbose_reset",
        alias = "verbose_revert",
        alias = "verbose_clear",
        alias = "verbose_default"
    )]
    /// flag allowing for reverting a builder-configured
    /// setting at runtime
    pub(crate) verbose_unset: bool,
    /// Enables file logging at the given path
    ///
    /// If no path is provided, file logging is disabled.
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    pub(crate) log_file_path: Option<String>,
    #[serde(
        default,
        alias = "log_file_path_reset",
        alias = "log_file_path_revert",
        alias = "log_file_path_clear",
        alias = "log_file_path_default"
    )]
    /// flag allowing for reverting a builder-configured
    /// setting at runtime
    pub(crate) log_file_path_unset: bool,
}
impl CrabLogOverrideConfig {
    /// Enables the Logger library
    ///
    /// ---
    ///
    /// - Library default: **`true`**
    #[must_use]
    pub fn enabled(&self) -> Option<bool> {
        self.enabled
    }
    /// Returns true if the `enabled` unset field has been explicitly set to true
    #[must_use]
    pub fn enabled_unset(&self) -> bool {
        self.enabled_unset
    }
    /// Enables verbose logging
    ///
    /// ---
    ///
    /// - Library default: **`false`**
    #[must_use]
    pub fn verbose(&self) -> Option<bool> {
        self.verbose
    }
    /// Returns true if the `verbose` unset field has been explicitly set to true
    #[must_use]
    pub fn verbose_unset(&self) -> bool {
        self.verbose_unset
    }
    /// Enables file logging at the given path
    ///
    /// If no path is provided, file logging is disabled.
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub fn log_file_path(&self) -> Option<&str> {
        self.log_file_path.as_ref().map(|s| s.as_str())
    }
    /// Returns true if the `log_file_path` unset field has been explicitly set to true
    #[must_use]
    pub fn log_file_path_unset(&self) -> bool {
        self.log_file_path_unset
    }
}
#[automatically_derived]
#[allow(clippy::redundant_field_names)]
impl Default for CrabLogOverrideConfig {
    fn default() -> Self {
        Self {
            enabled: None,
            enabled_unset: false,
            verbose: None,
            verbose_unset: false,
            log_file_path: None,
            log_file_path_unset: false,
        }
    }
}
