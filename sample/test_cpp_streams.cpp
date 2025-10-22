#include <iostream>

void test_cpp_streams() {
    int value = 42;

    // std::cout
    std::cout << "debug: testing cout" << std::endl;
    std::cout << "DEBUG: value = " << value << std::endl;

    // std::cerr
    std::cerr << "debug: testing cerr" << std::endl;
    std::cerr << "DEBUG: error message" << std::endl;

    // std::clog
    std::clog << "debug: testing clog" << std::endl;
    std::clog << "DEBUG: log message" << std::endl;

    // Normal output (should not be detected)
    std::cout << "Normal message" << std::endl;
    std::cerr << "Regular error" << std::endl;
}

int main() {
    test_cpp_streams();
    return 0;
}
