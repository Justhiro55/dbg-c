#include <stdio.h>

int gcd(int a, int b) {
    printf("debug: gcd(%d, %d) called\n", a, b);

    while (b != 0) {
        int temp = b;
        b = a % b;
        a = temp;
        printf("DEBUG: Current values - a=%d, b=%d\n", a, b);
    }

    printf("debug: GCD result = %d\n", a);
    return a;
}

int is_prime(int n) {
    fprintf(stderr, "DEBUG: Checking if %d is prime\n", n);

    if (n <= 1) {
        printf("debug: %d is not prime (n <= 1)\n", n);
        return 0;
    }

    for (int i = 2; i * i <= n; i++) {
        if (n % i == 0) {
            printf("debug: %d is divisible by %d, not prime\n", n, i);
            return 0;
        }
    }

    printf("DEBUG: %d is prime\n", n);
    return 1;
}

double power(double base, int exp) {
    printf("debug: Calculating %.2f^%d\n", base, exp);

    double result = 1.0;
    for (int i = 0; i < exp; i++) {
        result *= base;
    }

    printf("DEBUG: Result = %.2f\n", result);
    return result;
}
