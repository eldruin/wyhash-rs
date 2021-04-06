extern crate core;
extern crate wyhash;
use core::hash::Hasher;
use wyhash::v1::WyHash as WyHashV1;

#[test]
fn default_constructed() {
    let mut hasher = WyHashV1::default();
    hasher.write(&[0]);
    assert_eq!(0x8c73_a8ab_4659_6ae4, hasher.finish());
}
