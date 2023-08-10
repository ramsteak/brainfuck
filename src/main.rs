mod structs;

extern crate clap;
extern crate crossterm;

use structs::{AstNode, BFError, BFErrorCode, Tape, Token};

use crossterm::{
    event::{self, Event},
    terminal,
};

use clap::{arg, Command};

use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;

/// Takes a string representing the code and generates a vec of tokens,
/// representing the program.
fn tokenize(code: String, comments: bool) -> Result<Vec<Token>, BFError> {
    let mut tokens = Vec::<Token>::new();
    let mut comment = false;

    for char in code.chars() {
        match (char, &comment, comments) {
            ('+', false, _) => tokens.push(Token::INC),
            ('-', false, _) => tokens.push(Token::DEC),
            ('<', false, _) => tokens.push(Token::MLT),
            ('>', false, _) => tokens.push(Token::MRT),
            ('[', false, _) => tokens.push(Token::LEN),
            (']', false, _) => tokens.push(Token::LEX),
            (',', false, _) => tokens.push(Token::INP),
            ('.', false, _) => tokens.push(Token::OUT),
            ('&', false, _) => tokens.push(Token::END),
            ('?', false, _) => tokens.push(Token::DBG),
            ('#', _, true) => comment = true,
            ('\n', true, true) => comment = false,
            _ => (),
        };
    }

    Ok(tokens)
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
                Token::END => ast.push(AstNode::END),
                Token::DBG => ast.push(AstNode::DBG),
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

/// Executes a brainfuck AST, given the memory tape
fn execute(ast: Vec<AstNode>, tape: &mut Tape) -> Result<(), BFError> {
    terminal::enable_raw_mode().unwrap();
    for node in ast {
        match node {
            AstNode::INC => tape.add(),
            AstNode::DEC => tape.sub(),
            AstNode::MRT => tape.mrt(),
            AstNode::MLT => tape.mlt(),
            AstNode::DBG => println!("{:?}", tape),
            AstNode::END => {
                return Err(BFError {
                    code: BFErrorCode::Exit,
                    message: format!("Program exited with code {}", tape.get()),
                })
            }
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
    let command = Command::new("brainfuck").version("0.1.0")
        .arg(arg![path: [path] "The path of the .bf file.\nIf absent, reads from stdin to EOF (Ctrl-D / Ctrl-Z)"],)
        .arg(arg![comments: -c --comments "Line comments start with # and end at a newline"]);

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
    let comments = if let Some(comments) = matches.get_one::<bool>("comments") {
        comments
    } else {
        &false
    };
    let tokens = match tokenize(code, comments.clone()) {
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

    match execute(ast, &mut tape) {
        Ok(()) => (),
        Err(e) => eprintln!("\n{}", e.message),
    };
}
