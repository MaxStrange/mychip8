//! This module exposes a lex() function, which returns an instance of a Lexer struct.

use std::io;
use super::opcodes;
use super::tokens;

struct Lattice {
    /// The current possibilities
    options: Vec<tokens::Token>,

    // All the characters in the lattice so far
    state: Vec<char>,
}

pub struct Lexer {
    /// The lattice; i.e., what our current possiblities are
    lattice: Lattice,
    /// The input stream, tokenized into an output stream
    output_tokens: Vec<tokens::Token>,
}

impl Lexer {
    /// This function is private. Get a new Lexer by calling Lexer::lex(input_stream).
    fn new() -> Self {
        Lexer{
            lattice: Lattice{options: Vec::new(), state: Vec::new()},
            output_tokens: Vec::<tokens::Token>::new(),
        }
    }

    /// Lexes the input stream into a Lexer struct.
    pub fn lex<T: io::BufRead>(stream: T) -> Result<Self, String> {
        let mut ldata = Lexer::new();

        for maybeline in stream.lines() {
            let line = maybeline.expect("Could not get line");
            if let Err(e) = ldata.lex_line(line) {
                return Err(e);
            }
        }

        Ok(ldata)
    }

    fn lex_line(&mut self, line: String) -> Result<(), String> {
        for ch in line.chars() {
            // attempt to lex it as an opcode
            if let Some(opcode) = self.try_opcode(ch) {
                self.add_token(tokens::Token::from_opcode(opcode));
                continue;
            }

            // etc.

            // If we haven't matched this character, it's an error
            return Err("TODO: Fill this string with a useful error message!".to_string());
        }

        Ok(())
    }

    /// Add the given token to the output token stream.
    fn add_token(&mut self, t: tokens::Token) {
        self.output_tokens.push(t);
    }

    /// Try to parse `ch` as part of an Opcode, given the present state of the Lexer.
    fn try_opcode(&mut self, ch: char) -> Option<opcodes::Opcode> {
        // TODO
        None
    }
}
