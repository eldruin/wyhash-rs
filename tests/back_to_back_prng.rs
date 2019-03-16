extern crate wyhash;
use wyhash::wyrng;

#[test]
fn wyrng_test() {
    for (i, original) in ORIGINAL_PRNG.iter().enumerate() {
        println!("{}", i);
        assert_eq!(*original, wyrng(i as u64));
    }
}

// Results from the cannonical C implementation
#[allow(clippy::unreadable_literal)]
const ORIGINAL_PRNG: [u64; 10] = [
    0x5c71580fe1214a64,
    0xd603133dc4196d3,
    0x4931d0d4e732891c,
    0xaa4b1097deadb2f7,
    0x4aaab346ed6ff0e6,
    0x68078de9e6864ad6,
    0xff654be3c14d314e,
    0x41f29d7852a02193,
    0xdadd332cc6f6d810,
    0x1214a022c7ff269,
];
