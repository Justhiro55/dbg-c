package main

import (
	"fmt"
	"log"
)

func main() {
	// Already commented debug statements
	// fmt.Println("debug: starting")
	// fmt.Printf("debug: value=%d\n", 42)

	// Active non-debug output
	fmt.Println("Application running")

	// Commented multiline
	// fmt.Printf(
	// 	"debug: test=%s\n",
	// 	"value")

	// Commented log statements
	// log.Println("debug: log message")
	// log.Printf("DEBUG: formatted=%d\n", 100)

	process()
}

func process() {
	fmt.Println("Processing...")
	// fmt.Println("debug: in process function")
}
