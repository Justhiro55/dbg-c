package main

import (
	"fmt"
	"log"
)

func main() {
	fmt.Println("Starting application")
	fmt.Println("debug: initialization started")

	x := 42
	fmt.Printf("Value: %d\n", x)

	log.Println("error: something went wrong")
	log.Println("DEBUG: detailed error info")

	fmt.Print("inline message")

	result := calculate(10, 20)
	fmt.Println("Result:", result)
}

func calculate(a, b int) int {
	fmt.Printf("debug: calculating %d + %d\n", a, b)
	return a + b
}
