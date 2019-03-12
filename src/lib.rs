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

#![no_std]
#![deny(missing_docs, unsafe_code)]

mod wyhash;
pub use wyhash::wyhash;

#[cfg(feature = "std")]
mod std_impls;
#[cfg(feature = "std")]
pub use std_impls::WyHash;
