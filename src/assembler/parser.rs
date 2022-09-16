use core::panicking::panic;
use std::borrow::Borrow;

use logos::Lexer;
use crate::{Token, machine::{VmState, VmValue, VmString}};
use crate::machine::{Instruction, InstructionArg, OpCode};

enum Production
{
    FILE,
    STATEMENTS,
    STATEMENT,
    S_AWAIT,
    S_ABORT,
    S_EXIT,
    S_START,
    AWAIT,
    AWAIT_ANY,
    AWAIT_ALL,
    AWAIT_CALL,
    AWAIT_IDENT,
    CALL,
    VALUE,
    CONSTANT,
    RANGE,
    ARRAY,
    ARRAY_DATA,
    OBJ,
    OBJ_DATA,
    OBJ_PROP,
    ABORT,
    EXIT,
    IF_ELSE,
    IF,
    ELSE,
    CODE,
    FOR,
    FOR_VALUE,
    FOR_AWAIT_CALL,
    FOR_AWAIT_IDENT,
    FOR_IDENT,
    ASSIGNMENT,
    START,
}

pub struct ParseState {
    is_good: bool,
    errors: Vec<ParseError>,
}

pub struct ParseError {
    message: String,
}
pub fn parse(mut lexer: Lexer<Token>) -> Result<VmState, ParseState>
{
    let mut vm_state = VmState {
        function_list: Vec::new(),
        instruction_index: 0,
        instructions: Vec::new(),
        value_list: Vec::new(),
    };
    let mut state = ParseState {
        statement: Statement::File,
    };
    while let Some(tok) = lexer.next() {
        match state.statement {
            Statement::File => {
                match tok {
                    Token::Identifier => {
                        state.statement = Statement::AssignmentEqual;
                        push_string(vm_state, lexer.slice().to_string());
                    }
                    Token::If => {
                        state.statement = Statement::IfStart;
                    }
                    Token::For => {
                        state.statement = Statement::ForStart;
                    }
                    Token::Exit => {
                        state.statement = Statement::ExitStart;
                    }
                    Token::Abort => {
                        state.statement = Statement::AbortStart;
                    }
                    Token::Await => {
                        state.statement = Statement::AwaitStart;
                    }
                    _ => return Err(state),
                };
            }
            _ => panic!("Unmatched token, program is illformed")
        }
    }
    return Ok(vm_state);
}

fn prod_file(mut iter: std::iter::Peekable<Lexer<Token>>, mut vm_state: VmState, mut parse_state: ParseState){
    match iter.peek() {
        Some(_) => prod_statements(iter, vm_state, parse_state),
        None => return,
    }
}

fn prod_statements(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, mut parse_state: ParseState) {
    while let Some(_) = iter.peek() {
        prod_statement(iter, vm_state, parse_state);
    }
}

fn prod_statement(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, mut parse_state: ParseState) {
    match iter.peek() {
        Some(Token::Await) => prod_s_await(iter, vm_state, parse_state),
        Some(Token::Abort) => prod_s_abort(iter, vm_state, parse_state),
        Some(Token::Exit) => prod_s_exit(iter, vm_state, parse_state),
        Some(Token::Start) => prod_s_start(iter, vm_state, parse_state),
        Some(Token::If) => prod_if(iter, vm_state, parse_state),
        Some(Token::For) => prod_for(iter, vm_state, parse_state),
        Some(Token::Identifier) => prod_assignment(iter, vm_state, parse_state),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
}

fn prod_assignment(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, parse_state: ParseState) {
    todo!()
}

fn prod_for(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, parse_state: ParseState) {
    todo!()
}

fn prod_if(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, parse_state: ParseState) {
    todo!()
}

fn prod_s_exit(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, parse_state: ParseState) {
    match iter.peek() {
        Some(Token::Exit) => prod_exit(iter, vm_state, parse_state),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
    match iter.peek() {
        Some(Token::Semicolon) => _ = iter.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
}

fn prod_s_start(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, parse_state: ParseState) {
    match iter.peek() {
        Some(Token::Start) => prod_start(iter, vm_state, parse_state),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
    match iter.peek() {
        Some(Token::Semicolon) => _ = iter.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
}

fn prod_s_await(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, parse_state: ParseState) {
    match iter.peek() {
        Some(Token::Await) => prod_await(iter, vm_state, parse_state),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
    match iter.peek() {
        Some(Token::Semicolon) => _ = iter.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
}

fn prod_s_abort(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, parse_state: ParseState) {
    match iter.peek() {
        Some(Token::Abort) => prod_abort(iter, vm_state, parse_state),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
    match iter.peek() {
        Some(Token::Semicolon) => _ = iter.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
}

fn prod_exit(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, parse_state: ParseState) {
    todo!()
}

fn prod_start(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, parse_state: ParseState) {
    todo!()
}

fn prod_await(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, parse_state: ParseState) {
    // ToDo: modify BNF file to move await from AWAIT_* into AWAIT --> be LR(1) not LR(N)
    
    match iter.peek() {
        Some(Token::Await) => _ = iter.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
    match iter.peek() {
        Some(Token::Await) => prod_s_await(iter, vm_state, parse_state),
        Some(Token::Abort) => prod_s_abort(iter, vm_state, parse_state),
        Some(Token::Exit) => prod_s_exit(iter, vm_state, parse_state),
        Some(Token::Start) => prod_s_start(iter, vm_state, parse_state),
        Some(Token::If) => prod_if(iter, vm_state, parse_state),
        Some(Token::For) => prod_for(iter, vm_state, parse_state),
        Some(Token::Identifier) => prod_assignment(iter, vm_state, parse_state),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
}

fn prod_abort(iter: std::iter::Peekable<Lexer<Token>>, vm_state: VmState, parse_state: ParseState) {
    todo!()
}

fn error_token(parse_state: ParseState) {
    parse_state.is_good = false;
    parse_state.errors.push(ParseError {
        message: "Unmatched token".to_string(),
    });
}

fn push_value_string(mut vm_state: VmState, s: String)
{
    let vm_string = VmString(s);
    let vm_value = VmValue::String(vm_string);
    let data = Some(vm_value);
    let index = vm_state.value_list.len();
    vm_state.value_list.push(data);
    vm_state.instructions.push(Instruction {
        opcode: OpCode::PushValueU16,
        arg: InstructionArg {
            unsigned: index as u16
        },
    })
}