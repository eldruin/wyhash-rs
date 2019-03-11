extern crate wyhash;

#[cfg(feature = "std")]
fn run() {
    use self::wyhash::WyHash;
    use std::hash::Hasher;

    let mut hasher = WyHash::with_seed(1);
    hasher.write(&[0]);
    assert_eq!(0xcb4b8ebdf7240e2c, hasher.finish());
}

fn main() {
    #[cfg(feature = "std")]
    run();
}
