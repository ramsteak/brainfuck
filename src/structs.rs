#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    INC, // + Increment the current value
    DEC, // - Decrement the current value
    MLT, // < Move left in the memory array
    MRT, // > Move right in the memory array
    LEN, // [ Start a loop section
    LEX, // ] End a loop section
    OUT, // , Output the current value as character
    INP, // . Input a character as a value
    END, // & Immediately exits the program
}
#[derive(Debug)]
pub enum BFErrorCode {
    UnmatchedLoopExit,
    UnmatchedLoopEnter,
    KeyboardInterrupt,
    Exit,
}

#[derive(Debug)]
pub struct BFError {
    pub code: BFErrorCode,
    pub message: String,
}

impl BFError {
    pub fn new(code: BFErrorCode, message: String) -> Self {
        BFError { code, message }
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    INC,
    DEC,
    MRT,
    MLT,
    OUT,
    INP,
    END,
    LOP(Vec<AstNode>),
}

/// The struct represents the memory of a brainfuck program
pub struct Tape {
    cell: usize,
    tape: Vec<u8>,
}
impl Tape {
    pub fn new() -> Self {
        Tape {
            cell: 0,
            tape: vec![0],
        }
    }
    /// Moves the tape head one space to the right
    pub fn mrt(&mut self) {
        self.cell += 1;
        if self.cell == self.tape.len() {
            self.tape.push(0)
        }
    }
    /// Moves the tape head one space to the left
    pub fn mlt(&mut self) {
        if self.cell > 0 {
            self.cell -= 1
        };
    }
    /// Returns the value in the tape head cell
    pub fn get(&self) -> u8 {
        self.tape[self.cell]
    }
    /// Sets the value in the tape head cell
    pub fn set(&mut self, val: u8) {
        self.tape[self.cell] = val
    }
    /// Adds one to the value in the tape head cell, wrapping on overflow
    pub fn add(&mut self) {
        self.tape[self.cell] = self.tape[self.cell].wrapping_add(1)
    }
    /// Removes one to the value in the tape head cell, wrapping on underflow
    pub fn sub(&mut self) {
        self.tape[self.cell] = self.tape[self.cell].wrapping_sub(1)
    }
}
