//
// Created by Chase Condon on 30/12/2022.
//

#include <stdio.h>

int main() {

    // All below flows are identical to Java (because Java adopted them from C)
    int x = 0;
    if (x != 0) {
        printf("x is not 0\n");
    } else {
        printf("x is 0\n");
    }

    // Boolean logic using int. Truthiness similar to python
    if (x) {
        printf("x is true\n");
    } else {
        printf("x is false\n");
    }

    switch (x) {
        case 0:
            printf("x is 0\n");
            break;
        case 1:
            printf("x is 1\n");
            break;
        default:
            printf("x is not 0 or 1\n");
    }

    // below boolean is identical to x != 0
    while (x) {
        // Do something
    }

    // Originally, variable declaration within for block wasn't possible
    // Made possible in later versions, but may see variable declared prior to block
    for (int i = 0; i < 10; i++) {
        printf("%d\n", i);
    }
}