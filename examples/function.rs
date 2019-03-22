// Example of using the free functions.

extern crate wyhash;
use wyhash::{wyhash, wyrng};

fn main() {
    assert_eq!(0xb0f9_4152_0b1a_d95d, wyhash(&[0, 1, 2], 3));
    assert_eq!(0xcdef_1695_e1f8_ed2c, wyrng(1));
}
