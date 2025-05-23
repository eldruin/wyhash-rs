use crate::final3::functions::{wyhash_core, wyhash_finish, wyrng};
use crate::v1::functions::{read64, P0, P1, P2, P3};
use core::hash::{BuildHasher, Hasher};
use rand_core::{impls, RngCore, SeedableRng};

/// WyHash hasher builder
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Hash)]
pub struct WyHasherBuilder {
    seed: u64,
    secret: [u64; 4],
}

impl WyHasherBuilder {
    /// Create hasher builder with a seed
    pub fn new(seed: u64, secret: [u64; 4]) -> Self {
        WyHasherBuilder { seed, secret }
    }
}

impl Default for WyHasherBuilder {
    fn default() -> Self {
        WyHasherBuilder::new(0, [P0, P1, P2, P3])
    }
}

impl BuildHasher for WyHasherBuilder {
    type Hasher = WyHash;

    fn build_hasher(&self) -> Self::Hasher {
        WyHash::new(self.seed, self.secret)
    }
}

/// WyHash hasher
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Hash)]
pub struct WyHash {
    seed: u64,
    a: u64,
    b: u64,
    size: u64,
    secret: [u64; 4],
}

impl WyHash {
    /// Create hasher with a seed
    pub fn new(seed: u64, secret: [u64; 4]) -> Self {
        WyHash {
            seed,
            a: 0,
            b: 0,
            size: 0,
            secret,
        }
    }
}

impl Default for WyHash {
    fn default() -> Self {
        WyHash::new(0, [P0, P1, P2, P3])
    }
}

impl Hasher for WyHash {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.seed ^= self.secret[0];
        for chunk in bytes.chunks(u64::MAX as usize) {
            let (a, b, seed) = wyhash_core(chunk, self.seed, self.secret);
            self.a = a;
            self.b = b;
            self.seed = seed;
            self.size += chunk.len() as u64
        }
    }
    #[inline]
    fn finish(&self) -> u64 {
        wyhash_finish(self.a, self.b, self.seed, self.size, self.secret[1])
    }
}

/// WyRng random number generator
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct WyRng(u64);

impl RngCore for WyRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    fn next_u64(&mut self) -> u64 {
        wyrng(&mut self.0)
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
}

impl SeedableRng for WyRng {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        WyRng(read64(&seed))
    }

    fn seed_from_u64(state: u64) -> Self {
        WyRng(state)
    }
}
