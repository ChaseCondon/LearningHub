package main

import (
	"fmt"
	"os"
)

func main() {
	_, err := os.Open("C:\\Users\\chase\\GolandProjects\\learning\\go-fundamentals\\04-03-error-handling\\test1.txt")

	if err != nil {
		fmt.Println("This is the error code:", err)
	}
}
