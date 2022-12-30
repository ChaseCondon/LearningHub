//
// Created by Chase Condon on 30/12/2022.
//

#include <stdio.h>

// In C, these are "functions", not "methods"
int times2(int x) {
    return x * 2;
}

int main() {
    int x = 5;
    printf("%d * 2 = %d", x, times2(x));
}