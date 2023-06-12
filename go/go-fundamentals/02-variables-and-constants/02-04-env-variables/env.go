package main

import (
	"fmt"
	"os"
)

func main() {
	name := os.Getenv("USERNAME")
	course := "Go Fundamentals"

	fmt.Println("Hi,", name, "your current course is", course)
	updateCourse(&course, "Getting Started with Go")

	fmt.Println("You're currently watching", course)
}

func updateCourse(coursePtr *string, newCourse string) string {
	*coursePtr = newCourse
	fmt.Println("Updated course to", *coursePtr)
	return *coursePtr
}
