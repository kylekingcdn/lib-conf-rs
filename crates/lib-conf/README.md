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

### Roadmap

- Add default value getters to Config struct
- Generate .env.example util fn for consumers, using defaults + required
- Add/confirm offical support for nested usage
  - e.g. `lib1 -> lib2 -> bin`, where both libs use `lib-conf` and both
    are exposed to binary
  - Support lib2 exposing a subset of lib1 options to bin
