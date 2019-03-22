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
    0x111cb3a78f59a58e,
    0xcdef1695e1f8ed2c,
    0xa4eed0248024f5f6,
    0x3e99a772750dcbe,
    0xfae94589c79d2703,
    0xac19123cacd229cc,
    0xb18dc4f431e3006,
    0xe21b87e1e24a18c1,
    0x591b413082f6638b,
    0x35d5241efb19a892,
];
