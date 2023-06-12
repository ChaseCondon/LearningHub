package main

import (
	"fmt"
	"math/rand"
)

func main() {
	switch tmpNum := rand.Intn(10); tmpNum {
	case 0, 2, 4, 6, 8:
		fmt.Println("We go the following even number -", tmpNum)
	case 1, 3, 5, 7, 9:
		fmt.Println("We go the following odd number -", tmpNum)
	}
}
