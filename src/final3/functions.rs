use crate::v1::functions::{read32, read64, P0, P1};
use core::cmp::Ordering;

#[inline]
fn wymum(a: u64, b: u64) -> u64 {
    let r = u128::from(a) * u128::from(b);
    ((r >> 64) ^ r) as u64
}

#[inline]
fn read_up_to_24(bytes: &[u8]) -> u64 {
    u64::from(bytes[0]) << 16
        | u64::from(bytes[bytes.len() >> 1]) << 8
        | u64::from(bytes[bytes.len() - 1])
}

/// Generate a hash for the input data and seed
pub fn wyhash(bytes: &[u8], seed: u64, secret: [u64; 4]) -> u64 {
    let seed = seed ^ secret[0];
    let (a, b, seed) = wyhash_core(bytes, seed, secret);
    wyhash_finish(a, b, seed, bytes.len() as u64, secret[1])
}

#[inline]
pub fn wyhash_core(bytes: &[u8], seed: u64, secret: [u64; 4]) -> (u64, u64, u64) {
    let (mut a, mut b) = (0, 0);
    let mut seed = seed;
    let length = bytes.len();
    if length <= 16 {
        if length >= 4 {
            a = read32(bytes) << 32 | read32(&bytes[((length >> 3) << 2)..]);
            b = read32(&bytes[(length - 4)..]) << 32
                | read32(&bytes[(length - 4 - ((length >> 3) << 2))..]);
        } else if length > 0 {
            a = read_up_to_24(bytes);
        }
    } else {
        let mut rest_start = length - (length % 48);
        match length.cmp(&48) {
            Ordering::Greater => {
                let mut see1 = seed;
                let mut see2 = seed;

                let mut chunks = bytes.chunks(48).peekable();
                while let Some(chunk) = chunks.next() {
                    // Skip the last up to 48
                    if chunks.peek().is_some() {
                        seed = wymum(read64(chunk) ^ secret[1], read64(&chunk[8..]) ^ seed);
                        see1 = wymum(
                            read64(&chunk[16..]) ^ secret[2],
                            read64(&chunk[24..]) ^ see1,
                        );
                        see2 = wymum(
                            read64(&chunk[32..]) ^ secret[3],
                            read64(&chunk[40..]) ^ see2,
                        );
                    } else if chunk.len() == 48 {
                        rest_start -= 48;
                    }
                }
                seed ^= see1 ^ see2;
            }
            Ordering::Equal => rest_start -= 48,
            _ => (),
        }

        let mut chunks = bytes[rest_start..].chunks(16).peekable();
        while let Some(chunk) = chunks.next() {
            // Skip the last up to 16
            if chunks.peek().is_some() {
                seed = wymum(read64(chunk) ^ secret[1], read64(&chunk[8..]) ^ seed);
            }
        }

        a = read64(&bytes[(length - 16)..]);
        b = read64(&bytes[(length - 8)..]);
    }
    (a, b, seed)
}

#[inline]
pub fn wyhash_finish(a: u64, b: u64, seed: u64, length: u64, secret1: u64) -> u64 {
    wymum(secret1 ^ length, wymum(a ^ secret1, b ^ seed))
}

/// Generate new secret for wyhash
pub fn make_secret(seed: u64) -> [u64; 4] {
    let c = [
        15_u8, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58, 60, 71, 75, 77, 78, 83, 85, 86,
        89, 90, 92, 99, 101, 102, 105, 106, 108, 113, 114, 116, 120, 135, 139, 141, 142, 147, 149,
        150, 153, 154, 156, 163, 165, 166, 169, 170, 172, 177, 178, 180, 184, 195, 197, 198, 201,
        202, 204, 209, 210, 212, 216, 225, 226, 228, 232, 240,
    ];
    let mut secret = [0_u64; 4];
    let mut seed = seed;
    for i in 0..secret.len() {
        loop {
            secret[i] = 0;
            for j in (0..64).step_by(8) {
                secret[i] |= u64::from(c[((wyrng(&mut seed) as usize) % c.len())]) << j;
            }
            if secret[i] % 2 == 0 {
                continue;
            }
            let incorrect_number_of_ones_found = (0..i)
                .step_by(1)
                .find(|j| (secret[*j] ^ secret[i]).count_ones() != 32);
            if incorrect_number_of_ones_found.is_none() {
                break;
            }
        }
    }
    secret
}

/// Pseudo-Random Number Generator (PRNG)
///
/// Note that the input seed is updated
pub fn wyrng(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(P0);
    wymum(*seed, *seed ^ P1)
}

#[cfg(test)]
mod tests {
    use super::make_secret;

    #[test]
    fn generates_secret_with_32_ones() {
        let secret = make_secret(0);
        for s in &secret {
            assert_eq!(32, s.count_ones());
        }
    }
}
