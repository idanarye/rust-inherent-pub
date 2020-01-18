# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- Upgrade `syn` and `quote` versions to support newer syntax.
- Make `async` methods work. Rust does not support `async` methods in traits
  yet, but they can be supported by crates like
  [`async-trait`](https://crates.io/crates/async-trait)
- [**POTENTIALLY BREAKING**] The `impl Type` block no longer copies the
  attributes of the `impl Trait for Type` block.

## 0.2.0 - 2018-11-01

### Added
- Make generated methods `#[inline(always)]`.

## 0.1.0 - 2018-10-31

### Added
- The `proc_macro` attribute `#[inherent_pub]`.
