use super::*;

///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct WithRequiredConfig {
    pub(crate) foo: bool,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl WithRequiredConfig {
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
    pub(crate) fn new(foo: bool) -> Self {
        Self { foo: foo }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder(foo: bool) -> WithRequiredConfigBuilder {
        <WithRequiredConfigBuilder>::new(foo)
    }
    ///
    /// ---
    ///
    #[must_use]
    pub fn foo(&self) -> bool {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(&mut self, override_conf: WithRequiredOverrideConfig) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::marker::Copy for WithRequiredConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride for WithRequiredConfig {
    type Override = WithRequiredOverrideConfig;
    fn merge_with_override(&mut self, override_conf: Self::Override) {
        {
            let val = override_conf.foo;
            if let ::std::option::Option::Some(val) = val {
                self.foo = val;
            }
        }
    }
}
#[automatically_derived]
impl ::std::convert::TryFrom<WithRequiredOverrideConfig> for WithRequiredConfig {
    type Error = WithRequiredConfigError;
    fn try_from(
        override_conf: WithRequiredOverrideConfig,
    ) -> ::std::result::Result<Self, Self::Error> {
        let out = Self {
            foo: {
                let val = override_conf.foo;
                if let ::std::option::Option::Some(val) = val {
                    val
                } else {
                    ::std::result::Result::Err(WithRequiredConfigError::MissingRequiredFields {
                        field_names: vec!["foo".to_string()],
                    })?
                }
            },
        };
        ::std::result::Result::Ok(out)
    }
}
#[automatically_derived]
impl ::std::ops::Add<WithRequiredOverrideConfig> for WithRequiredConfig {
    type Output = Self;
    fn add(mut self, rhs: WithRequiredOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<WithRequiredOverrideConfig> for WithRequiredConfig {
    fn add_assign(&mut self, rhs: WithRequiredOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Possible errors encountered during build of [`WithRequiredConfig`](struct@WithRequiredConfig)
#[derive(::std::fmt::Debug)]
pub enum WithRequiredConfigError {
    /// Unable to initialize struct due to missing required fields
    MissingRequiredFields {
        /// Names of missing fields
        field_names: Vec<String>,
    },
}
impl WithRequiredConfigError {
    #[doc(hidden)]
    fn __error_field_list(names: &[::std::string::String]) -> ::std::string::String {
        names
            .iter()
            .map(|name| format!("- `{name}`"))
            .collect::<::std::vec::Vec<_>>()
            .join("\n")
    }
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
impl ::std::fmt::Display for WithRequiredConfigError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let text = match self {
            Self::MissingRequiredFields { field_names } => {
                format!(
                    "Missing required `WithRequiredConfig` fields:\n{}",
                    Self::__error_field_list(field_names)
                )
            }
        };
        write!(f, "{text}")?;
        ::std::result::Result::Ok(())
    }
}
#[automatically_derived]
impl ::std::error::Error for WithRequiredConfigError {}
#[automatically_derived]
impl ::lib_conf::traits::error::ConfigError for WithRequiredConfigError {
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
/// Provides a builder used to configure [`WithRequiredConfig`](struct@WithRequiredConfig) at compile-time.
///
/// Can be constructed using either [`WithRequiredConfig::builder()`](method@WithRequiredConfig::builder) or [`WithRequiredConfigBuilder::new()`](method@WithRequiredConfigBuilder::new).
///
/// For run-time configuration of `WithRequiredConfig`, see [`WithRequiredOverrideConfig`](struct@WithRequiredOverrideConfig)
#[derive(Debug, Clone, Copy)]
pub struct WithRequiredConfigBuilder {
    pub(crate) inner: WithRequiredConfig,
    pub(crate) override_conf: ::std::option::Option<WithRequiredOverrideConfig>,
}
impl WithRequiredConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new(foo: bool) -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <WithRequiredConfig>::new(foo),
        }
    }
    ///
    /// ---
    ///
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
    pub fn with_override(mut self, override_conf: WithRequiredOverrideConfig) -> Self {
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
    /// Builds the [`WithRequiredConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> WithRequiredConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig for WithRequiredConfigBuilder {
    type Config = WithRequiredConfig;
    fn build(mut self) -> Self::Config {
        if let ::std::option::Option::Some(override_conf) = self.override_conf {
            <Self::Config as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
                &mut self.inner,
                override_conf,
            );
        }
        self.inner
    }
}
/// Provides support for configuring [`WithRequiredConfig`](struct@WithRequiredConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `WithRequiredOverrideConfig` is provided for just this case.
///
/// ---
///
/// `WithRequiredOverrideConfig` provides run-time support for:
/// - *Setting* [`WithRequiredConfig`](struct@WithRequiredConfig) fields
/// - *Overriding* compile-time assignments (made using [`WithRequiredConfigBuilder`](struct@WithRequiredConfigBuilder))
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     with_required: WithRequiredOverrideConfig,
/// }
/// ```
///
/// # Fields
///
/// | Name | Type |
/// | - | - |
/// | `foo` | `bool` |
#[derive(Debug, Clone, Copy, ::serde::Deserialize)]
pub struct WithRequiredOverrideConfig {
    ///
    /// ---
    ///
    pub foo: ::std::option::Option<bool>,
}
impl WithRequiredOverrideConfig {
    ///
    /// ---
    ///
    #[must_use]
    pub fn foo(&self) -> ::std::option::Option<bool> {
        self.foo
    }
}
#[automatically_derived]
#[allow(
    clippy::redundant_field_names,
    reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
)]
impl ::std::default::Default for WithRequiredOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
        }
    }
}
