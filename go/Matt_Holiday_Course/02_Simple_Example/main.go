package main

import (
	"fmt"
	"os"
	"02_Simple_Example/hello"
)

func main() {
	fmt.Printf(hello.Say(os.Args[1:]))
}