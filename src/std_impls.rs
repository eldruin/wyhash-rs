use wyhash;
extern crate std;
use self::std::hash::Hasher;

/// WyHash hasher
#[derive(Default)]
pub struct WyHash {
    h: u64,
}

impl WyHash {
    /// Create hasher with a seed
    pub fn with_seed(seed: u64) -> Self {
        WyHash { h: seed }
    }
}

impl Hasher for WyHash {
    fn write(&mut self, bytes: &[u8]) {
        for bytes in bytes.chunks(u64::max_value() as usize) {
            self.h = wyhash(bytes, self.h);
        }
    }
    fn finish(&self) -> u64 {
        self.h
    }
}
