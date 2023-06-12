package main

import (
	"fmt"
	"reflect"
	"strconv"
)

//var (
//	name, course string
//	module, clip int
//
//	name, course = "Chase Condon", "Go Fundamentals"
//	module, clip = 4, 2
//
//	name = "Chase Condon"
//	course = "Go Fundamentals"
//	module = "4" // Uh-oh... that's a string!!
//	clip = 2 // Needs to be an integer
//)

func main() {
	name := "Chase Condon"
	course := "Go Fundamentals"
	module := "4"
	clip := 2
	//courseComplete := false

	fmt.Println("Name and course are set to", name, "and", course)
	fmt.Println("Module and clip are set to", module, "and", clip)
	fmt.Println("Name is of type", reflect.TypeOf(name))
	fmt.Println("Module is of type", reflect.TypeOf(module))

	// total := module + clip
	// fmt.Println("module + clip =", total)

	iModule, err := strconv.Atoi(module)
	if err == nil {
		total := iModule + clip
		fmt.Println("module + clip =", total)
	}

	fmt.Println("Memory address of *course* variable is", &course)

	var ptr *string = &course
	fmt.Println("Pointing to course variable at address", ptr, "which holds this value", *ptr)
}
