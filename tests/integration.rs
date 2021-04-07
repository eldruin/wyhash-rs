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
    #[cfg(not(feature = "mum32bit"))]
    let expected = 0x22a2_d5db_3856_770f;
    #[cfg(feature = "mum32bit")]
    let expected = 0x6a2c_d506_62d4_cdf1;

    let mut hasher = final3::WyHash::default();
    hasher.write(&[0]);
    assert_eq!(expected, hasher.finish());
}
