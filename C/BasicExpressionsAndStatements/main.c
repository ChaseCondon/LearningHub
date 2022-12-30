//
// Created by Chase Condon on 29/12/2022.
//

#include <stdio.h>

int main() {
    // Basic printf usage
    printf("%d %f %s\n\n", 123, 3.14159, "abc");      // 123 3.14159 abc

    // Conditional logic
    printf("%d\n", 3 < 2);      // 0
    printf("%d\n", 2 < 3);      // 1
    printf("%d\n", 1 && 0);     // boolean logic - 0 is false, non-zero is true. 0
    printf("%d\n", 1 || 0);     // 1
    printf("%d\n", 0 && 123);   // 0
    printf("%d\n", 0 || 123);   // 1
    printf("%d\n\n", 1 < 2 && 2 < 3);     // 1

    // Variables
    int x = 1;
    const float f = 3.14159;

    // Arrays. Declaring size as bellow auto allocates memory
    int numbers[3] = { 1, 2, 3};
    char letters[5] = { 'a', 'b', 'c', 'd', 'z'};

    printf("%d\n", numbers[0]);     // 1
    printf("%c\n", letters[1]);     // b

    numbers[1] = 99;
    printf("%d\n", numbers[1]);     // 99
    numbers[2] += 1;
    printf("%d\n", numbers[2]);     // 4

    // Strings
    char name[32];
    char name2[] = "George";    // Array length determined by compiler using length of string literal

    return 0;
}