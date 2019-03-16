extern crate core;
extern crate wyhash;
use core::hash::Hasher;
use wyhash::WyHash;

#[test]
fn default_constructed() {
    let mut hasher = WyHash::default();
    hasher.write(&[0]);
    assert_eq!(0x5e7b_e380_f6a1_3709, hasher.finish());
}

