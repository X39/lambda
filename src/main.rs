#![allow(dead_code)]

mod machine;
mod assembler;

use crate::assembler::Token;
use logos::Logos;

fn main() {
    let mut lexer = Token::lexer("");
}
