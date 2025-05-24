.PHONY: help build test clean check fmt clippy run install dev doc

# Default target
help:
	@echo "Available targets:"
	@echo "  build    - Build the project in release mode"
	@echo "  dev      - Build the project in debug mode"
	@echo "  test     - Run all tests"
	@echo "  check    - Check code without building"
	@echo "  fmt      - Format code"
	@echo "  clippy   - Run clippy linter"
	@echo "  clean    - Clean build artifacts"
	@echo "  run      - Run in development mode"
	@echo "  install  - Install the binary"
	@echo "  doc      - Generate documentation"

# Build targets
build:
	cargo build --release

dev:
	cargo build

# Testing
test:
	cargo test

test-verbose:
	cargo test -- --nocapture

# Code quality
check:
	cargo check

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

# Utility targets
clean:
	cargo clean

run:
	cargo run

install:
	cargo install --path .

doc:
	cargo doc --open

# Development workflow
quick: check test
	@echo "Quick check passed!"

full: fmt clippy test build
	@echo "Full build pipeline completed!" 