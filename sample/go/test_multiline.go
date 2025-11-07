package main

import (
	"fmt"
	"os"
)

func main() {
	// Single line output
	fmt.Println("Single line debug")

	// Multiline Printf
	fmt.Printf(
		"debug: multiline message with value=%d and name=%s\n",
		42,
		"test")

	// Multiline Fprintf to stderr
	fmt.Fprintf(
		os.Stderr,
		"DEBUG: error details code=%d\n",
		500)

	// Complex multiline with multiple arguments
	fmt.Printf(
		"debug: processing item %d of %d, status=%s\n",
		1,
		10,
		"active")

	// Deeply nested multiline
	result := fmt.Sprintf(
		"debug: result=%v",
		map[string]int{
			"a": 1,
			"b": 2,
		})
	fmt.Println(result)
}
