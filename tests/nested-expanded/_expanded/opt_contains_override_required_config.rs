use super::*;

///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct OptContainsOverrideRequiredConfig {
    pub(crate) foo: Option<WithOverrideRequiredConfig>,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl OptContainsOverrideRequiredConfig {
    #[doc(hidden)]
    #[allow(dead_code)]
    fn __assertions() {
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            Option < WithOverrideRequiredConfig > : ::std::clone::Clone,
            ::std::fmt::Debug
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            Option < WithOverrideRequiredConfig > : ::std::marker::Copy
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithOverrideRequiredConfig : ::lib_conf::traits::merge::MergeOverride <
            Override = WithOverrideRequiredOverrideConfig >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithOverrideRequiredConfigBuilder : ::lib_conf::traits::build::BuildConfig <
            Config = WithOverrideRequiredConfig >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithOverrideRequiredConfig : ::std::convert::From <
            WithOverrideRequiredOverrideConfig >
        );
    }
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub(crate) fn new() -> Self {
        Self { foo: None }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder() -> OptContainsOverrideRequiredConfigBuilder {
        <OptContainsOverrideRequiredConfigBuilder>::new()
    }
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub fn foo(&self) -> Option<WithOverrideRequiredConfig> {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(
        &mut self,
        override_conf: OptContainsOverrideRequiredOverrideConfig,
    ) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::default::Default for OptContainsOverrideRequiredConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::std::marker::Copy for OptContainsOverrideRequiredConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride for OptContainsOverrideRequiredConfig {
    type Override = OptContainsOverrideRequiredOverrideConfig;
    fn merge_with_override(&mut self, override_conf: Self::Override) {
        {
            let val = override_conf.foo;
            if let ::std::option::Option::Some(val) = val {
                if let ::std::option::Option::Some(current) = &mut self.foo {
                    <WithOverrideRequiredConfig as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
                        current,
                        val,
                    );
                } else {
                    let mut inited = <WithOverrideRequiredConfig>::from(val);
                    self.foo = ::std::option::Option::Some(inited);
                }
            }
        }
    }
}
#[automatically_derived]
impl ::std::convert::From<OptContainsOverrideRequiredOverrideConfig>
for OptContainsOverrideRequiredConfig {
    fn from(override_conf: OptContainsOverrideRequiredOverrideConfig) -> Self {
        let mut out = Self::default();
        out.merge_with_override(override_conf);
        out
    }
}
#[automatically_derived]
impl ::std::ops::Add<OptContainsOverrideRequiredOverrideConfig>
for OptContainsOverrideRequiredConfig {
    type Output = Self;
    fn add(mut self, rhs: OptContainsOverrideRequiredOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<OptContainsOverrideRequiredOverrideConfig>
for OptContainsOverrideRequiredConfig {
    fn add_assign(&mut self, rhs: OptContainsOverrideRequiredOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Provides a builder used to configure [`OptContainsOverrideRequiredConfig`](struct@OptContainsOverrideRequiredConfig) at compile-time.
///
/// Can be constructed using either [`OptContainsOverrideRequiredConfig::builder()`](method@OptContainsOverrideRequiredConfig::builder) or [`OptContainsOverrideRequiredConfigBuilder::new()`](method@OptContainsOverrideRequiredConfigBuilder::new).
///
/// For run-time configuration of `OptContainsOverrideRequiredConfig`, see [`OptContainsOverrideRequiredOverrideConfig`](struct@OptContainsOverrideRequiredOverrideConfig)
#[derive(Debug, Clone, Copy, Default)]
pub struct OptContainsOverrideRequiredConfigBuilder {
    pub(crate) inner: OptContainsOverrideRequiredConfig,
    pub(crate) override_conf: ::std::option::Option<
        OptContainsOverrideRequiredOverrideConfig,
    >,
}
impl OptContainsOverrideRequiredConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new() -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <OptContainsOverrideRequiredConfig>::new(),
        }
    }
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub(crate) fn foo(mut self, val: Option<WithOverrideRequiredConfig>) -> Self {
        self.inner.foo = val;
        self
    }
    /// Allows for inline configuration of nested [`WithOverrideRequiredConfig`](struct@WithOverrideRequiredConfig)
    ///
    /// Closure takes no parameters, and returns a compile-time configured [`WithOverrideRequiredConfigBuilder`](struct@WithOverrideRequiredConfigBuilder).
    ///
    /// The [`WithOverrideRequiredConfigBuilder`](struct@WithOverrideRequiredConfigBuilder) returned by the closure will automatically be built (and associated [`WithOverrideRequiredOverrideConfig`](struct@WithOverrideRequiredOverrideConfig) merged).
    #[must_use]
    pub fn foo_builder<F>(mut self, f: F) -> Self
    where
        F: Fn() -> WithOverrideRequiredConfigBuilder,
    {
        let builder = f();
        self.inner.foo = ::std::option::Option::Some(
            <WithOverrideRequiredConfigBuilder as ::lib_conf::traits::build::BuildConfig>::build(
                builder,
            ),
        );
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
    pub fn with_override(
        mut self,
        override_conf: OptContainsOverrideRequiredOverrideConfig,
    ) -> Self {
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
    /// Builds the [`OptContainsOverrideRequiredConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> OptContainsOverrideRequiredConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig
for OptContainsOverrideRequiredConfigBuilder {
    type Config = OptContainsOverrideRequiredConfig;
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
/// Provides support for configuring [`OptContainsOverrideRequiredConfig`](struct@OptContainsOverrideRequiredConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `OptContainsOverrideRequiredOverrideConfig` is provided for just this case.
///
/// ---
///
/// `OptContainsOverrideRequiredOverrideConfig` provides run-time support for:
/// - *Setting* [`OptContainsOverrideRequiredConfig`](struct@OptContainsOverrideRequiredConfig) fields
/// - *Overriding* compile-time assignments (made using [`OptContainsOverrideRequiredConfigBuilder`](struct@OptContainsOverrideRequiredConfigBuilder))
/// - *Reverting* compile-time (or run-time) assignments back to provided defaults
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     opt_contains_override_required: OptContainsOverrideRequiredOverrideConfig,
/// }
/// ```
///
/// # Fields
///
/// | Name | Type |
/// | - | - |
/// | `foo` | `WithOverrideRequiredOverrideConfig` |
#[derive(Debug, Clone, Copy, ::serde::Deserialize)]
pub struct OptContainsOverrideRequiredOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    pub foo: ::std::option::Option<WithOverrideRequiredOverrideConfig>,
}
impl OptContainsOverrideRequiredOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub fn foo(&self) -> ::std::option::Option<WithOverrideRequiredOverrideConfig> {
        self.foo
    }
}
#[automatically_derived]
#[allow(
    clippy::redundant_field_names,
    reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
)]
impl ::std::default::Default for OptContainsOverrideRequiredOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
        }
    }
}
