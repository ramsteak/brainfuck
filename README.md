# Brainfuck
This is a simple Brainfuck interpreter written in Rust. Brainfuck is an esoteric programming language consisting of only eight commands and ran on an infinite memory tape.

## Features
- Tokenizes and parses Brainfuck code into an abstract syntax tree (AST).
- Provides error handling for unmatched loop brackets and other syntax errors.
- Executes the AST on a virtual tape that simulates the memory of the Brainfuck program.
- Supports both reading Brainfuck code from a file and reading from standard input.
- Handles input and output with ASCII characters.

## Usage
Make sure to have rust and cargo installed on your system

1. Clone the repository
1. Run with cargo `cargo run -- ./code.bf`
1. Alternatively you can input your program from stdin with `cargo run --`
1. To terminate the input press Ctrl-D on Unix and Ctrl-Z on Windows.

## Contributions
Contributions to this project are welcome! If you find any issues, have feature suggestions, or want to improve the code, feel free to submit a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.