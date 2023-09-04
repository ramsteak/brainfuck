# Brainfuck

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![GitHub release](https://img.shields.io/github/v/release/ramsteak/brainfuck.svg)
[![Rust](https://github.com/ramsteak/brainfuck/actions/workflows/rust.yml/badge.svg)](https://github.com/ramsteak/brainfuck/actions/workflows/rust.yml)

This is a simple Brainfuck interpreter/transpiler written in Rust. Brainfuck is an esoteric programming language consisting of only eight commands and ran on an infinite memory tape.
This version supports line comments with `#` and implements two additional commands:

- `#` : everything between a `#` and a newline is considered a comment (must be activated with the `-m` flag)
- `&` : immediately terminates the program, with exit code of the active cell
- `?` : prints the entire memory tape, useful for debug purposes

## Features

- Tokenizes and parses Brainfuck code into an abstract syntax tree (AST).
- Provides error handling for unmatched loop brackets and other syntax errors.
- Executes the AST on a virtual tape that simulates the memory of the Brainfuck program.
- Supports both reading Brainfuck code from a file and reading from standard input.
- Handles input and output with ASCII characters.
- Custom instructions & and ?
- Transpiles the code into c (each instruction is translated directly with no optimization) in order to compile to binaries

## Compilation

Make sure to have rust and cargo installed on your system.

1. Clone the repository
1. Build the project with cargo `cargo build`
1. Run with cargo `cargo run -- ./code.bf`
1. Alternatively you can input your program from stdin with `cargo run --`
1. To terminate the input press Ctrl-D on Unix and Ctrl-Z on Windows.

You can also build the interpreter with make:

1. Build with the provided makefile with `make build`
1. Run the interpreter with `./brainfuck ./code.bf`

### Usage

`brainfuck [OPTIONS] [path]`

- `-h`: prints the usage
- `-c`: transpiles the bf file into c
- `-m`: turns on line comments
- `-v`: prints the version
- `path`: the path of the bf file

## Contributions

Contributions to this project are welcome! If you find any issues, have feature suggestions, or want to improve the code, feel free to submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Issues

The program runs as expected on Windows, but on Linux the `,` instruction reads to a buffer, and does not print until a newline is sent
