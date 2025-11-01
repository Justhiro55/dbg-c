#include <stdio.h>
#include <ctype.h>

void to_uppercase(char *str) {
    printf("DEBUG: Converting string to uppercase\n");

    for (int i = 0; str[i] != '\0'; i++) {
        str[i] = toupper(str[i]);
    }

    printf("debug: Conversion completed\n");
}

int count_vowels(const char *str) {
    fprintf(stderr, "debug: Counting vowels in string\n");

    int count = 0;
    for (int i = 0; str[i] != '\0'; i++) {
        char c = tolower(str[i]);
        if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
            count++;
            printf("DEBUG: Found vowel '%c' at position %d\n", str[i], i);
        }
    }

    printf("debug: Total vowels = %d\n", count);
    return count;
}

int str_length(const char *str) {
    printf("debug: Calculating string length\n");

    int len = 0;
    while (str[len] != '\0') {
        len++;
    }

    printf("DEBUG: String length = %d\n", len);
    return len;
}
