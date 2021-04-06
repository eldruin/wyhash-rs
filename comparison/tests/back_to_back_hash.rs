use comparison::ffi::{c_wyhash_final3, c_wyhash_v1};
use core::hash::Hasher;
use libc::{c_ulonglong, c_void};
use wyhash::final3;
use wyhash::{wyhash, WyHash};

#[test]
fn wyhash_test() {
    let mut data = [0; 256];
    for i in 0..data.len() {
        data[i] = i as u8;
        let original = unsafe { c_wyhash_v1(data.as_ptr() as *const c_void, i as u64, i as u64) };
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
        let original = unsafe { c_wyhash_v1(data.as_ptr() as *const c_void, i as u64, i as u64) };
        assert_eq!(original, hasher.finish());
    }
}

#[test]
fn wyhash_test_final3() {
    let secret = final3::make_secret(0);
    let mut data = [0; 256];
    for i in 0..data.len() {
        data[i] = i as u8;
        let original = unsafe {
            c_wyhash_final3(
                data.as_ptr() as *const c_void,
                i as u64,
                i as u64,
                secret.as_ptr() as *const c_ulonglong,
            )
        };
        assert_eq!(original, final3::wyhash(&data[..i], i as u64, secret));
    }
}
