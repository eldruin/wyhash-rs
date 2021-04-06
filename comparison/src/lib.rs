#[link(name = "original", kind = "static")]
pub mod ffi {
    use libc::{c_ulonglong, c_void};

    extern "C" {
        pub fn c_wyhash(data: *const c_void, length: c_ulonglong, seed: c_ulonglong)
            -> c_ulonglong;

        pub fn c_wyrng(seed: *mut c_ulonglong) -> c_ulonglong;
    }
}
