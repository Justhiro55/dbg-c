#include <iostream>

// Test file for multiline statement detection in C++

int main() {
    int value = 42;

    // Test 1: Single line debug with cout
    // std::cout << "debug: single line test" << std::endl;

    // Test 2: Multiline cout with continuation
    // std::cout << "debug: this is a very \
long debug message" << std::endl;

    // Test 3: Multiline cerr
    // std::cerr << "DEBUG: error message \
on stderr" << std::endl;

    // Test 4: Normal output (should not be detected with --debug flag)
    // std::cout << "Normal message" << std::endl;

    // Test 5: Multiline normal output
    // std::cout << "This is a normal \
multiline message" << std::endl;

    // Test 6: clog with debug
    // std::clog << "debug: log message" << std::endl;

    // Test 7: Complex multiline with multiple operators
    // std::cout << "debug: value=" << value << \
" result=" << (value * 2) << std::endl;

    // Test 8: Very long multiline
    // std::cout << "DEBUG: line1 " \
<< "line2 " \
<< "line3 " \
<< "line4" << std::endl;

    return 0;
}
