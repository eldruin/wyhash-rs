# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

...

## [0.2.1] - 2019-03-30
### Added
- `rand_core::RngCore` and `rand_core::SeedableRng` trait implementations
  for the random number generator.
- MIT license

## [0.2.0] - 2019-03-23
### Added
- Added random number generation function.
- Added C++ program using the upstream library to generate the results used
  in the tests.

### Changed
- The standard library is not necessary any more. The hasher trait implemented
  now is `core::hash::Hasher`, which is equivalent to `std::hash::Hasher`.
  The code should continue to work without change but deactivating
  the default features for `no_std` compatibility is not necessary any more.

- The generated hashes have changed following the upstream project.

## 0.1.0 - 2019-03-11

This is the initial release to crates.io.

[Unreleased]: https://github.com/eldruin/wyhash-rs/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/eldruin/wyhash-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/eldruin/wyhash-rs/compare/v0.1.0...v0.2.0
