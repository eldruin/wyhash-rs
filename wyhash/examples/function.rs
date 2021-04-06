// Example of using the free functions.

use wyhash::{make_secret, wyhash, wyrng};

fn main() {
    let secret = make_secret(0);
    assert_eq!(0x2614_9574_d5fa_c1fe, wyhash(&[0, 1, 2], 3, secret));

    let mut rng_seed = 1;
    let random_number = wyrng(&mut rng_seed);
    assert_eq!(0xcdef_1695_e1f8_ed2c, random_number);
    assert_eq!(0xa076_1d64_78bd_6430, rng_seed);
}
