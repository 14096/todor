BINARY_NAME := todor
CARGO := cargo
TARGET_DIR := target
RELEASE_DIR := $(TARGET_DIR)/release
DEBUG_DIR := $(TARGET_DIR)/debug

.PHONY: all
all: build

.PHONY: build
build:
	clear
	@echo "Building $(BINARY_NAME) DEBUG mode..."
	$(CARGO) build

.PHONY: release
release:
	clear
	@echo "Building $(BINARY_NAME) RELEASE mode"
	$(CARGO) build --release

.PHONY: run
run: build
	@./target/debug/todor


.PHONY: fmt
fmt:
	clear
	@echo "Formatting code"
	$(CARGO) fmt

.PHONY: test
test:
	@echo "Running tests"
	$(CARGO) test

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: doc
doc:
	@echo "Generating documentation..."
	$(CARGO) doc --open

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build         - Build in debug mode"
	@echo "  release       - Build in release mode"
	@echo "  run           - Run in debug mode"
	@echo "  fmt           - Format code"
	@echo "  test          - Run tests"
	@echo "  clean         - Clean build artifacts"
	@echo "  doc           - Generate and open documentation"
	@echo "  help          - Show this help message"
