//! This file has an enum for all the Lexer Tokens

use super::opcodes;

pub enum Token {
    Op(opcodes::Opcode),
}

impl Token {
    pub fn from_opcode(op: opcodes::Opcode) -> Self {
        Token::Op(op)
    }
}
