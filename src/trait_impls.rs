use core::hash::Hasher;
use wyhash::{mix_with_p0, wyhash_core, wyhash_finish};

/// WyHash hasher
#[derive(Default)]
pub struct WyHash {
    h: u64,
    size: u64,
}

impl WyHash {
    /// Create hasher with a seed
    pub fn with_seed(seed: u64) -> Self {
        WyHash { h: seed, size: 0 }
    }
}

impl Hasher for WyHash {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            self.h = mix_with_p0(self.h);
        } else {
            for bytes in bytes.chunks(u64::max_value() as usize) {
                self.h = wyhash_core(bytes, self.h);
                self.size += bytes.len() as u64
            }
        }
    }
    #[inline]
    fn finish(&self) -> u64 {
        wyhash_finish(self.size, self.h)
    }
}
