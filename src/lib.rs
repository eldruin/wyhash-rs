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
//! [original-version]: https://github.com/eldruin/wyhash-rs/blob/master/comparison/original/CMakeLists.txt
//!
//! ## Crate features
//!
//! By default this crate uses 128-bit integer multiplications.
//! To restrict that to 64 bits you can enable the feature `mum32bit`. This offers better
//! performance on 32-bit architectures.
//! Beware that this feature produces different the results.
//!
//! ## Usage (see also examples folder)
//!
//! For the hashing function you can use either the free function or the
//! `Hasher` trait.
//!
//! ### `wyhash` function usage
//!
//! ```
//! use wyhash::{make_secret, wyhash};
//!
//! let data = [0, 1, 2];
//! let seed = 3;
//! let hash = wyhash(&data, seed, make_secret(seed));
//!
//! println!("{:x}", hash); // prints ebd49a0ad01078ce
//! ```
//!
//! ### `Hasher` trait usage
//!
//! You can also use `std::hash::Hasher`, it is the same.
//!
//! ```
//! use core::hash::Hasher;
//! use wyhash::{make_secret, WyHash};
//!
//! let secret = make_secret(3);
//! let mut hasher = WyHash::new(4, secret);
//! hasher.write(&[0, 1, 2]);
//!
//! println!("{:x}", hasher.finish()); // prints 830142a244045ff4
//! ```
//!
//! ### `wyrng` function usage
//!
//! Note that the seed parameter is updated so that it is possible to
//! generate a sequence of random numbers.
//!
//! ```
//! use wyhash::wyrng;
//!
//! let mut seed = 3;
//! let random_number = wyrng(&mut seed);
//!
//! println!("{:x}", random_number); // prints 3e99a772750dcbe
//! println!("{:x}", seed); //prints a0761d6478bd6432
//! ```
//!
//! ### `RngCore` trait usage
//!
//! You can also use `rand::Rng`, it is the same.
//!
//! ```
//! use rand_core::RngCore;
//! use wyhash::WyRng;
//!
//! let mut rng = WyRng::default();
//! println!("{:x}", rng.next_u64()); // prints 111cb3a78f59a58e
//! ```
//!
//! ### `SeedableRng` trait usage
//!
//! You can also use `rand::SeedableRng`, it is the same.
//!
//! ```
//! use rand_core::{RngCore, SeedableRng};
//! use wyhash::WyRng;
//!
//! // Seeds are 8-byte long.
//! let seed = [0, 1, 2, 3, 4, 5, 6, 7];
//! let mut rng1 = WyRng::from_seed(seed);
//! println!("{:x}", rng1.next_u64()); // prints d730135774c6ae31
//!
//! // Alternatively you can also use this convenience method:
//! let mut rng2 = WyRng::seed_from_u64(3);
//! println!("{:x}", rng2.next_u64()); // prints 3e99a772750dcbe
//! ```

#![doc(html_root_url = "https://docs.rs/wyhash/0.5.0")]
#![no_std]
#![deny(missing_docs, unsafe_code)]

/// WyHash version 1
pub mod v1;

/// WyHash version final 3
pub mod final3;
pub use crate::final3::{make_secret, wyhash, wyrng};
pub use crate::final3::{WyHash, WyRng, WyHasherBuilder};
