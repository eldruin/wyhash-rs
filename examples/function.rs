// Example of using the free functions.

extern crate wyhash;
use wyhash::{wyhash, wyrng};

fn main() {
    assert_eq!(0xff72c1dd91f7f9b7, wyhash(&[0, 1, 2], 3));
    assert_eq!(0xd603133dc4196d3, wyrng(1));
}
