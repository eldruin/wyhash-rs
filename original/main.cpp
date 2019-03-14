// Generate the hash table for the tests using the original implementation.
// This can be built using the CMake script included here or however else you want.

#include <iostream>
#include <vector>
#include "wyhash.h"

void print_hash_table()
{
    std::cout << "Hash table:\n\n";
    std::vector<unsigned char> data(256);
    for (size_t i = 0; i < data.size(); ++i)
    {
        data[i] = i;
        std::cout << std::hex << wyhash(data.data(), i, i) << std::endl;
    }
}

void print_prng_table()
{
    std::cout << "PRNG table:\n\n";
    for (unsigned long long i = 0; i < 10; ++i)
    {
        unsigned long long d = i;
        std::cout << std::hex << wyrng(&d) << std::endl;
    }
}

int main(int, char**)
{
    print_hash_table();
    std::cout << "\n\n";
    print_prng_table();
}