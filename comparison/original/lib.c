#include "wyhash.h"

unsigned long long c_wyhash(const void* key, unsigned long long len, unsigned long long seed) {
    return wyhash(key, len, seed);
}

unsigned long long c_wyrng(unsigned long long *s) {
    return wyrng(s);
}