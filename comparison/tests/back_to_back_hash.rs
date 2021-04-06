use comparison::ffi::c_wyhash;
use core::hash::Hasher;
use libc::c_void;
use wyhash::{wyhash, WyHash};

#[test]
fn wyhash_test() {
    let mut data = [0; 256];
    for i in 0..data.len() {
        data[i] = i as u8;
        let original = unsafe { c_wyhash(data.as_ptr() as *const c_void, i as u64, i as u64) };
        assert_eq!(original, wyhash(&data[..i], i as u64));
    }
}

#[test]
fn core_hasher() {
    let mut data = [0; 256];
    for i in 0..data.len() {
        data[i] = i as u8;
        let mut hasher = WyHash::with_seed(i as u64);
        hasher.write(&data[..i]);
        let original = unsafe { c_wyhash(data.as_ptr() as *const c_void, i as u64, i as u64) };
        assert_eq!(original, hasher.finish());
    }
}
