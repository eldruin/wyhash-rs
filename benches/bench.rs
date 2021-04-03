#![cfg_attr(test, feature(test))]
pub extern crate test;
pub use std::hash::Hasher;
pub use std::collections::hash_map::DefaultHasher;

pub fn hasher_bench<H>(b: &mut test::Bencher, len: usize)
where
    H: Hasher + Default,
{
    let bytes: Vec<_> = (0..100).cycle().take(len).collect();
    b.bytes = bytes.len() as u64;
    b.iter(|| {
        let mut hasher: H = Default::default();
        hasher.write(&bytes);
        hasher.finish()
    });
}

pub fn hasher_bench_u64<H>(b: &mut test::Bencher)
where
    H: Hasher + Default,
{
    let int = b as *const _ as usize as u64;
    b.bytes = 8;
    b.iter(|| {
        let mut hasher: H = Default::default();
        hasher.write_u64(int);
        hasher.finish()
    });
}

macro_rules! impl_bench {
    ($mod_name: ident, $hasher: ty) => {
        mod $mod_name {
            use super::*;

            #[bench]
            fn hash_1_mega_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 1024 * 1024)
            }

            #[bench]
            fn hash_1_kilo_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 1024)
            }

            #[bench]
            fn hash_512_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 512)
            }

            #[bench]
            fn hash_256_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 256)
            }

            #[bench]
            fn hash_128_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 128)
            }

            #[bench]
            fn hash_032_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 32)
            }

            #[bench]
            fn hash_016_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 16)
            }

            #[bench]
            fn hash_012_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 12)
            }

            #[bench]
            fn hash_009_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 9)
            }

            #[bench]
            fn hash_008_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 8)
            }

            #[bench]
            fn hash_007_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 7)
            }

            #[bench]
            fn hash_004_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 4)
            }

            #[bench]
            fn hash_u64(b: &mut test::Bencher) {
                hasher_bench_u64::<$hasher>(b)
            }
        }
    };
}

impl_bench!(std_hash_map, DefaultHasher);
impl_bench!(metro64, metrohash::MetroHash64);
impl_bench!(metro128, metrohash::MetroHash128);
impl_bench!(fnvh, fnv::FnvHasher);
impl_bench!(xxh, twox_hash::XxHash);

impl_bench!(wyhash_bench, wyhash::WyHash);
