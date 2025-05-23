#[cfg(not(feature = "mum32bit"))]
use comparison::ffi::c_wyrng_final3;
#[cfg(feature = "mum32bit")]
use comparison::ffi::c_wyrng_final3_32bit_mum;
use comparison::ffi::c_wyrng_v1;
use rand::{RngCore, SeedableRng, TryRngCore};
use wyhash::final3::wyrng as wyrng_final3;
use wyhash::final3::WyRng as WyRngFinal3;
use wyhash::v1::wyrng as wyrng_v1;
use wyhash::v1::WyRng as WyRngV1;

macro_rules! impl_tests {
    ($mod_name: ident, $hasher: ty, $function: ident, $c_function: ident) => {
        mod $mod_name {
            use super::*;
            #[test]
            fn wyrng_test() {
                for i in 0..10 {
                    let mut seed = i as u64;
                    let original = unsafe { $c_function(&mut seed) };

                    let mut seed = i as u64;
                    assert_eq!(original, $function(&mut seed));
                }
            }

            #[test]
            fn rngcore_trait_next_64() {
                let mut rng = <$hasher>::default();
                let mut seed = 0;
                for _ in 0..10 {
                    let original = unsafe { $c_function(&mut seed) };
                    assert_eq!(original, rng.next_u64());
                }
            }

            #[test]
            fn rngcore_trait_next_32() {
                let mut rng = <$hasher>::default();
                let mut seed = 0;
                for _ in 0..10 {
                    let original = unsafe { $c_function(&mut seed) };
                    assert_eq!(original as u32, rng.next_u32());
                }
            }

            #[test]
            fn seedablerng_trait() {
                for i in 0..10 {
                    let seed = [i as u8, 0, 0, 0, 0, 0, 0, 0];
                    let mut rng = <$hasher>::from_seed(seed);

                    let mut seed = i as u64;
                    let original = unsafe { $c_function(&mut seed) };
                    assert_eq!(original, rng.next_u64());
                }
            }

            #[test]
            fn seedablerng_trait_seed_from_u64() {
                for i in 0..10 {
                    let mut seed = i as u64;
                    let original = unsafe { $c_function(&mut seed) };

                    let mut rng = <$hasher>::seed_from_u64(i as u64);
                    assert_eq!(original, rng.next_u64());
                }
            }

            fn read64_le(data: &[u8]) -> [u64; 10] {
                let mut packed = [0; 10];
                for (i, chunk) in data.chunks(8).enumerate() {
                    for (j, d) in chunk.iter().enumerate() {
                        packed[i] |= u64::from(*d) << (j * 8);
                    }
                }
                packed
            }

            fn check_prng_seq(data: &[u8]) {
                let packed = read64_le(&data);
                let mut seed = 0;
                for current in packed.iter() {
                    let original = unsafe { $c_function(&mut seed) };
                    assert_eq!(original, *current);
                }
            }

            #[test]
            fn rngcore_trait_fill_bytes() {
                let mut rng = <$hasher>::default();
                let mut data = [0; 80];
                rng.fill_bytes(&mut data);

                check_prng_seq(&data);
            }

            #[test]
            fn rngcore_trait_try_fill_bytes() {
                let mut rng = <$hasher>::default();
                let mut data = [0; 80];
                rng.try_fill_bytes(&mut data).expect("Failed to fill bytes");

                check_prng_seq(&data);
            }
        }
    };
}

impl_tests!(v1, WyRngV1, wyrng_v1, c_wyrng_v1);

#[cfg(not(feature = "mum32bit"))]
impl_tests!(final3, WyRngFinal3, wyrng_final3, c_wyrng_final3);

#[cfg(feature = "mum32bit")]
impl_tests!(final3, WyRngFinal3, wyrng_final3, c_wyrng_final3_32bit_mum);
