#include "wyhash_final3.h"

uint64_t c_wyhash_final3(const void* key, size_t len, uint64_t seed,
    const uint64_t* secret) {
    return wyhash(key, len, seed, secret);
}

uint64_t c_wyrng_final3(uint64_t *s) {
    return wyrand(s);
}