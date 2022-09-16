#![allow(dead_code)]

extern crate core;

mod machine;
mod assembler;

use crate::assembler::Token;
use logos::{Lexer, Logos};

fn main() {
    let mut lexer: Lexer<Token> = Token::lexer("");

}
