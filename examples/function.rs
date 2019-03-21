// Example of using the free functions.

extern crate wyhash;
use wyhash::{wyhash, wyrng};

fn main() {
    assert_eq!(0xcc24_2106_e707_6a48, wyhash(&[0, 1, 2], 3));
    assert_eq!(0x692c_ac55_f587_8c41, wyrng(1));
}
