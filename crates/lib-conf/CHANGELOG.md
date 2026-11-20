# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0-rc.1](https://github.com/kylekingcdn/lib-conf-rs/compare/lib-conf-v0.4.2...lib-conf-v0.5.0-rc.1) - 2026-11-20

### Chore

- add no_autoderive test
- fix serde flags
- no default dep features, fix dev deps
- fix typos
- improve expansion
- lint fixes
- add copy/default to builder plan
- update deps
- drop field attrs field vis
- direct StrExpr -> String helper
- wrap field ident in Rc
- add path support to IdentPool

### Doc

- remove optional example field name
- fix broken link
- wip attributes update
- fix typos
- fmt/typos
- expr handling fixed

### Feat

- support for custom dep crate paths
- config_docs attr macro
- nested support
- add no_autoderive attr
- auto-derive copy
- add assign impl for Config += Override
- preliminary support for validation

### Fix

- no-serde lints
- .env ignore paths
- re-enable test attr
- derives use plan
- default/copy derive on empty config
- improve invalid attr shape error

## [0.4.2](https://github.com/kylekingcdn/lib-conf-rs/compare/lib-conf-v0.4.1...lib-conf-v0.4.2) - 2026-10-21

### Chore

- add collapsed doc get utils

### Doc

- improve unset docs, add  collapsed aliases
- fix broken unset field docs

### Fix

- no unset fields for skip_builder/override
- add missing must_use attrs

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
