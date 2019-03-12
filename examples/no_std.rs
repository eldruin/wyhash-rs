// Example of usage without needing the standard library.
// (You will still need to add #![no_std], a panic handler and so on.)
extern crate wyhash;
use wyhash::wyhash;

fn main() {
    assert_eq!(0xcb4b8ebdf7240e2c, wyhash(&[0], 1));
}
