#include <string>
#include <sstream>
#include <iostream>
#include <bitset>
#include <chrono>

int main() {
    auto sum = 0;

    auto start = std::chrono::high_resolution_clock::now();

    for (std::string group; std::getline(std::cin, group);) {
        std::bitset<26> yesses = 0;
        for (const char &q : group) {
            if (q >= 'a' && q <= 'z') {
                yesses |= 1 << (q - 'a');
            }
        }
        sum += yesses.count();
    }

    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);

    std::cout << duration.count() << std::endl;

    std::cout << sum << std::endl;

    return 0;
}