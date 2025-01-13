# Makefile for the Rust Learning Journey
# This Makefile is part of my journey to learn Rust, featuring a guessing game project.

# Default target: build the project
all: build

# Build the project
build:
    @echo "Building the Guessing Game..."
    cargo build

# Run the project
run:
    @echo "Running the Guessing Game..."
    cargo run

# Clean the project
clean:
    @echo "Cleaning up build artifacts..."
    cargo clean

# Format the code
format:
    @echo "Formatting the code..."
    cargo fmt

# Check for warnings and errors
check:
    @echo "Checking the code for warnings and errors..."
    cargo check

# Run tests (if any)
test:
    @echo "Running tests..."
    cargo test

# Display a message about the project
about:
    @echo "----------------------------------------"
    @echo "Rust Learning Journey: Guessing Game"
    @echo "----------------------------------------"
    @echo "This project is part of my journey to learn Rust."
    @echo "It features a simple guessing game where you try to guess a randomly generated number."
    @echo "The game provides feedback on whether your guess is too high, too low, or correct."
    @echo "You can exit the game at any time by typing 'exit' or 'quit'."
    @echo "----------------------------------------"

# Showcase the learning journey
journey:
    @echo "----------------------------------------"
    @echo "My Rust Learning Journey"
    @echo "----------------------------------------"
    @echo "1. Understanding Rust basics: types, variables, and functions."
    @echo "2. Exploring modules and how to organize code."
    @echo "3. Building a simple guessing game to apply Rust concepts."
    @echo "4. Learning about error handling and user input."
    @echo "5. Experimenting with external crates like 'rand'."
    @echo "----------------------------------------"

# Help target to display available commands
help:
    @echo "Available targets:"
    @echo "  all      - Build the project"
    @echo "  build    - Build the project"
    @echo "  run      - Run the project"
    @echo "  clean    - Clean the project"
    @echo "  format   - Format the code"
    @echo "  check    - Check for warnings and errors"
    @echo "  test     - Run tests"
    @echo "  about    - Display information about the project"
    @echo "  journey  - Showcase the learning journey"
    @echo "  help     - Display this help message"