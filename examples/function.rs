// Example of using the free functions.

extern crate wyhash;
use wyhash::{wyhash, wyrng};

fn main() {
    assert_eq!(0xcb4b8ebdf7240e2c, wyhash(&[0], 1));
    assert_eq!(0xd603133dc4196d3, wyrng(1));
}
