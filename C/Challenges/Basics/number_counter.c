//
// Created by Chase Condon on 30/12/2022.
//

#include <stdio.h>

int main() {
    for (int i = 1; i <= 10; i++) {
        printf("%d ", i);
    }
    printf("\n");

    for (int i = 10; i >= 1; i -= 2) {
        printf("%d ", i);
    }
}