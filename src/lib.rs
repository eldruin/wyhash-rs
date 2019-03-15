//! This is a Rust implementation of the [wyhash algorithm by Wang Yi][original].
//! The generated hashes are equal (see tests) although the speed varies
//! ([PRs are welcome][issue-tracker]).
//!
//! The algorithm passes the SMHasher, BigCrush and practrand and as of now it
//! is the fastest algorithm in the SMHasher benchmark (faster than t1ha).
//! See [here][original].
//!
//! Furthermore, this algorithm is portable (does not need hardware support),
//! simple and has no dependencies.
//!
//! [issue-tracker]: https://github.com/eldruin/wyhash-rs/issues
//! [original]: https://github.com/wangyi-fudan/wyhash
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
//!     assert_eq!(0xff72c1dd91f7f9b7, hash);
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
//!     assert_eq!(0xff72c1dd91f7f9b7, hasher.finish());
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
//!     assert_eq!(0xaa4b1097deadb2f7, random_number);
//! }
//! ```

#![no_std]
#![deny(missing_docs, unsafe_code)]

mod wyhash;
pub use wyhash::{wyhash, wyrng};

mod trait_impls;
pub use trait_impls::WyHash;
