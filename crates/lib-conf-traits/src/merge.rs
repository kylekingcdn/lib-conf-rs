use crate::ConfigError;

/// Indicates support for infallibly merging an `Override` config struct into self.
///
/// # Implementation notes
///
/// The implementing type should typically be a named struct.
/// 
/// The [`Override`](Self::Override) type should mirror the implementing type, with the
/// notable change of all types being wrapped in `Option<_>`.
///
/// If a type is already `Option<_>`, the `Override` counterpart does not need to be
// wrapped for a second time.
///
/// **Note:** `MergeOverride` and [`TryMergeOverride`] should not be implemented for the same
/// type simultaneously.
pub trait MergeOverride {
    /// The config struct's `Override` counterpart.
    ///
    /// The [`Override`](Self::Override) type should mirror the implementing type, with the
    /// notable change of all types being wrapped in `Option<_>`.
    ///
    /// If a type is already `Option<_>`, the `Override` counterpart does not need to be
    /// wrapped for a second time.
    type Override;
    
    /// Merges the [`Override`](Self::Override) struct into self.
    ///
    /// # Implementation notes
    ///
    /// Implementation will typically resemble below, one for **each of the struct's fields**
    ///
    /// ```ignore
    /// fn merge_with_override(&mut self, override_config: Self::Override) {
    ///     // merge the `verbose` field in (non-`Option` in `Config`)
    ///     if let Some(verbose) = override_config.verbose {
    ///         self.verbose = verbose;
    ///     }
    /// 
    ///     // merge the `log_level` field in (this field is `Option<_>` in `Config`)
    ///     if let Some(log_level) = override_config.level {
    ///         // note that we don't overwrite if `Override` field is `None`.
    ///         self.log_level = Some(log_level);
    ///     }
    /// }
    /// ```
    fn merge_with_override(&mut self, override_config: Self::Override);
}

/// Indicates support for fallibly merging of an `Override` config struct into self.
///
/// # Implementation notes
///
/// The implementing type should typically be a named struct.
///
/// The [`Override`](Self::Override) type should mirror the implementing type, with the
/// notable change of all types being wrapped in `Option<_>`.
///
/// If a type is already `Option<_>`, the `Override` counterpart does not need to be
/// wrapped for a second time.
///
/// **Note:** `TryMergeOverride` and [`MergeOverride`] should not be implemented for the same
/// type simultaneously.
pub trait TryMergeOverride {
    /// The config struct's `Override` counterpart.
    ///
    /// The [`Override`](Self::Override) type should mirror the implementing type, with the
    /// notable change of all types being wrapped in `Option<_>`.
    ///
    /// If a type is already `Option<_>`, the `Override` counterpart does not need to be
    /// wrapped for a second time.
    type Override;
    
    /// The common error type used for the `Config` struct.
    ///
    /// See the [`ConfigError`] trait docs for more information.
    type Error: ConfigError;
    
    /// Attempts to merge the [`Override`](Self::Override) struct into self.
    ///
    /// # Errors
    ///
    /// Possible failure scenarios are implementor-dependent, but may include:
    ///
    /// - Failure in validation of self or a nested config
    /// - An optional nested config is only partially provided
    ///
    /// # Implementation notes
    ///
    /// Implementation will typically resemble below, one for **each of the struct's fields**
    ///
    /// ```ignore
    /// fn try_merge_with_override(&mut self, override_config: Self::Override) -> Result<(), Self::Error> {
    ///     // merge the `verbose` field in (non-`Option` in `Config`)
    ///     if let Some(verbose) = override_config.verbose {
    ///         self.verbose = verbose;
    ///     }
    /// 
    ///     // merge the `log_level` field in (this field is `Option<_>` in `Config`)
    ///     if let Some(log_level) = override_config.level {
    ///         // note that we don't overwrite if `Override` field is `None`.
    ///         self.log_level = Some(log_level);
    ///     }
    ///
    ///     // if validation is required
    ///     my_validate_fn(&self.inner)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn try_merge_with_override(&mut self, override_config: Self::Override) -> Result<(), Self::Error>;
}
