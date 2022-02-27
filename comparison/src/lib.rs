pub mod ffi {
    use libc::{c_ulonglong, c_void};

    extern "C" {
        pub fn c_wyhash_v1(
            data: *const c_void,
            length: c_ulonglong,
            seed: c_ulonglong,
        ) -> c_ulonglong;

        pub fn c_wyrng_v1(seed: *mut c_ulonglong) -> c_ulonglong;

        pub fn c_wyhash_final3(
            data: *const c_void,
            length: c_ulonglong,
            seed: c_ulonglong,
            secret: *const c_ulonglong,
        ) -> c_ulonglong;

        pub fn c_wyrng_final3(seed: *mut c_ulonglong) -> c_ulonglong;

        pub fn c_wyhash_final3_32bit_mum(
            data: *const c_void,
            length: c_ulonglong,
            seed: c_ulonglong,
            secret: *const c_ulonglong,
        ) -> c_ulonglong;

        pub fn c_wyrng_final3_32bit_mum(seed: *mut c_ulonglong) -> c_ulonglong;
    }
}
