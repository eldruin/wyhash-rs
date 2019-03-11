# Rust implementation of the wyhash fast non-cryptographic hashing algorithm

[![crates.io](https://img.shields.io/crates/v/wyhash.svg)](https://crates.io/crates/wyhash)
[![Docs](https://docs.rs/wyhash/badge.svg)](https://docs.rs/wyhash)
[![Build Status](https://travis-ci.org/eldruin/wyhash-rs.svg?branch=master)](https://travis-ci.org/eldruin/wyhash-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/wyhash-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/wyhash-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a Rust implementation of the [wyhash algorithm by Wang Yi][1].
The generated hashes are equal (see tests).

[1]: https://github.com/wangyi-fudan/wyhash

### Usage

`no_std` mode is activated by setting `default-features = false;`

```rust
extern crate wyhash;
use wyhash::WyHash;
use std::hash::Hasher;

fn main() {    
  let mut hasher = WyHash::with_seed(1);
  hasher.write(&[0]);
  assert_eq!(0xcb4b8ebdf7240e2c, hasher.finish());
}
```

## Status

- [x] Hashing function
- [ ] PRNG

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/wyhash-rs/issues).

## License

Licensed under

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be licensed as above, without any additional terms or conditions.

