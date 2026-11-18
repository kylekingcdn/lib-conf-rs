# Non-workspace tests

Contained are integration "tests" used to:

- Validate compilation of derived items
- Greatly improve internal debugging experience
- Provide indication of regressions

## Layout

There are currently 2 crates contained:

- `nested-test-cases`
- `nested-expanded`

### `nested-test-cases`

Contains various `LibConfig`-derived config structs in nested combinations.

This crate provides flags to `lib-conf-derive` which instruct it to write it's expanded
output to files.
It also writes the source config struct without any of `lib-conf`'s attributes.

### `nested-expanded`

Contains the files that were expanded from `nested-test-cases`.

Additionally, some files are shared from `nested-test-cases` using symlinks to ensure
non-derived items required for compilation are available.

## Debugging

Because `nested-expanded` contains the post-expansion code from `nested-test-cases`, it is equivalent and will
report the same errors.

However, since the code is expanded, errors are no longer incredibly cryptic.

**Instead of receiving the following:**

```
error[E0308]: mismatched types
 --> src/nested.rs:4:10
  |
4 | #[derive(LibConfig, Debug, Clone)]
  |          ^^^^^^^^^
  |          |
  |          expected `WithDefaultConfigBuilder`, found `Option<WithDefaultConfigBuilder>`
  |          expected `nested::WithDefaultConfigBuilder` because of return type
  |
  = note: expected struct `WithDefaultConfigBuilder`
               found enum `std::option::Option<WithDefaultConfigBuilder>`
  = note: this error originates in the derive macro `LibConfig` (in Nightly builds, run with -Z macro-backtrace for more info)
```

**You'll get:**

```
error[E0308]: mismatched types
  --> src/expanded/with_default_config.rs:38:9
   |
37 |     pub fn builder() -> WithDefaultConfigBuilder {
   |                         ------------------------ expected `WithDefaultConfigBuilder` because of return type
38 |         Some(<WithDefaultConfigBuilder>::new())
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `WithDefaultConfigBuilder`, found `Option<WithDefaultConfigBuilder>`
   |
   = note: expected struct `WithDefaultConfigBuilder`
                found enum `std::option::Option<WithDefaultConfigBuilder>`
```

Furthermore, the expanded file can be directly investigated (and you'll benefit from LSP support)!

## Versioning

Contrary to common practice, the expanded/generated structs have intentionally been checked into version control.

This is done as it provides an additional layer of confidence that regressions aren't being introduced with changes.

Any unexpected changes will be evident when committing work.

## Workspace exclusion

The `nested-expanded` crate has been excluded from the workspace.

This prevents:
- Excessive noise introduced during development.
  - A single compilation error in the derive macro would otherwise result in 40+ duplicate errors.
- Cyclic checking by `rust-analyzer` or tools that watch for file changes (such as `bacon`), upon expansion
