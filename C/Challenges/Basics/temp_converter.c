//
// Created by Chase Condon on 30/12/2022.
//

#include <stdio.h>

double convert_to_kelvin(double temp_fahrenheit) {
    return (temp_fahrenheit - 32) * (5.0 / 9.0) + 273.15;
}

int main() {
    double temp_fahrenheit = 212.0;
    double temp_kelvin = convert_to_kelvin(temp_fahrenheit);
    printf("%.2f degrees F is %.2f degrees K\n",
           temp_fahrenheit, temp_kelvin);

    if (temp_fahrenheit <= 32.0) {
        printf("That's freezing!\n");
    } else if (temp_fahrenheit >= 212.0) {
        printf("That's boiling!\n");
    } else {
        printf("That's nice.\n");
    }
}