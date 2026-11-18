use crate::{
    error::ConfigError,
    merge::{MergeOverride, TryMergeOverride},
};

/// Indicates support for building a [`Config`](Self::Config) struct featuring
/// infallible override merge.
///
/// Infallible merge support requires the underlying [`Config`](Self::Config)
/// struct to implement the [`MergeOverride`] trait.
///
/// # Implementation notes
///
/// Inplementations should typically be on a named `Builder` struct containing 2 fields:
///
/// - `inner:` [`Config`](Self::Config)
/// - `override_conf:` [`Option<OverrideConfig>`](@'MergeOverride::Override)
///
/// **Note:** `BuildConfig` and [`TryBuildConfig`] should not be implemented for the same
/// type simultaneously.
pub trait BuildConfig {
    /// The primary config struct (with infallible override merge)
    type Config: MergeOverride;

    /// Builds the final [`Config`](Self::Config) struct.
    ///
    /// # Implementation notes
    ///
    /// Assuming the [recommended type structure](BuildConfig#implementation-notes) is followed,
    /// implementations should resemble:
    ///
    /// ```ignore
    /// fn build(mut self) -> Self::Config {
    ///     if let Some(override_config) = self.override_config {
    ///         self.inner.merge_with_override(override_config);
    ///     }
    ///
    ///     self.inner
    /// }
    /// ```
    fn build(self) -> Self::Config;
}

/// Indicates support for building a [`Config`](Self::Config) struct featuring
/// fallible override merge.
///
/// Fallible merge support requires the underlying [`Config`](Self::Config)
/// struct to implement the [`TryMergeOverride`] trait.
///
/// # Implementation notes
///
/// Inplementations should typically be on a named `Builder` struct containing 2 fields:
///
/// - `inner:` [`Config`](Self::Config)
/// - `override_conf:` [`Option<OverrideConfig>`](TryMergeOverride::Override)
///
/// **Note:** `TryBuildConfig` and [`BuildConfig`] should not be implemented for the same
/// type simultaneously.
///
/// # Errors
///
/// Both [`TryBuildConfig`] and [`TryMergeOverride`] must share the same error type.
/// This error type must implement [`ConfigError`].
pub trait TryBuildConfig {
    /// The primary config struct (with fallible override merge)
    type Config: TryMergeOverride<Error = Self::Error>;
    /// The error type used by [`try_build`](Self::try_build).
    ///
    /// This type must be identical to the error type set by [`TryMergeOverride::Error`].
    ///
    /// See the [`ConfigError`] trait docs for more information.
    type Error: ConfigError;
    
    /// Attempts to build the final [`Config`](Self::Config) struct.
    ///
    /// # Implementation notes
    ///
    /// Assuming the [recommended type structure](TryBuildConfig#implementation-notes) is followed,
    /// implementations should resemble:
    ///
    /// ```ignore
    /// fn try_build(mut self) -> Result<Self::Config, Self::Error> {
    ///     if let Some(override_config) {
    ///         self.inner.try_merge_with_override(override_config)?;
    ///     }
    ///     // if validation is required
    ///     my_validate_fn(&self.inner)?; // note: validate fn should also be called within try_merge_with_override.
    ///                                   //       in which case, this should be in an else block to avoid a double call
    ///     Ok(self.inner)
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Possible failure scenarios are implementor-dependent, but may include:
    ///
    /// - Failure in validation of self or a nested config
    /// - An optional nested config is only partially provided
    fn try_build(self) -> Result<Self::Config, Self::Error>;
}
