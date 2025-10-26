#include <stdio.h>
#include <stdlib.h>

// 関数プロトタイプ
int add(int a, int b);
int factorial(int n);
void reverse_string(char *str);

int main() {
    printf("=== Sample C Program ===\n\n");

    // 加算のテスト
    int a = 5, b = 3;
    printf("debug: Testing add function with a=%d, b=%d\n", a, b);
    int sum = add(a, b);
    printf("Result: %d + %d = %d\n\n", a, b, sum);

    // 階乗のテスト
    int n = 5;
    fprintf(stderr, "DEBUG: Calculating factorial of %d\n", n);
    int fact = factorial(n);
    printf("Result: %d! = %d\n\n", n, fact);

    // 文字列反転のテスト
    char text[] = "Hello, World!";
    printf("debug: Original string: %s\n", text);
    reverse_string(text);
    printf("Result: Reversed string: %s\n\n", text);

    printf("=== Program finished ===\n");
    return 0;
}

int add(int a, int b) {
    printf("DEBUG: add(%d, %d) called\n", a, b);
    return a + b;
}

int factorial(int n) {
    if (n <= 1) {
        printf("debug: Base case reached, returning 1\n");
        return 1;
    }
    printf("debug: factorial(%d) = %d * factorial(%d)\n", n, n, n-1);
    return n * factorial(n - 1);
}

void reverse_string(char *str) {
    printf("DEBUG: reverse_string called\n");
    int len = 0;
    while (str[len] != '\0') len++;

    for (int i = 0; i < len / 2; i++) {
        char temp = str[i];
        str[i] = str[len - 1 - i];
        str[len - 1 - i] = temp;
    }
    printf("debug: String reversal completed\n");
}
