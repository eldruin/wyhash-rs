// Generate the hash table for the tests using the original implementation.
// This can be built using the CMake script included here or however else you want.

#include <iostream>
#include <vector>
#include "wyhash.h"

int main(int, char**)
{
    std::vector<unsigned char> data(256);
    for (size_t i = 0; i < data.size(); ++i)
    {
        data[i] = i;
        std::cout << std::hex << wyhash(data.data(), i, i) << std::endl;
    }
}