# Define variables
CARGO := cargo
TARGET := release
EXECNAME := brainfuck

# Check if Cargo is installed
ifeq (, $(shell which $(CARGO)))
    $(error "Cargo is not installed. Please install Rust and Cargo from https://www.rust-lang.org/tools/install")
endif

# Default target
all: build

# Build the project
build:
        $(CARGO) build --$(TARGET)
        cp ./target/release/$(EXECNAME) .

# Clean the project
clean:
        $(CARGO) clean

# Build and run the project
run: build
        $(CARGO) run

# Build for debug mode
debug:
        $(MAKE) TARGET=debug build

# Build for release mode
release:
        $(MAKE) TARGET=release build

# PHONY targets to avoid conflicts with filenames
.PHONY: all build clean run debug release
