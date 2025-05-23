# wyhash fast portable non-cryptographic hashing algorithm

[![crates.io](https://img.shields.io/crates/v/wyhash.svg)](https://crates.io/crates/wyhash)
[![Docs](https://docs.rs/wyhash/badge.svg)](https://docs.rs/wyhash)
[![Build Status](https://github.com/eldruin/wyhash-rs/workflows/Build/badge.svg)](https://github.com/eldruin/wyhash-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/wyhash-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/wyhash-rs?branch=master)

Rust implementation of the [wyhash algorithm by Wang Yi][original].

The hashing algorithm passes SMHasher and the random number generator passes BigCrush and practrand.
As of now it is the fastest algorithm in the SMHasher benchmark (faster than t1ha and XXH3).
See [here][original].

Furthermore, this algorithm is solid, simple, portable (does not need hardware support, can be
used in `no_std` environments) and has no dependencies (except the traits from `rand_core`).

The generated hashes are equal (see tests) as of the version stated [here][original-version]
although the speed varies ([PRs are welcome][issue-tracker]).

## Crate features

By default this crate uses 128-bit integer multiplications.
To restrict that to 64 bits you can enable the feature `mum32bit`. This offers better
performance on 32-bit architectures.
Beware that this feature produces different results.

## Usage

This crate provides free functions as well as implementations of the `Hasher`, `BuildHasher`, `Rng` and
`SeedableRng` traits.

```rust
use core::hash::Hasher;
use wyhash::WyHash;

fn main() {
  let mut hasher = WyHash::with_seed(3);
  hasher.write(&[0, 1, 2]);
  assert_eq!(0xcc24_2106_e707_6a48, hasher.finish());
}
```

See further examples of the hasher and RNG in the documentation.


## Performance comparison

A basic performance comparison benchmark against [`fnv`], [`twox-hash`], [`metrohash`]
and the standard hash_map hash [`collections::hash_map::DefaultHasher`][hash-map-hash]
is included in the sources and you can run it with nightly Rust:

```
cargo +nightly bench
```

[`fnv`]: https://crates.io/crates/fnv
[`twox-hash`]: https://crates.io/crates/twox-hash
[`metrohash`]: https://crates.io/crates/metrohash
[hash-map-hash]: https://doc.rust-lang.org/std/collections/hash_map/struct.DefaultHasher.html

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project][issue-tracker].

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.36.0 and up. It *might*
compile with older versions but that may change in any new patch release.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[issue-tracker]: https://github.com/eldruin/wyhash-rs/issues
[original]: https://github.com/wangyi-fudan/wyhash
[original-version]: https://github.com/eldruin/wyhash-rs/blob/master/comparison/original/CMakeLists.txt
