use comparison::ffi::{c_wyhash_final3, c_wyhash_v1};
use core::hash::Hasher;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use fnv::FnvHasher;
use metrohash::{MetroHash128, MetroHash64};
use rand::distributions::Standard;
use rand::prelude::*;
pub use std::collections::hash_map::DefaultHasher;
use twox_hash::XxHash;
use wyhash::{final3, v1};

pub fn hasher_bench<H>(data: &[u8])
where
    H: Hasher + Default,
{
    let mut hasher: H = Default::default();
    hasher.write(data);
    let _ = hasher.finish();
}

fn benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let data: Vec<Vec<u8>> = [0, 4, 7, 8, 9, 12, 16, 32, 64, 128, 512, 1024, 1024 * 1024]
        .iter()
        .map(|&l| (&mut rng).sample_iter(Standard).take(l).collect())
        .collect();

    let secret = final3::make_secret(0);

    let mut group = c.benchmark_group("all");
    for input in data.iter() {
        let length = input.len();
        group.throughput(Throughput::Bytes(length as u64));

        group.bench_with_input(BenchmarkId::new("FNV Hasher", length), input, |b, input| {
            b.iter(|| hasher_bench::<FnvHasher>(input))
        });

        group.bench_with_input(
            BenchmarkId::new("Std HashMap Default Hasher", length),
            input,
            |b, input| b.iter(|| hasher_bench::<DefaultHasher>(input)),
        );

        group.bench_with_input(
            BenchmarkId::new("MetroHash64 Hasher", length),
            input,
            |b, input| b.iter(|| hasher_bench::<MetroHash64>(input)),
        );

        group.bench_with_input(
            BenchmarkId::new("MetroHash128 Hasher", length),
            input,
            |b, input| b.iter(|| hasher_bench::<MetroHash128>(input)),
        );

        group.bench_with_input(
            BenchmarkId::new("XxHash Hasher", length),
            input,
            |b, input| b.iter(|| hasher_bench::<XxHash>(input)),
        );

        group.bench_with_input(
            BenchmarkId::new("wyhash v1 (C function)", length),
            input,
            |b, input| {
                b.iter(|| unsafe {
                    c_wyhash_v1(input.as_ptr() as *const libc::c_void, length as u64, 0)
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("wyhash v1 (Rust function)", length),
            input,
            |b, input| {
                b.iter(|| v1::wyhash(input, 0));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("wyhash v1 (Rust Hasher)", length),
            input,
            |b, input| b.iter(|| hasher_bench::<v1::WyHash>(input)),
        );

        group.bench_with_input(
            BenchmarkId::new("wyhash final3 (C function)", length),
            input,
            |b, input| {
                b.iter(|| unsafe {
                    c_wyhash_final3(
                        input.as_ptr() as *const libc::c_void,
                        length as u64,
                        0,
                        secret.as_ptr() as *const libc::c_ulonglong,
                    )
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("wyhash final3 (Rust function)", length),
            input,
            |b, input| {
                b.iter(|| final3::wyhash(input, 0, secret));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("wyhash final3 (Rust Hasher)", length),
            input,
            |b, input| b.iter(|| hasher_bench::<final3::WyHash>(input)),
        );
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
