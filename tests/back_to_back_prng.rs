extern crate wyhash;
use wyhash::wyrng;

#[test]
fn wyrng_test() {
    for (i, original) in ORIGINAL_PRNG.iter().enumerate() {
        assert_eq!(*original, wyrng(i as u64));
    }
}

// Results from the cannonical C implementation
#[allow(clippy::unreadable_literal)]
const ORIGINAL_PRNG: [u64; 10] = [
    0xb01eebcf5b9ef14f,
    0x9742cbb78a1b31eb,
    0x8f1b664b0136eec,
    0x692cac55f5878c41,
    0xcb5b171aa3803d4d,
    0x218f78c2d23c7ded,
    0x833fe3b79834aae9,
    0xe06ad960dda8c866,
    0x459944558ba1796b,
    0xb8cda41e3a5db986,
];
