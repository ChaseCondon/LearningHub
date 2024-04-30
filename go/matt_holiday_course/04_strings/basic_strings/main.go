package main

import "fmt"

func main() {
	// all strings in Go are unicode
	// byte = uint8
	// rune = int32

	// string is physically a sequence of bytes (UTF-8 encoding)
	// logically a sequence of (unicode) runes

	// Runes (characters) enclosed in single quotes
	// "Raw" strings use backtick quotes --> `string with "quotes"`
	// Also don't evaluate escape characters

	s := "élite"
	fmt.Printf("%8T %[1]v %d\n", s, len(s)) // Length of string is length of byte array equivalent
	fmt.Printf("%8T %[1]v %d\n", []rune(s), len([]rune(s)))
	fmt.Printf("%8T %[1]v %d\n", []byte(s), len([]byte(s))) // UTF-8 encoding expands encoding for special characters like é

	s2 := "hello, world"
	hello := s2[:5] // reuses memory from s2 due to substring nature
	world := s2[7:] // reuses memory from s2 due to substring nature

	fmt.Printf("%s, %s", hello, world)

	s3 := "the quick brown fox"

	// a := len(s)                   // 19
	// b := s3[:3]                   // "the" - reference
	// c := s3[4:9]                  // "quick" - reference
	// d := s3[:4] + "slow" + s3[9:] // "the slow brown fox" - copy

	//s3[5] = 'a' 					// SYNTAX ERROR
	s3 += "es" // COPY - s3 now points to a new block of memory with "the quick brown foxes"
	// data from original memory remains though as b, c, and d all reference to its memory address
	// If b, c, d did not exist, then garbage collector would clean up original memory after copy
}
