package main

import "fmt"

func main() {
	coursesInProg := []string{
		"Docker & Kubernetes: The Big Picture",
		"Docker Networking",
		"Getting Started with Kubernetes",
		"Kubernetes Deep Dive"}

	coursesCompleted := []string{
		"Docker & Kubernetes: The Big Picture",
		"Docker Deep Dive"}

	for i, courseProg := range coursesInProg {
		fmt.Println(i+1, "-", courseProg)
		for _, courseComp := range coursesCompleted {
			if courseProg == courseComp {
				fmt.Println("Clash!", courseComp, "is in both lists!")
			}
		}
	}
}
