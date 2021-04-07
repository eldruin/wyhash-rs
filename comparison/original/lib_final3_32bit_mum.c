#define WYHASH_32BIT_MUM 1
#include "wyhash_final3.h"

uint64_t c_wyhash_final3_32bit_mum(const void* key, size_t len, uint64_t seed,
    const uint64_t* secret) {
    return wyhash(key, len, seed, secret);
}

uint64_t c_wyrng_final3_32bit_mum(uint64_t *s) {
    return wyrand(s);
}