extern crate clap;
extern crate crossterm;

use crossterm::{
    event::{self, Event},
    terminal,
};

use clap::{arg, Command};

use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    INC, //+ Increment the current value
    DEC, //- Decrement the current value
    MLT, //< Move left in the memory array
    MRT, //> Move right in the memory array
    LEN, //[ Start a loop section
    LEX, //] End a loop section
    OUT, //, Output the current value as character
    INP, //. Input a character as a value
}
#[derive(Debug)]
enum BFErrorCode {
    UnmatchedLoopExit,
    UnmatchedLoopEnter,
    KeyboardInterrupt,
}

#[allow(dead_code)]
#[derive(Debug)]
struct BFError {
    code: BFErrorCode,
    message: String,
}

impl BFError {
    fn new(code: BFErrorCode, message: String) -> Self {
        BFError {
            code: code,
            message: message,
        }
    }
}

/// Takes a string representing the code and generates a vec of tokens,
/// representing the program.
fn tokenize(code: String) -> Result<Vec<Token>, BFError> {
    let mut tokens = Vec::<Token>::new();

    for char in code.chars() {
        match char {
            '+' => tokens.push(Token::INC),
            '-' => tokens.push(Token::DEC),
            '<' => tokens.push(Token::MLT),
            '>' => tokens.push(Token::MRT),
            '[' => tokens.push(Token::LEN),
            ']' => tokens.push(Token::LEX),
            ',' => tokens.push(Token::INP),
            '.' => tokens.push(Token::OUT),
            _ => (),
        };
    }

    Ok(tokens)
}

#[derive(Debug, Clone)]
enum AstNode {
    INC,
    DEC,
    MRT,
    MLT,
    OUT,
    INP,
    LOP(Vec<AstNode>),
}

/// Parses a vector of tokens into an AST
fn parse(tokens: Vec<Token>) -> Result<Vec<AstNode>, BFError> {
    let mut ast = Vec::<AstNode>::new();
    let mut depth = 0;
    let mut loopstart = 0;

    for (i, token) in tokens.iter().enumerate() {
        if depth == 0 {
            match token {
                Token::INC => ast.push(AstNode::INC),
                Token::DEC => ast.push(AstNode::DEC),
                Token::MRT => ast.push(AstNode::MRT),
                Token::MLT => ast.push(AstNode::MLT),
                Token::OUT => ast.push(AstNode::OUT),
                Token::INP => ast.push(AstNode::INP),
                Token::LEX => {
                    return Err(BFError::new(
                        BFErrorCode::UnmatchedLoopExit,
                        "Unmatched character \"]\"".to_string(),
                    ))
                }
                Token::LEN => {
                    depth += 1;
                    loopstart = i + 1
                }
            }
        } else {
            match token {
                Token::LEN => depth += 1,
                Token::LEX => depth -= 1,
                _ => (),
            }
            if depth == 0 {
                let loopend = i;
                let subast = parse(tokens[loopstart..loopend].to_vec())?;
                ast.push(AstNode::LOP(subast));
            }
        }
    }
    if depth != 0 {
        return Err(BFError {
            code: BFErrorCode::UnmatchedLoopEnter,
            message: "Unmatched character \"]\"".to_string(),
        });
    }

    Ok(ast)
}

/// The struct represents the memory of a brainfuck program
struct Tape {
    cell: usize,
    tape: Vec<u8>,
}
impl Tape {
    fn new() -> Self {
        Tape {
            cell: 0,
            tape: vec![0],
        }
    }
    /// Moves the tape head one space to the right
    fn mrt(&mut self) {
        self.cell += 1;
        if self.cell == self.tape.len() {
            self.tape.push(0)
        }
    }
    /// Moves the tape head one space to the left
    fn mlt(&mut self) {
        if self.cell > 0 {
            self.cell -= 1
        };
    }
    /// Returns the value in the tape head cell
    fn get(&self) -> u8 {
        self.tape[self.cell]
    }
    /// Sets the value in the tape head cell
    fn set(&mut self, val: u8) {
        self.tape[self.cell] = val
    }
    /// Adds one to the value in the tape head cell, wrapping on overflow
    fn add(&mut self) {
        self.tape[self.cell] = self.tape[self.cell].wrapping_add(1)
    }
    /// Removes one to the value in the tape head cell, wrapping on underflow
    fn sub(&mut self) {
        self.tape[self.cell] = self.tape[self.cell].wrapping_sub(1)
    }
}

/// Executes a brainfuck AST, given the memory tape
fn execute(ast: Vec<AstNode>, tape: &mut Tape) -> Result<(), BFError> {
    terminal::enable_raw_mode().unwrap();
    for node in ast {
        match node {
            AstNode::INC => tape.add(),
            AstNode::DEC => tape.sub(),
            AstNode::MRT => tape.mrt(),
            AstNode::MLT => tape.mlt(),
            AstNode::OUT => {
                print!("{}", tape.get() as char);
                stdout().flush().unwrap_or_default()
            }
            AstNode::INP => loop {
                match event::read() {
                    Ok(Event::Key(event::KeyEvent {
                        code: event::KeyCode::Char('c'),
                        kind: event::KeyEventKind::Press,
                        state: _,
                        modifiers: event::KeyModifiers::CONTROL,
                    })) => {
                        return Err(BFError {
                            code: BFErrorCode::KeyboardInterrupt,
                            message: "Received Ctrl-C".to_string(),
                        })
                    }
                    Ok(Event::Key(event::KeyEvent {
                        code: event::KeyCode::Enter,
                        kind: event::KeyEventKind::Press,
                        state: _,
                        modifiers: _,
                    })) => {
                        tape.set('\n' as u8);
                        break;
                    }

                    Ok(Event::Key(event::KeyEvent {
                        code: event::KeyCode::Char(char),
                        kind: event::KeyEventKind::Press,
                        state: _,
                        modifiers: _,
                    })) => {
                        tape.set(char as u8);
                        break;
                    }
                    _ => (),
                }
            },
            AstNode::LOP(subloop) => {
                while tape.get() != 0 {
                    execute(subloop.clone(), tape)?
                }
            }
        }
    }
    terminal::disable_raw_mode().unwrap();
    Ok(())
}

fn main() {
    let command = Command::new("brainfuck").version("0.1.0").arg(
        arg![path: [path] "The path of the .bf file.\nIf no path is specified, reads from stdin to EOF (Ctrl-D / Ctrl-Z)"],
    );

    let matches = command.get_matches();

    let mut code = String::new();
    let code = match matches.get_one::<String>("path") {
        None => {
            stdin()
                .read_to_string(&mut code)
                .expect("Failed to read from stdin");
            code
        }
        Some(filename) => {
            let mut file = File::open(filename).expect("Failed to open file");
            file.read_to_string(&mut code)
                .expect("Failed to read from file");
            code
        }
    };
    let tokens = match tokenize(code) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("{}", e.message);
            exit(-1)
        }
    };
    let ast = match parse(tokens) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e.message);
            exit(-1)
        }
    };
    let mut tape = Tape::new();
    execute(ast, &mut tape);
}
