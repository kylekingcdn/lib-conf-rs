# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/kylekingcdn/lib-conf-rs/compare/lib-conf-v0.4.0...lib-conf-v0.4.1) - 2026-10-17

### Fix

- replaced incorrect builder link

## [0.4.0](https://github.com/kylekingcdn/lib-conf-rs/compare/lib-conf-v0.3.2...lib-conf-v0.4.0) - 2026-10-16

### Build

- explicit enable public api quality warnings

### Chore

- add doctest scaffolding
- update parse error types
- add scope-agnostic attr types
- add dedicated struct/field attr modules
- add shared IdentPool
- add doc generation utils

### Doc

- fix readme indentation
- add missing lib docs
- update docs for new attributes

### Feat

- passthrough cfg, doc(cfg) attrs
- integrate unified attr parsing
- extend support for generated docs

### Fix

- example derive Clone
- no-serde conditionals
- add req’d attrs to examples
- clippy warnings
- drop item vis
- add newly req’d attrs to examples, tests
- handle parsing doc(cfg) attrs
