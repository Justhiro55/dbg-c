#include <stdio.h>

int main() {
    int x = 10;
    int y = 20;

    printf("debug: x = %d\n", x);
    printf("Regular message\n");
    fprintf(stderr, "DEBUG: y = %d\n", y);

    printf("debug: some commented debug\n");

    return 0;
}
