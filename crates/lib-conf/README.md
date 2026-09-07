# `lib-conf`

Rust proc-macro crate providing library authors with first-class configuration for their end-users

## Overview

Included is a `LibConfig` derive macro to be used on named structs containing fields used to publicly configure your library.

This derive macro generates a builder struct, an override struct (for runtime configuration), and helper methods.

### Features

- Provides ready-to-use runtime configuration of your library for dependant binaries
- Automatic builder generation
- Runtime revert to library defined default
- Inline evaluation of library defined defaults
    - Default expression gets copied into docs of all associated fn's, preventing doc drift
- Runtime-only initialization of fields, deterring hard-coding of sensitive values
- Supports `serde`-optional libraries
- Full support for structs utilizing generics

### Resources

For more details, the following resources are provided:

- [Feature flags](#feature-flags)
- [Attribute reference](#attribute-reference)
- [Examples](https://github.com/kylekingcdn/lib-conf-rs/tree/main/examples)
- [Explanation of generated code](#generation)
- [Roadmap](#roadmap)

[**crates.io**](https://crates.io/crates/lib-conf)
|
[**Docs**](https://docs.rs/lib-conf/latest)
|
[**GitHub**](https://github.com/kylekingcdn/lib-conf-rs)

## Feature flags

### `default`

Only the `derive` feature is enabled by default.

### `standard`

Enables what we believe to be the most common set of desired features.

This is provided as a feature group to allow for automatic opt-in of new, common use-case features introduced in future releases.

Currently includes: `derive`, `serde`

### `derive`

If disabled, this crate provides nothing.

This feature exists to allow for simple exclusion if other macro crates are also feature gated.

### `serde`

Enables automatically deriving `Deserialize` on the `Override`struct.

This will also drop the visibility of `Override` fields from `pub` to `pub(crate)`, deterring manual, non-runtime initialization of the `Override` struct.

If `serde` is a mandatory dependency of your library, you should always have this feature enabled.

Otherwise, if `serde` is optional, you can include this feature in your `serde` feature gate to allow for full support in dependant crates that use `serde`, while retaining compatibility with crates which do not want to depend on `serde`.

### `syn-debug`

Enables the `extra-traits` feature of `syn`.

This is almost certainly only useful for internal development and likely shouldn't be enabled.

## Attribute reference

Attribute documentation can be found on [docs.rs](https://docs.rs/lib-conf-derive/latest/lib_conf_derive/derive.LibConfig.html) or in the [following markdown file](https://github.com/kylekingcdn/lib-conf-rs/blob/main/crates/lib-conf-derive/doc/attributes.md).

## Examples

Please see the [examples](https://github.com/kylekingcdn/lib-conf-rs/tree/main/example) directory and accompanying `README.md` file for real-world usage examples.

## Generation

Deriving `LibConfig` on a '`MyLibraryConfig`' named struct will generate:

**A settings builder struct:** `MyLibraryConfigBuilder`

- Provides consumers with simple baseline configuration at compile time

**An override settings struct:** `MyLibraryOverrideConfig`

- Provides consumers with a `serde::Deserialize` variant of the settings struct, allowing for out-of-the-box runtime adjustment of settings
  - designed to fit the configuration pattern already in-use by the binary. E.g. using `config` to load settings from various files or env variables, or using `dotenvy` to pull from a `.env` file

**impl's on the *derive* struct**: `MyLibraryConfig`

- `builder()` fn
- Getter methods
- Merge method to handle absorbing fields present in a `MyLibraryOverrideConfig`
- `Default` impl, if all fields are optional (`Option<_>` type or `default = ..` attr present)
- `Add<MyLibraryOverrideConfig>` impl, which calls the merge fn

### Benefits of builder/override pattern

- Provide's out-of-the-box runtime configuration, bypassing the common need for binary-authors to manually create settings fields/structs used to initialize your library
- Supports adjusting settings at runtime that were not configured using the builder
- Allows for overriding settings at runtime that were configured using the builder
- Supports reverting settings at runtime that were configured using the builder - back to library-defined defaults
- Allows for mapping values from deserialize-supported types, without relying on `serde_as`
- Retains compatibility in serde-optional libraries via feature flags
- Allows for gating fields to runtime-only initialization, which can greatly deter hard-coding of sensitive values (e.g. API tokens, database credentials, etc.)

## Progress

### Roadmap

- Add default value getters to Config struct
- Generate .env.example util fn for consumers, using defaults + required
- Add/confirm offical support for nested usage
  - e.g. `lib1 -> lib2 -> bin`, where both libs use `lib-conf` and both
    are exposed to binary
  - Support lib2 exposing a subset of lib1 options to bin
