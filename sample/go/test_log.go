package main

import (
	"log"
	"os"
)

func main() {
	// Setup logger
	log.SetOutput(os.Stdout)

	log.Println("Starting application")
	log.Println("debug: initialization complete")

	log.Printf("debug: value=%d, name=%s", 42, "test")

	// Fatal and Panic (normally wouldn't use in production)
	// log.Fatal("debug: fatal error")
	// log.Fatalf("DEBUG: formatted fatal %d", 1)
	// log.Panic("debug: panic message")
	// log.Panicf("DEBUG: formatted panic %s", "error")

	log.Print("Regular log message")

	debugMode := true
	if debugMode {
		log.Println("DEBUG: running in debug mode")
	}
}
