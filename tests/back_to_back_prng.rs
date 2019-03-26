extern crate wyhash;
use wyhash::{wyrng, WyRng, WyRngSeed};
extern crate rand_core;
use rand_core::{RngCore, SeedableRng};

#[test]
fn wyrng_test() {
    for (i, original) in ORIGINAL_PRNG.iter().enumerate() {
        assert_eq!(*original, wyrng(i as u64));
    }
}

#[test]
fn rngcore_trait_next_64() {
    let mut rng = WyRng::default();
    for original in ORIGINAL_PRNG_SEQ.iter() {
        assert_eq!(*original, rng.next_u64());
    }
}

#[test]
fn rngcore_trait_next_32() {
    let mut rng = WyRng::default();
    for original in ORIGINAL_PRNG_SEQ.iter() {
        assert_eq!((*original) as u32, rng.next_u32());
    }
}

#[test]
fn seedablerng_trait() {
    for (i, original) in ORIGINAL_PRNG.iter().enumerate() {
        let seed = WyRngSeed([i as u8, 0, 0, 0, 0, 0, 0, 0]);
        let mut rng = WyRng::from_seed(seed);
        assert_eq!(*original, rng.next_u64());
    }
}

fn read64_le(data: &[u8]) -> [u64; 10] {
    let mut packed = [0; 10];
    for (i, d) in data.chunks(8).enumerate() {
        for di in 0..d.len() {
            packed[i] |= u64::from(d[di]) << (di * 8);
        }
    }
    packed
}

fn check_prng_seq(data: &[u8]) {
    let packed = read64_le(&data);
    for (original, current) in ORIGINAL_PRNG_SEQ.iter().zip(&packed) {
        assert_eq!(*original, *current);
    }
}

#[test]
fn rngcore_trait_fill_bytes() {
    let mut rng = WyRng::default();
    let mut data = [0; 80];
    rng.fill_bytes(&mut data);

    check_prng_seq(&data);
}

#[test]
fn rngcore_trait_try_fill_bytes() {
    let mut rng = WyRng::default();
    let mut data = [0; 80];
    rng.try_fill_bytes(&mut data).expect("Failed to fill bytes");

    check_prng_seq(&data);
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

// Results from the cannonical C implementation
#[allow(clippy::unreadable_literal)]
const ORIGINAL_PRNG_SEQ: [u64; 10] = [
    0x111cb3a78f59a58e,
    0x570ed851fce1ba5c,
    0xf13773a78a159211,
    0x1c7fbc880de76e44,
    0xcb1f7e8e8a235065,
    0x48cf825a117c0c17,
    0x23f75cf7415a09ae,
    0x6b0740c4d9d8d7a7,
    0xf3dfb6f3def27a27,
    0x9a31260b4da7de1f,
];
