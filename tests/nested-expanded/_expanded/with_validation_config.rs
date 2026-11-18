use super::*;
///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct WithValidationConfig {
    pub(crate) foo: bool,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl WithValidationConfig {
    #[doc(hidden)]
    #[allow(dead_code)]
    fn __assertions() {
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            bool : ::std::clone::Clone, ::std::fmt::Debug
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            bool : ::std::marker::Copy
        );
    }
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub(crate) fn new() -> Self {
        Self { foo: true }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder() -> WithValidationConfigBuilder {
        <WithValidationConfigBuilder>::new()
    }
    ///
    /// ---
    ///
    /// - Library default: **`true`**
    #[must_use]
    pub fn foo(&self) -> bool {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn try_merge_with_override(
        &mut self,
        override_conf: WithValidationOverrideConfig,
    ) -> ::std::result::Result<(), WithValidationConfigError> {
        <Self as ::lib_conf::traits::merge::TryMergeOverride>::try_merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::default::Default for WithValidationConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::std::marker::Copy for WithValidationConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::TryMergeOverride for WithValidationConfig {
    type Override = WithValidationOverrideConfig;
    type Error = WithValidationConfigError;
    fn try_merge_with_override(
        &mut self,
        override_conf: Self::Override,
    ) -> ::std::result::Result<(), Self::Error> {
        {
            let val = override_conf.foo;
            if override_conf.foo_unset {
                self.foo = true;
            } else if let ::std::option::Option::Some(val) = val {
                self.foo = val;
            }
        }
        validate_conf(&self)?;
        ::std::result::Result::Ok(())
    }
}
#[automatically_derived]
impl ::std::convert::TryFrom<WithValidationOverrideConfig> for WithValidationConfig {
    type Error = WithValidationConfigError;
    fn try_from(
        override_conf: WithValidationOverrideConfig,
    ) -> ::std::result::Result<Self, Self::Error> {
        let out = Self {
            foo: {
                let val = override_conf.foo;
                if override_conf.foo_unset {
                    true
                } else if let ::std::option::Option::Some(val) = val {
                    val
                } else {
                    true
                }
            },
        };
        validate_conf(&out)?;
        ::std::result::Result::Ok(out)
    }
}
/// Possible errors encountered during builf of [`WithValidationConfig`](struct@WithValidationConfig)
#[derive(::std::fmt::Debug)]
pub enum WithValidationConfigError {
    /// Config init failed during validation
    Validation(ValidationError),
}
impl WithValidationConfigError {
    /// List of field names leading to the source of a nested error.
    ///
    /// Does not include the offending field(s) themselves, just the sequence of fields leading to their containing `Struct`
    pub fn source_chain(&self) -> ::std::vec::Vec<::std::string::String> {
        <Self as ::lib_conf::traits::error::ConfigError>::source_chain(self)
    }
    /// Same as [`source_chain()`](Self::source_chain), but returns the chain as a single dot-delimited string
    pub fn source_path(&self) -> ::std::string::String {
        <Self as ::lib_conf::traits::error::ConfigError>::source_path(self)
    }
    /// Returns the source error message
    ///
    /// Used to generate the full error including the nested path
    pub fn origin_message(&self) -> ::std::string::String {
        <Self as ::lib_conf::traits::error::ConfigError>::origin_message(self)
    }
}
#[automatically_derived]
impl ::std::fmt::Display for WithValidationConfigError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let text = match self {
            Self::Validation(error) => error.to_string(),
        };
        write!(f, "{text}")?;
        ::std::result::Result::Ok(())
    }
}
#[automatically_derived]
impl ::std::error::Error for WithValidationConfigError {}
#[automatically_derived]
impl ::lib_conf::traits::error::ConfigError for WithValidationConfigError {
    fn build_field_chain(
        &self,
        chain: &mut ::std::vec::Vec<::std::string::String>,
        self_field_name: ::std::option::Option<&str>,
    ) {
        if let ::std::option::Option::Some(name) = self_field_name {
            chain.push(name.to_string());
        }
        match self {
            _ => {}
        };
    }
    fn origin_message(&self) -> ::std::string::String {
        match self {
            _ => self.to_string(),
        }
    }
}
#[automatically_derived]
impl ::std::convert::From<ValidationError> for WithValidationConfigError {
    fn from(validation_error: ValidationError) -> Self {
        Self::Validation(validation_error)
    }
}
/// Provides a builder used to configure [`WithValidationConfig`](struct@WithValidationConfig) at compile-time.
///
/// Can be constructed using either [`WithValidationConfig::builder()`](method@WithValidationConfig::builder) or [`WithValidationConfigBuilder::new()`](method@WithValidationConfigBuilder::new).
///
/// For run-time configuration of `WithValidationConfig`, see [`WithValidationOverrideConfig`](struct@WithValidationOverrideConfig)
#[derive(Debug, Clone)]
#[derive(Copy)]
pub struct WithValidationConfigBuilder {
    pub(crate) inner: WithValidationConfig,
    pub(crate) override_conf: ::std::option::Option<WithValidationOverrideConfig>,
}
impl WithValidationConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new() -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <WithValidationConfig>::new(),
        }
    }
    /// # Revert at run-time
    ///
    /// Has run-time support for reverting back to the provided default (**`true`**)
    ///
    /// See [`foo_unset()`](method@WithValidationOverrideConfig::foo_unset) for more information
    ///
    /// ---
    ///
    /// - Library default: **`true`**
    #[must_use]
    pub fn foo(mut self, val: bool) -> Self {
        self.inner.foo = val;
        self
    }
    /// If supplied, will overwrite any values present in the provided
    /// override config.
    ///
    /// This happens as the last step within the
    ///[`build()`](self.build) method.
    ///
    /// Any previous calls to [`with_override()`](Self::with_override)
    /// in the builder chain will have no effect on the resulting
    /// config.
    ///
    /// The order in which this is chained with the builder's setter
    /// methods does not matter.
    #[must_use]
    pub fn with_override(mut self, override_conf: WithValidationOverrideConfig) -> Self {
        self.override_conf = ::std::option::Option::Some(override_conf);
        self
    }
    /// Clears the override config previously set via
    /// [`with_override()`](Self::with_override).
    #[must_use]
    pub fn clear_override(mut self) -> Self {
        self.override_conf = ::std::option::Option::None;
        self
    }
    /// Builds the [`WithValidationConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn try_build(
        mut self,
    ) -> ::std::result::Result<WithValidationConfig, WithValidationConfigError> {
        <Self as ::lib_conf::traits::build::TryBuildConfig>::try_build(self)
    }
}
#[automatically_derived]
impl ::std::default::Default for WithValidationConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::TryBuildConfig for WithValidationConfigBuilder {
    type Config = WithValidationConfig;
    type Error = WithValidationConfigError;
    fn try_build(mut self) -> ::std::result::Result<Self::Config, Self::Error> {
        if let ::std::option::Option::Some(override_conf) = self.override_conf {
            <Self::Config as ::lib_conf::traits::merge::TryMergeOverride>::try_merge_with_override(
                &mut self.inner,
                override_conf,
            )?;
        }
        validate_conf(&self.inner)?;
        ::std::result::Result::Ok(self.inner)
    }
}
/// Provides support for configuring [`WithValidationConfig`](struct@WithValidationConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `WithValidationOverrideConfig` is provided for just this case.
///
/// ---
///
/// `WithValidationOverrideConfig` provides run-time support for:
/// - *Setting* [`WithValidationConfig`](struct@WithValidationConfig) fields
/// - *Overriding* compile-time assignments (made using [`WithValidationConfigBuilder`](struct@WithValidationConfigBuilder))
/// - *Reverting* compile-time (or run-time) assignments back to provided defaults
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     with_validation: WithValidationOverrideConfig,
/// }
/// ```
///
/// # Fields
///
/// | Name | Type |
/// | - | - |
/// | `foo` | `bool` |
///
/// ## Revert to default
///
/// For each of [`WithValidationConfig`](struct@WithValidationConfig)'s optional fields, there is a corresponding **`bool`** with a name in the form of `*_unset`.
///
/// If set to `true`, the associated field will have its value reverted to the provided default, e.g.
/// - if `foo_unset` deserializes to `true`:
///   - `foo` will be reverted back to the provided default: `true`
///
/// ### Unset fields
///
/// The following fields are supported:
///
/// | Name |
/// | - |
/// | `foo_unset` |
///
/// ### Unset aliases
///
/// Defaults can also be restored using the following aliases
///
/// | Alias |
/// | - |
/// | `reset` |
/// | `revert` |
/// | `clear` |
/// | `default` |
///
/// For example, `foo` could be reset using either `foo_unset`, or any of the following:
/// - `foo_reset`
/// - `foo_revert`
/// - `foo_clear`
/// - `foo_default`
#[derive(Debug, Clone)]
#[derive(Copy)]
#[derive(::serde::Deserialize)]
pub struct WithValidationOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`true`**
    pub foo: ::std::option::Option<bool>,
    #[serde(
        default,
        alias = "foo_reset",
        alias = "foo_revert",
        alias = "foo_clear",
        alias = "foo_default"
    )]
    /// flag allowing for reverting a builder-configured
    /// setting at runtime
    pub foo_unset: bool,
}
impl WithValidationOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`true`**
    #[must_use]
    pub fn foo(&self) -> ::std::option::Option<bool> {
        self.foo
    }
    /// Whether the unset field for [`foo`](Self::foo) has explicitly been set to `true`.
    ///
    /// # Aliases
    ///
    /// Various aliases are supported in addition to reverting via `foo_unset`.
    ///
    /// - **Note:** unset aliases are only supported when the **`serde` feature is enabled**.
    ///
    /// <details>
    /// <summary><b>Expand to view supported unset field aliases</b></summary
    ///
    /// > - `foo_reset`
    /// > - `foo_revert`
    /// > - `foo_clear`
    /// > - `foo_default`
    ///
    /// </details>
    #[must_use]
    pub fn foo_unset(&self) -> bool {
        self.foo_unset
    }
}
#[automatically_derived]
#[allow(
    clippy::redundant_field_names,
    reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
)]
impl ::std::default::Default for WithValidationOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
            foo_unset: false,
        }
    }
}
