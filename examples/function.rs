// Example of using the free functions.

extern crate wyhash;
use wyhash::{wyhash, wyrng};

fn main() {
    assert_eq!(0xff72_c1dd_91f7_f9b7, wyhash(&[0, 1, 2], 3));
    assert_eq!(0xd60_3133_dc41_96d3, wyrng(1));
}
