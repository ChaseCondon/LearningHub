package main

import (
	"fmt"
	"strings"
	"unicode"
)

func main() {
	words := "this is a string"

	fmt.Println(titleCase(words))

	fmt.Println("Best int is", greatestOf(1234, 123512, 341, 23412345, 1234, 2135123, 5235))
}

func titleCase(title string) string {
	titleArr := strings.Split(title, " ")

	for i, word := range titleArr {
		titleArr[i] = string(unicode.ToUpper(rune(word[0]))) + word[1:]
	}

	return strings.Join(titleArr, " ")
}

func greatestOf(ints ...int) int {
	greatest := ints[0]

	for _, i := range ints {
		if i > greatest {
			greatest = i
		}
	}

	return greatest
}
