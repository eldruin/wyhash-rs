//! wyhash is a fast non-cryptographic hash by Wang Yi.
//!
//! This is an implementation of the [original algorithm][1] in Rust.
//! The generated hashes are equal (see tests).
//!
//! [1]: https://github.com/wangyi-fudan/wyhash
//!

#![no_std]
#![deny(missing_docs, unsafe_code)]

mod wyhash;
pub use wyhash::wyhash;

#[cfg(feature = "std")]
mod std_impls;
#[cfg(feature = "std")]
pub use std_impls::WyHash;
