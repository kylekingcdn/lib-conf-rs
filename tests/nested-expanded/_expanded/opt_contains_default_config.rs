use super::*;

///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct OptContainsDefaultConfig {
    pub(crate) foo: Option<WithDefaultConfig>,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl OptContainsDefaultConfig {
    #[doc(hidden)]
    #[allow(dead_code)]
    fn __assertions() {
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            Option < WithDefaultConfig > : ::std::clone::Clone, ::std::fmt::Debug
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            Option < WithDefaultConfig > : ::std::marker::Copy
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithDefaultConfigBuilder : ::std::default::Default
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithDefaultConfig : ::lib_conf::traits::merge::MergeOverride < Override =
            WithDefaultOverrideConfig >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithDefaultConfigBuilder : ::lib_conf::traits::build::BuildConfig < Config =
            WithDefaultConfig >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithDefaultConfig : ::std::convert::From < WithDefaultOverrideConfig >
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
    pub fn builder() -> OptContainsDefaultConfigBuilder {
        <OptContainsDefaultConfigBuilder>::new()
    }
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub fn foo(&self) -> Option<WithDefaultConfig> {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(
        &mut self,
        override_conf: OptContainsDefaultOverrideConfig,
    ) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::default::Default for OptContainsDefaultConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::std::marker::Copy for OptContainsDefaultConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride for OptContainsDefaultConfig {
    type Override = OptContainsDefaultOverrideConfig;
    fn merge_with_override(&mut self, override_conf: Self::Override) {
        {
            let val = override_conf.foo;
            if let ::std::option::Option::Some(val) = val {
                if let ::std::option::Option::Some(current) = &mut self.foo {
                    <WithDefaultConfig as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
                        current,
                        val,
                    );
                } else {
                    let mut inited = <WithDefaultConfig>::from(val);
                    self.foo = ::std::option::Option::Some(inited);
                }
            }
        }
    }
}
#[automatically_derived]
impl ::std::convert::From<OptContainsDefaultOverrideConfig>
for OptContainsDefaultConfig {
    fn from(override_conf: OptContainsDefaultOverrideConfig) -> Self {
        let mut out = Self::default();
        out.merge_with_override(override_conf);
        out
    }
}
#[automatically_derived]
impl ::std::ops::Add<OptContainsDefaultOverrideConfig> for OptContainsDefaultConfig {
    type Output = Self;
    fn add(mut self, rhs: OptContainsDefaultOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<OptContainsDefaultOverrideConfig>
for OptContainsDefaultConfig {
    fn add_assign(&mut self, rhs: OptContainsDefaultOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Provides a builder used to configure [`OptContainsDefaultConfig`](struct@OptContainsDefaultConfig) at compile-time.
///
/// Can be constructed using either [`OptContainsDefaultConfig::builder()`](method@OptContainsDefaultConfig::builder) or [`OptContainsDefaultConfigBuilder::new()`](method@OptContainsDefaultConfigBuilder::new).
///
/// For run-time configuration of `OptContainsDefaultConfig`, see [`OptContainsDefaultOverrideConfig`](struct@OptContainsDefaultOverrideConfig)
#[derive(Debug, Clone, Copy, Default)]
pub struct OptContainsDefaultConfigBuilder {
    pub(crate) inner: OptContainsDefaultConfig,
    pub(crate) override_conf: ::std::option::Option<OptContainsDefaultOverrideConfig>,
}
impl OptContainsDefaultConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new() -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <OptContainsDefaultConfig>::new(),
        }
    }
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub(crate) fn foo(mut self, val: Option<WithDefaultConfig>) -> Self {
        self.inner.foo = val;
        self
    }
    /// Allows for inline configuration of nested [`WithDefaultConfig`](struct@WithDefaultConfig)
    ///
    /// Closure has a single [`WithDefaultConfigBuilder`](struct@WithDefaultConfigBuilder) parameter and returns the builder after configured.
    ///
    /// The [`WithDefaultConfigBuilder`](struct@WithDefaultConfigBuilder) returned by the closure will automatically be built (and associated [`WithDefaultOverrideConfig`](struct@WithDefaultOverrideConfig) merged).
    #[must_use]
    pub fn foo_builder<F>(mut self, f: F) -> Self
    where
        F: Fn() -> WithDefaultConfigBuilder,
    {
        let builder = f();
        self.inner.foo = ::std::option::Option::Some(
            <WithDefaultConfigBuilder as ::lib_conf::traits::build::BuildConfig>::build(
                builder,
            ),
        );
        self
    }
    /// Allows for inline configuration of nested [`WithDefaultConfig`](struct@WithDefaultConfig)
    ///
    /// Closure has a single [`WithDefaultConfigBuilder`](struct@WithDefaultConfigBuilder) parameter and returns the builder after configured.
    ///
    /// The [`WithDefaultConfigBuilder`](struct@WithDefaultConfigBuilder) returned by the closure will automatically be built (and associated [`WithDefaultOverrideConfig`](struct@WithDefaultOverrideConfig) merged).
    #[must_use]
    pub fn foo_builder_default<F>(mut self, f: F) -> Self
    where
        F: Fn(WithDefaultConfigBuilder) -> WithDefaultConfigBuilder,
    {
        let builder = f(<WithDefaultConfigBuilder>::default());
        self.inner.foo = ::std::option::Option::Some(
            <WithDefaultConfigBuilder as ::lib_conf::traits::build::BuildConfig>::build(
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
        override_conf: OptContainsDefaultOverrideConfig,
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
    /// Builds the [`OptContainsDefaultConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> OptContainsDefaultConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig for OptContainsDefaultConfigBuilder {
    type Config = OptContainsDefaultConfig;
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
/// Provides support for configuring [`OptContainsDefaultConfig`](struct@OptContainsDefaultConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `OptContainsDefaultOverrideConfig` is provided for just this case.
///
/// ---
///
/// `OptContainsDefaultOverrideConfig` provides run-time support for:
/// - *Setting* [`OptContainsDefaultConfig`](struct@OptContainsDefaultConfig) fields
/// - *Overriding* compile-time assignments (made using [`OptContainsDefaultConfigBuilder`](struct@OptContainsDefaultConfigBuilder))
/// - *Reverting* compile-time (or run-time) assignments back to provided defaults
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     opt_contains_default: OptContainsDefaultOverrideConfig,
/// }
/// ```
///
/// # Fields
///
/// | Name | Type |
/// | - | - |
/// | `foo` | `WithDefaultOverrideConfig` |
#[derive(Debug, Clone, Copy, ::serde::Deserialize)]
pub struct OptContainsDefaultOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    pub foo: ::std::option::Option<WithDefaultOverrideConfig>,
}
impl OptContainsDefaultOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub fn foo(&self) -> ::std::option::Option<WithDefaultOverrideConfig> {
        self.foo
    }
}
#[automatically_derived]
#[allow(
    clippy::redundant_field_names,
    reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
)]
impl ::std::default::Default for OptContainsDefaultOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
        }
    }
}
