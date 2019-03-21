extern crate core;
extern crate wyhash;
use core::hash::Hasher;
use wyhash::WyHash;

#[test]
fn default_constructed() {
    let mut hasher = WyHash::default();
    hasher.write(&[0]);
    assert_eq!(0x9c4c_36b1_3662_61d1, hasher.finish());
}
