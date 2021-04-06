// Example using the `core::hash::Hasher` interface.
// (Same as using `std::hash::Hasher`)

use core::hash::Hasher;
use wyhash::{make_secret, WyHash};

fn main() {
    let secret = make_secret(3);
    let mut hasher = WyHash::new(4, secret);
    hasher.write(&[0, 1, 2]);
    assert_eq!(0x8301_42a2_4404_5ff4, hasher.finish());
}
