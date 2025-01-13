# Makefile for the Rust Guessing Game project

# Default target: build the project
all: build

# Build the project
build:
    cargo build

# Run the project
run:
    cargo run

# Clean the project
clean:
    cargo clean

# Format the code
format:
    cargo fmt

# Check for warnings and errors
check:
    cargo check

# Run tests (if any)
test:
    cargo test