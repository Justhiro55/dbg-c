#include <stdio.h>

// Test file for multiline statement detection in C

int main() {
    // Test 1: Single line debug statement
    printf("debug: single line test\n");

    // Test 2: Multiline with backslash continuation
    printf("debug: this is a very \
long debug message that spans \
multiple lines");

    // Test 3: Multiline fprintf with continuation
    fprintf(stderr, "DEBUG: error message \
on multiple lines");

    // Test 4: Normal output (should not be detected with --debug flag)
    printf("Normal single line\n");

    // Test 5: Multiline normal output
    printf("This is a normal \
multiline message");

    // Test 6: puts with debug
    puts("debug: testing puts");

    // Test 7: fputs multiline
    fputs("DEBUG: fputs \
multiline test\n", stderr);

    // Test 8: Complex printf with multiple arguments
    printf("debug: value1=%d, value2=%d\n", 42, 100);

    // Test 9: Very long multiline debug
    printf("debug: line1 \
line2 \
line3 \
line4");

    return 0;
}
