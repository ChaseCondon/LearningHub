//
// Created by Chase Condon on 30/12/2022.
//

#include <stdio.h>

void print_stars(int num_stars) {
    for(int i = 1; i <= num_stars; i++) {
        printf("*");
    }
    printf("\n");
}

int main() {
    for (int i = 1; i <= 10; i++) {
        print_stars(i);
    }
}