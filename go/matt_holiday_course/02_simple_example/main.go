package main

import (
	"02_simple_example/hello"
	"fmt"
	"os"
)

func main() {
	fmt.Printf(hello.Say(os.Args[1:]))
}
