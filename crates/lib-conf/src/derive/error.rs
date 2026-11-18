// !- Traits

// ! Seals

mod sealed {
    pub(super) trait Sealed {}
}

/// Implemented on lib-conf's base provided Error type
///
/// Used to prevent working around re-export restrictions via struct `error` attr
#[doc(hidden)]
#[expect(private_bounds, reason = "Sealed")]
pub trait ConfigErrorBundled: sealed::Sealed {}

// !- Error types

// ! Build error

/// Source error for default derived build error type.
///
/// A custom error type can be substituted using the `build_error` attr.
///
/// Custom build errors must impl:
/// - `From<Merge{ConfigStructName}Error>`
///   - The `Merge{ConfigStructName}Error` is automatically generated/derived
/// - If the `validation_fn` struct attr is present, the custom error type must also impl:
///   - `From<{validation_error}>`
///     - Where `{validation_error}` is the type provided by the corresponding struct attr.
///
/// # Examples
///
/// This can be done very easily with the help of [`thiserror`].
///
/// E.g, for a `Config` struct named `SdkConfig`:
///
/// ```
/// /// Error returned by [`SdkConfigBuilder::try_build()`]
/// #[derive(Debug, thiserror::Error)]
/// pub enum BuildSdkConfigError {
///     /// An error occurred while merging [`SdkOverrideConfig`] into [`SdkConfig`]
///     #[error(transparent)]
///     Merge(#[from] MergeSdkConfigError), // this type is already generated for you!
///
///     // -- Note: the following is only needed when using the `validate_fn` attr
///
///     /// An error occurred while validating the built [`SdkConfig`]
///     #[error(transparent)]
///     Validate(#[from] ValidateSdkConfigError),
///     //               ^^^^^^^^^^^|^^^^^^^^^^
///     //     the error type retured by your validate fn
///
///     // ..
/// }
/// ```
#[derive(Debug, thiserror::Error)]
pub enum BuildConfigError {
    /// An error occurred during override config
    #[error(transparent)]
    Merge(#[from] MergeConfigError),
}
impl sealed::Sealed for BuildConfigError {}
impl ConfigErrorBundled for BuildConfigError {}

// ! Merge error

/// Source error for derived custom merge error type.
///
/// The derived error mirrors this definition.
#[derive(Debug, thiserror::Error)]
pub enum MergeConfigError {
    /// Unabke to initialize Config struct due to unset required fields
    #[error("Config is missing required fields:\n`{}`", error_field_list(field_names))]
    MissingRequiredFields {
        /// Names of missing fields
        field_names: Vec<String>,
    },
}
impl sealed::Sealed for MergeConfigError {}
impl ConfigErrorBundled for MergeConfigError {}

// !- Utils

#[doc(hidden)]
#[must_use]
pub fn error_field_list(names: &[String]) -> String {
    names.iter().map(|name| format!("- `{name}`")).collect::<Vec<_>>().join("\n")
}
