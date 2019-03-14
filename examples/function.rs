// Example of using the free function.

extern crate wyhash;
use wyhash::wyhash;

fn main() {
    assert_eq!(0xcb4b8ebdf7240e2c, wyhash(&[0], 1));
}
