//! Rust implementation of the [wyhash algorithm by Wang Yi][original].
//!
//! The hashing algorithm passes SMHasher and the random number generator
//! passes BigCrush and practrand.
//! As of now it is the fastest algorithm in the SMHasher benchmark
//! (faster than t1ha and XXH3).
//! See [here][original].
//!
//! Furthermore, this algorithm is solid, simple, portable (does not need
//! hardware support, can be used in `no_std` environments) and has
//! no dependencies.
//!
//! The generated hashes are equal (see tests) as of the version stated
//! [here][original-version] although the speed varies
//! ([PRs are welcome][issue-tracker]).
//!
//! [issue-tracker]: https://github.com/eldruin/wyhash-rs/issues
//! [original]: https://github.com/wangyi-fudan/wyhash
//! [original-version]: https://github.com/eldruin/wyhash-rs/blob/master/original/CMakeLists.txt
//!
//! ## Usage (see also examples folder)
//!
//! For the hashing function you can use either the free function or the
//! `Hasher` trait.
//!
//! ### `wyhash` function usage
//!
//! ```
//! extern crate wyhash;
//! use wyhash::wyhash;
//!
//! fn main() {
//!     let data = [0, 1, 2];
//!     let seed = 3;
//!     let hash = wyhash(&data, seed);
//!
//!     assert_eq!(0xb0f9_4152_0b1a_d95d, hash);
//! }
//! ```
//!
//! ### `Hasher` trait usage
//!
//! You can also use `std::hash::Hasher`, it is the same.
//!
//! ```
//! extern crate core;
//! extern crate wyhash;
//! use core::hash::Hasher;
//! use wyhash::WyHash;
//!
//! fn main() {
//!     let mut hasher = WyHash::with_seed(3);
//!     hasher.write(&[0, 1, 2]);
//!
//!     assert_eq!(0xb0f9_4152_0b1a_d95d, hasher.finish());
//! }
//! ```
//!
//! ### `wyrng` function usage
//!
//! ```
//! extern crate wyhash;
//! use wyhash::wyrng;
//!
//! fn main() {
//!     let seed = 3;
//!     let random_number = wyrng(seed);
//!
//!     assert_eq!(0x3e9_9a77_2750_dcbe, random_number);
//! }
//! ```

#![no_std]
#![deny(missing_docs, unsafe_code)]

mod wyhash;
pub use wyhash::{wyhash, wyrng};

mod trait_impls;
pub use trait_impls::WyHash;
