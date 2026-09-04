# `lib-conf`

Rust proc-macro crate providing library authors with first-class configuration for their end-users

## Overview

<div class="warning">
  <b>This crate is currently a work in progress.</b>
  <br/><br/>
  Until stabilized, usage in production code is <b>strongly discouraged</b>.
</div>

[**crates.io**](https://crates.io/crates/lib-conf)
|
[**Docs**](https://docs.rs/lib-conf/latest)
|
[**GitHub**](https://github.com/kylekingcdn/lib-conf-rs)

## Progress

### Todo (initial release)

- [x] Handle required fields in `new()` + `builder()` fns, strip `Default` impl
- [ ] Take override in as param if `override_required` is set on any fields,
  strip `with_override`fn
  - [x] `override_required`should imply `builder_skip`
  - [x] `override_required` should not be allowed with default or Option types
- [x] Implement override type mapping for `override_from` and `override_via`
- [ ] Return non-copy `Option<_>` getters using `as_ref()`
- [ ] Return `String`/`Option<String>` getters fns using `as_str()` to
  avoid `&String` return types.

### Roadmap

- Add default value getters to Config struct
- Generate .env.example util fn for consumers, using defaults + required
- Add/confirm offical support for nested usage
  - e.g. `lib1 -> lib2 -> bin`, where both libs use `lib-conf` and both
    are exposed to binary
  - Support lib2 exposing a subset of lib1 options to bin
