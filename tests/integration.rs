extern crate core;
extern crate wyhash;
use core::hash::Hasher;
use wyhash::{final3, v1};

#[test]
fn v1_default_constructed() {
    let mut hasher = v1::WyHash::default();
    hasher.write(&[0]);
    assert_eq!(0x8c73_a8ab_4659_6ae4, hasher.finish());
}

#[test]
fn final3_default_constructed() {
    let mut hasher = final3::WyHash::default();
    hasher.write(&[0]);
    assert_eq!(0x22a2_d5db_3856_770f, hasher.finish());
}
