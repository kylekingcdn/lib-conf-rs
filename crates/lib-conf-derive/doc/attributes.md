# Attribute reference

## Quick links

- [Struct attributes](#struct-attributes)
  - [`derive`](#derive)
  - [`builder_derive`](#builder_derive)
  - [`override_derive`](#override_derive)
  - [`attr`](#attr)
  - [`builder_attr`](#builder_attr)
  - [`override_attr`](#override_attr_struct)
- [Field attributes](#field-attributes)
  - [`copy`](#copy)
  - [`default`](#default)
  - [`skip_all`](#skip_all)
  - [`config_skip_getter`](#config_skip_getter)
  - [`builder_skip`](#builder_skip)
  - [`override_skip`](#override_skip)
  - [`override_required`](#override_required)
  - [`override_from`](#override_from)
  - [`override_via`](#override_via)
  - [`override_attr`](#override_attr_field)

## Struct attributes

### `derive`

Applies the provided derive's to both the `Builder` and `Override` generated structs.

Can contain multiple derives and can be specified multiple times.

#### Example

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, Copy, LibConfig)]
#[config(derive(Copy))]
pub struct MySdkConfig {
    #[config(copy, default = false)]
    pub(crate) verbose: bool,
}
```

#### Considerations

- The resulting set of derives cannot contain duplicates.
  - Both the `Builder` struct and the `Override` struct automatically
    derive `Debug` and `Clone`.
  - Additionally, the `Override` struct will derive `serde::Deserialize`
    automatically if the `serde` feature is enabled.

### `builder_derive`

Applies the provided derive's to the generated `Builder` struct.

Can contain multiple derives and can be specified multiple times.

#### Example

In reality, this example is very questionable.

I can't reason why someone would want to derive `Deserialize` on a Builder struct.
However, this does showcase how to handle `serde` attributes in `serde`-optional libraries.

We can't use the `#[config(derive(..))]` here, as `Deserialize` gets auto-derived for `Override`
structs if the `serde` feature is enabled.

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))] // derive Deserialize on this struct
#[cfg_attr(feature = "serde", config(builder_derive(serde::Deserialize)))] // derive on builder
pub struct MySdkConfig {
    // ...
}
```

For `serde`-mandatory, this would be done as follows:

```rust
# #[cfg(feature = "serde")]
# {
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig, serde::Deserialize)] // derive Deserialize on this struct
#[config(builder_derive(serde::Deserialize))] // derive Deserialize on the Builder struct
pub struct MySdkConfig {
    // ...
}
# }
```

#### Considerations

- The resulting set of derives cannot contain duplicates
  - The `Builder` struct automatically derives `Debug` and `Clone`

### `override_derive`

Applies the provided derive's to the generated `Override` struct.

Can contain multiple derives and can be specified multiple times.

#### Example

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
#[config(override_derive(serde::Serialize))]
pub struct MySdkConfig {
    // ...
}
```

#### Considerations

- The resulting set of derives cannot contain duplicates
  - The override struct automatically derives `Debug`, `Clone`, and, if the
    `serde` feature is enabled, `serde::Deserialize`

### `attr`

Passes through the contained attributes to the `Builder` and `Override` generated structs.

Can contain multiple attributes and can be specified multiple times.

#### Example

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
#[config(attr(non_exhaustive))]
pub struct MySdkConfig {
    // ...
}
```

### `builder_attr`

Passes through the contained attributes to the generated `Builder` struct.

Can contain multiple attributes and can be specified multiple times.

#### Example

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
#[config(builder_attr(non_exhaustive))]
pub struct MySdkConfig {
    // ...
}
```

### `override_attr` <a name="override_attr_struct"></a>

Passes through the contained attributes to the generated `Override` struct.

Can contain multiple attributes and can be specified multiple times.

#### Example

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
#[config(override_attr(non_exhaustive))]
pub struct MySdkConfig {
    // ...
}
```

## Field attributes

### `copy`

Flag indicating that the field's type implements `Copy`.

This only affects the return types of getter methods associated with this field. Getter return types
will be an owned type rather than a borrow/ref.

In order to avoid wonky return types, this should *always* be used if the type does in fact
support `Copy`.

#### Example

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
pub struct MySdkConfig {
    #[config(copy)]
    pub(crate) worker_count: Option<u8>,

    pub(crate) client_name: Option<String>,
}
```

#### Considerations

- This behavior does not apply to the `Override`'s getter if either
  [`override_from`](#override_from) or [`override_via`](#override_via) are present
- See [Getter return types](#getter-return-types) for more information

### `default`

Sets the default value for the given field, which makes initialization optional for consumers.

Can be used on both `Option<_>` and non-`Option<_>` types.

#### Example

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
pub struct MySdkConfig {
    #[config(default = false)]
    pub(crate) debug_mode: bool,

    #[config(default = true)]
    pub(crate) ssl_cert_verification: bool,

    #[config(default = Some("local".to_string()))]
    pub(crate) deployment_env: Option<String>,

    #[config(default = String::from("https://api.example.com"))]
    pub(crate) endpoint: String,
}
```

#### Considerations

- Unlike `serde`'s `default` attribute, implicit (no-value) declarations are **not supported**
  - This is because the provided expression is directly used to provide the 'Library default' value
    in generated struct's method docs
- The provided value can be a literal (`true`, `5`, `Some(false)`), an
  expression (`String::from("local")`), or a path/fn call
  - Support for chained calls is not currently supported, however it is being worked on - e.g.
    - `"hello".to_string()` is **not** supported
    - `Some("hello".to_string())` ***is* supported**
    - `String::from("hello")` ***is* supported**

### `skip_all`

Flag indicating that this field should be skipped in the `Override` struct, and any relevant
methods for the deriving, `Builder`, and `Override` structs.

This is identical to specifying the `config_skip_getter`, `builder_skip`, and `override_skip`
attributes.

#### Example

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
pub struct MySdkConfig {
    #[config(skip_all)]
    pub(crate) some_internal_only_state: Option<bool>,
}
```

#### Considerations

- Implies: `config_skip_getter`, `builder_skip`, `override_skip`
- Incompatible with: `config_skip_getter`, `builder_skip`, `override_skip`, `override_required`,
  `override_from`, `override_via`, `override_attr`
- This should probably only be used with fields that are either `Option<_>` or have
  the `default` attribute present
  - Otherwise, consumers are forced to initialize this as a parameter in the `builder()` fn.
- If your config struct is generic, any skipped fields will be substituted
  with `PhantomData<{type}>` fields in order to preserve generics in the `Builder`
  and `Override` structs
  - Therefore, this should probably not be used with generic fields where the
    concrete types are not public-facing

### `config_skip_getter`

Flag used to skip the deriving struct's getter method for this field.

#### Example

```rust
# use lib_conf_derive as lib_conf;
use lib_conf::LibConfig;
use secrecy::SecretString;

#[derive(Debug, Clone, LibConfig)]
pub struct MySdkConfig {
    #[config(config_skip_getter)]
    pub(crate) api_token: Option<SecretString>,
}
```

#### Considerations

- Incompatible with: `skip_all`

### `builder_skip`

Flag used to skip the `Builder` struct setter method for this field.

A potential use-case is allowing an optional field to only be configured at runtime.

#### Example

```rust
# use lib_conf_derive as lib_conf;
use lib_conf::LibConfig;
use secrecy::SecretString;

#[derive(Debug, Clone, LibConfig)]
pub struct MySdkConfig {
    #[config(builder_skip)]
    pub(crate) api_token: Option<SecretString>,
}
```

#### Considerations

- Incompatible with: `skip_all`, `override_required` (implied by)

### `override_skip`

Flag used to prevent inclusion of this field in the generated `Override` struct.

Can be used to prevent runtime adjustment of a setting.

#### Example

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
pub struct MySdkConfig {
    #[config(override_skip)]
    pub(crate) client_version: Option<String>,
}
```

#### Considerations

- Incompatible with: `skip_all`, `override_required`, `override_from`, `override_via`,
  `override_attr`
- If your config struct is generic, any skipped fields will be substituted
  with `PhantomData<{type}>` fields in order to preserve generics in the `Builder`
  and `Override` structs
  - Therefore, this should probably not be used with generic fields where the
    concrete types are not public-facing

### `override_required`

By default, fields in the `Override` struct will always be wrapped in an `Option`, even if the
source type is fully required (non-`Option`, no `default` attr).

This is the default behaviour because the `Override` struct is an additional layer used to allow
overriding `Builder`-configured (or library-provided default) settings. It is by design that
integrating the `Override` struct is entirely optional. It's available to dependant crates that wish
to support runtime configuration of your library, and seamlessly absent for those that don't.

This attribute reverts this behaviour, enforcing presence of this field at runtime.

> Note: the following explanation uses a fake deriving struct name of `MySdkConfig` to improve legibility.

An undesirable consequence - the `MySdkOverrideConfig` struct loses it's `Default` impl as it now has a
required field.

- To account for this, the `MySdkConfig::builder()` and `MySdkConfigBuilder::new()` functions now require
  a `override_conf: MySdkOverrideConfig` parameter.
- Further, as it doesn't make sense to provide a `MySdkConfigBuilder::with_override()` method after having just
  initialized the builder with one, the `MySdkConfigBuilder::with_override` method is completely removed.

The *only* scenario where you should consider using this, is one where **all** of the following conditions
are met:
- The field stores a sensitive value and you wish to deter hard-coded initialization of the field.
- You recognize the introduction or removal of this attribute is a **guaranteed breaking change**.
- Your library dependant are already required to initialize some or all configuration at runtime.
  - This will typically only be true for internal/personal-use crates or for very large frameworks
    such as `Loco`.

#### Example

```rust
# use lib_conf_derive as lib_conf;
use lib_conf::LibConfig;
use secrecy::SecretString;

#[derive(Debug, Clone, LibConfig)]
pub struct MySdkConfig {
    #[config(override_required)]
    pub(crate) api_token: SecretString,
}
```

#### Considerations

- Implies: `builder_skip`
- Incompatible attributes: `default`, `skip_all`, `builder_skip`, `override_skip`
- Incompatible types: `Option<_>`
- Runtime configuration is now mandatory and is no longer opt-in
- The `Builder` struct loses it's `with_override()` method
- The `Builder` struct must be initialized with an `Override` config
- No setter is available for this field in the `Builder` struct

### `override_from`

Allows for specifying a custom type to be used for the field in the `Override` struct.

The mapped type must implement `Into` against the field's type used in the deriving struct.

This is most useful for types that lack or have undesired/incompatible `Deserialize` impl's.

#### Example

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
pub struct MySdkConfig {
    #[config(override_from = bool, copy, default = ModuleSupport::Automatic)]
    pub(crate) proxy_module_enabled: ModuleSupport,
}

#[derive(Debug, Clone, Copy)]
pub enum ModuleSupport {
    Automatic,
    Enabled,
    Disabled,
}
impl From<bool> for ModuleSupport {
    fn from(enable: bool) -> Self {
        if enable {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}
```

#### Considerations

- `Option<_>`-wrapping of the type in the `Override` struct is handled automatically (only
  applicable when `override_required` is not present)
- The `copy` attribute does not apply to mapped override types, but does still apply to the getter
  in the `Config` struct, and therefore should still be used
- Incompatible with: `skip_all`, `override_skip`
- As the mapped type is directly used in the `Override` struct, the type should be public

### `override_via`

To be used alongside `override_from`, this attribute allows for using some intermediate type to
handle the conversion between the `override_from` type and the `Config` field type.

This type must implement `From<OverrideFrom>` (where `OverrideFrom` is the type provided
by `override_from`) as well as `Into<T>` (where `T` is the field type defined in the
deriving `Config` struct.

This is most useful for types that lack or have undesired/incompatible `Deserialize` impl's.

#### Example

In this example, the `Override` struct stores the interval as a `u64`, allowing for runtime
configuration of the interval in seconds.

A custom adapter struct is used to handle the conversion, as `Duration` doesn't
implement `From<u64>`.

This adapter is only used internally during the `Override` merge, and therefore does not need to be
public.

```rust
# use lib_conf_derive as lib_conf;
use lib_conf::LibConfig;
use std::time::Duration;

#[derive(Debug, Clone, LibConfig)]
pub struct MySdkConfig {
    #[config(
        copy, default = Duration::from_secs(30),
        override_from = u64, override_via = SecondsAdapter,
    )]
    pub(crate) refresh_interval: Duration,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SecondsAdapter(pub u64);

impl From<u64> for SecondsAdapter {
    fn from(val: u64) -> Self {
        Self(val)
    }
}
impl From<SecondsAdapter> for Duration {
    fn from(adapter: SecondsAdapter) -> Self {
        Self::from_secs(adapter.0)
    }
}
```

#### Considerations

- Must be used with: [`override_from`](#override_from)
- Incompatible with: `skip_all`, `override_skip`

### `override_attr` <a name="override_attr_field"></a>

Passes through the contained attributes to the corresponding field in the generated `Override`
struct.

Can contain multiple attributes and can be specified multiple times.

#### Example

```rust
# #[cfg(feature = "serde")]
# {
# use lib_conf_derive as lib_conf;
use lib_conf::LibConfig;
use url::Url;

#[derive(Debug, Clone, LibConfig)]
pub struct MySdkConfig {
    #[config(override_attr(serde(alias = "db_url")))]
    pub(crate) database_url: Option<Url>,
}
# }
```

#### Considerations

- Incompatible with: `skip_all`, `override_skip`

## Getter return types

Return types of getter fn's for the user-provided `Config` struct and the `Override` struct are
decided as follows:

- Does the field have the [`#[config(copy)]`](#copy) attribute present?
  - Return type is **identical to the field's type**
  - **Note**: This is ignored for the `Override` struct if either [`override_from`](#override_from)
    or [`override_via`](#override_via) are present
    - Support for indicating `Copy`-support of mapped override types may be added in the future

- Is the field's type `Option<String>`?
  - Return type is **`Option<&str>`**
  - Handled by calling `as_ref().map(|x| x.as_str())` on the field

- Is the field's type `Option<_>`?
  - Return type is **`Option<&_>`**
  - Handled by calling `as_ref()` on the field

- Is the field's type `String`?
  - Return type is **`&str`**
  - Handled by calling `as_str()` on the field

- Otherwise:
  - Return type is a borrow, e.g. `&T` for a field with type `T`
