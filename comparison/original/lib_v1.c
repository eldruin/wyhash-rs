#include "wyhash_v1.h"

unsigned long long c_wyhash_v1(const void* key, unsigned long long len, unsigned long long seed) {
    return wyhash(key, len, seed);
}

unsigned long long c_wyrng_v1(unsigned long long *s) {
    return wyrng(s);
}