// Example using the `core::hash::Hasher` interface.
// (Same as using `std::hash::Hasher`)

extern crate core;
extern crate wyhash;
use core::hash::Hasher;
use wyhash::WyHash;

fn main() {
    let mut hasher = WyHash::with_seed(3);
    hasher.write(&[0, 1, 2]);
    assert_eq!(0xff72_c1dd_91f7_f9b7, hasher.finish());
}
