#include <stdio.h>
#include <unistd.h>

void test_c_functions() {
    // printf family
    printf("debug: testing printf\n");
    fprintf(stderr, "DEBUG: testing fprintf\n");

    // puts family
    puts("debug: testing puts");
    fputs("DEBUG: testing fputs\n", stderr);

    // character output
    fputc('D', stderr); // Should not match (no debug keyword)

    // write (POSIX)
    char buffer[] = "debug: testing write\n";
    write(1, buffer, sizeof(buffer) - 1);

    // perror
    perror("debug");
    perror("DEBUG: error occurred");

    // Normal output (should not be detected)
    puts("Normal message");
    printf("Regular output\n");
}
