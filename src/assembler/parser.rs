use std::{borrow::{Borrow, BorrowMut}, ops::Deref};

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
    AWAIT_CALL_OR_IDENT,
    CALL,
    VALUE,
    CONSTANT,
    NUMERIC,
    ARRAY,
    ARRAY_DATA,
    OBJ,
    OBJ_DATA,
    OBJ_PROP,
    ABORT,
    EXIT,
    IF_ELSE,
    IF,
    IF_PART,
    ELSE,
    ELSE_PART,
    CODE,
    FOR,
    FOR_VARIANT,
    FOR_VALUE,
    FOR_AWAIT_CALL,
    FOR_AWAIT_IDENT,
    FOR_IDENT,
    ASSIGNMENT,
    ASSIGNMENT_VALUE,
    START,
}

struct ParseState<'a> {
    is_good: bool,
    errors: &'a Vec<ParseError<'a>>,
    peekable: &'a std::iter::Peekable<Lexer<'a, Token>>,
    vm: &'a VmState,
}

pub struct ParseError<'a> {
    message: &'a String,
}
pub fn parse(mut lexer: Lexer<Token>) -> Result<Vec<ProdStatement>, Vec<ParseError>>
{
    let mut vm_state = VmState {
        function_list: Vec::new(),
        instruction_index: 0,
        instructions: Vec::new(),
        value_list: Vec::new(),
    };
    let mut errors = Vec::new();
    let mut peekable = lexer.peekable();

    let mut state = ParseState {
        is_good: true,
        errors: errors.borrow_mut(),
        peekable: peekable.borrow_mut(),
        vm: vm_state.borrow_mut(),
    };
    let result = prod_file(state.borrow_mut());
    if state.errors.len() > 0 {
        return Err(errors);
    }
    return Ok(result);
}

fn prod_file<'a>(mut parse_state: &ParseState<'a>) -> Vec<ProdStatement<'a>> {
    match parse_state.peekable.peek() {
        Some(_) => prod_statements(parse_state),
        None => Vec::new(),
    }
}

fn prod_statements<'a>(mut parse_state: &ParseState) -> Vec<ProdStatement<'a>> {
    let vec = Vec::new();
    while let Some(_) = parse_state.peekable.peek() {
        let result = prod_statement(parse_state);
        if result.is_none() {
            return Vec::new();
        }
        vec.push(result.unwrap());
    }
    return vec;
}
pub enum ProdStatement<'a> {
    ProdAwait(ProdAwait<'a>),
    ProdAbort(ProdAbort<'a>),
    ProdExit(ProdExit),
    ProdStart(ProdStart<'a>),
    ProdIfElse(ProdIfElse<'a>),
    ProdFor(ProdFor<'a>),
    ProdAssignment(ProdAssignment<'a>),
}
fn prod_statement<'a>(mut parse_state: &'a ParseState<'a>) -> Option<ProdStatement<'a>> {
    enum Tmp<'a> {
        ProdAwait(Option<ProdAwait<'a>>),
        ProdAbort(Option<ProdAbort<'a>>),
        ProdExit(Option<ProdExit>),
        ProdStart(Option<ProdStart<'a>>),
        ProdIfElse(Option<ProdIfElse<'a>>),
        ProdFor(Option<ProdFor<'a>>),
        ProdAssignment(Option<ProdAssignment<'a>>),
    }
    let result = match parse_state.peekable.peek() {
        Some(Token::Await) => Tmp::ProdAwait(prod_s_await(parse_state)),
        Some(Token::Abort) => Tmp::ProdAbort(prod_s_abort(parse_state)),
        Some(Token::Exit) => Tmp::ProdExit(prod_s_exit(parse_state)),
        Some(Token::Start) => Tmp::ProdStart(prod_s_start(parse_state)),
        Some(Token::If) => Tmp::ProdIfElse(prod_if_else(parse_state)),
        Some(Token::For) => Tmp::ProdFor(prod_for(parse_state)),
        Some(Token::Identifier(s)) => Tmp::ProdAssignment(prod_assignment(parse_state)),
        None => panic!("statement reached none, indicating invalid program"),
        _ => {error_token(parse_state); return None;},
    };
    return match result {
        Tmp::ProdAwait(optProdAwait) => match optProdAwait {
            Some(prodAwait) => return Some(ProdStatement::ProdAwait(prodAwait)),
            _ => return None,
        },
        Tmp::ProdAbort(optProdAbort) => match optProdAbort {
            Some(prodAbort) => return Some(ProdStatement::ProdAbort(prodAbort)),
            _ => return None,
        },
        Tmp::ProdExit(optProdExit) => match optProdExit {
            Some(prodExit) => return Some(ProdStatement::ProdExit(prodExit)),
            _ => return None,
        },
        Tmp::ProdStart(optProdStart) => match optProdStart {
            Some(prodStart) => return Some(ProdStatement::ProdStart(prodStart)),
            _ => return None,
        },
        Tmp::ProdIfElse(optProdIf) => match optProdIf {
            Some(prodIf) => return Some(ProdStatement::ProdIfElse(prodIf)),
            _ => return None,
        },
        Tmp::ProdFor(optProdFor) => match optProdFor {
            Some(prodFor) => return Some(ProdStatement::ProdFor(prodFor)),
            _ => return None,
        },
        Tmp::ProdAssignment(optProdAssignment) => match optProdAssignment {
            Some(prodAssignment) => return Some(ProdStatement::ProdAssignment(prodAssignment)),
            _ => return None,
        },
        _ => panic!("Invalid program")
    };
}

pub struct ProdAssignment<'a> {
    identifier: &'a String,
    value: ProdAssignmentValue<'a>,
}
fn prod_assignment<'a>(mut parse_state: &ParseState) -> Option<ProdAssignment<'a>> {
    let ident = match parse_state.peekable.peek() {
        Some(Token::Identifier(s)) => {parse_state.peekable.next(); s},
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    match parse_state.peekable.peek() {
        Some(Token::Identifier(s)) => {parse_state.peekable.next(); s},
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    let assignment = match parse_state.peekable.peek() {
        Some(Token::Await) => prod_assignment_value(parse_state),
        Some(Token::CurlyOpen) => prod_assignment_value(parse_state),
        Some(Token::False) => prod_assignment_value(parse_state),
        Some(Token::True) => prod_assignment_value(parse_state),
        Some(Token::Null) => prod_assignment_value(parse_state),
        Some(Token::Number(d)) => prod_assignment_value(parse_state),
        Some(Token::SquareOpen) => prod_assignment_value(parse_state),
        Some(Token::Start) => prod_assignment_value(parse_state),
        Some(Token::String(s)) => prod_assignment_value(parse_state),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    if assignment.is_none() {
        return None;
    }
    
    return Some(ProdAssignment {
        identifier: ident,
        value: assignment.unwrap(),
    });
}

pub enum ProdAssignmentValue<'a> {
    ProdAwait(ProdAwait<'a>),
    ProdValue(ProdValue<'a>),
    ProdStart(ProdStart<'a>),
}
fn prod_assignment_value<'a>(mut parse_state: &ParseState) -> Option<ProdAssignmentValue<'a>> {
    enum Tmp<'a> {
        ProdAwait(Option<ProdAwait<'a>>),
        ProdValue(Option<ProdValue<'a>>),
        ProdStart(Option<ProdStart<'a>>),
    }
    let tmp = match parse_state.peekable.peek() {
        Some(Token::Await) => Tmp::ProdAwait(prod_await(parse_state)),
        Some(Token::Start) => Tmp::ProdStart(prod_start(parse_state)),
        Some(Token::CurlyOpen) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::False) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::True) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::Null) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::Number(d)) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::SquareOpen) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::String(s)) => Tmp::ProdValue(prod_value(parse_state)),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    match tmp {
        Tmp::ProdAwait(optProdAwait) => match optProdAwait {
            Some(prodAwait) => return Some(ProdAssignmentValue::ProdAwait(prodAwait)),
            _ => return None,
        },
        Tmp::ProdValue(optProdValue) => match optProdValue {
            Some(prodValue) => return Some(ProdAssignmentValue::ProdValue(prodValue)),
            _ => return None,
        },
        Tmp::ProdStart(optProdStart) => match optProdStart {
            Some(prodStart) => return Some(ProdAssignmentValue::ProdStart(prodStart)),
            _ => return None,
        },
    };
}

pub enum ProdStart<'a> {
    ProdCall(ProdCall<'a>),
}
fn prod_start<'a>(mut parse_state: &ParseState) -> Option<ProdStart<'a>> {
    match parse_state.peekable.peek() {
        Some(Token::Start) => {parse_state.peekable.next();},
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    let call = match parse_state.peekable.peek() {
        Some(Token::Identifier(s)) => prod_call(parse_state),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    if call.is_none() {
        return None;
    }
    return Some(ProdStart::ProdCall(call.unwrap()));
}

pub struct ProdCall<'a> {
    ident: &'a String,
    value: ProdValue<'a>,
}
fn prod_call<'a>(mut parse_state: &ParseState) -> Option<ProdCall<'a>> {
    let ident = match parse_state.peekable.peek() {
        Some(Token::Identifier(s)) => {parse_state.peekable.next(); s},
        None => panic!("Invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    match parse_state.peekable.peek() {
        Some(Token::RoundOpen) => parse_state.peekable.next(),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    let value = match parse_state.peekable.peek() {
        Some(Token::CurlyOpen) => prod_value(parse_state),
        Some(Token::False) => prod_value(parse_state),
        Some(Token::True) => prod_value(parse_state),
        Some(Token::Null) => prod_value(parse_state),
        Some(Token::Number(d)) => prod_value(parse_state),
        Some(Token::SquareOpen) => prod_value(parse_state),
        Some(Token::String(s)) => prod_value(parse_state),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    if value.is_none() {
        return None;
    }
    match parse_state.peekable.peek() {
        Some(Token::RoundClose) => parse_state.peekable.next(),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    return Some(ProdCall {
        ident: ident,
        value: value.unwrap(),
    });
}

pub enum ProdValue<'a> {
    ProdObj(ProdObj<'a>),
    ProdArray(ProdArray<'a>),
    ProdNumeric(ProdNumeric),
    ProdConstant(ProdConstant<'a>),
}
fn prod_value<'a>(mut parse_state: &ParseState) -> Option<ProdValue<'a>> {
    enum Tmp<'a> {
        ProdObj(Option<ProdObj<'a>>),
        ProdArray(Option<ProdArray<'a>>),
        ProdNumeric(Option<ProdNumeric>),
        ProdConstant(Option<ProdConstant<'a>>),
    }
    let value = match parse_state.peekable.peek() {
        Some(Token::CurlyOpen) => Tmp::ProdObj(prod_obj(parse_state)),
        Some(Token::False) => Tmp::ProdConstant(prod_constant(parse_state)),
        Some(Token::True) => Tmp::ProdConstant(prod_constant(parse_state)),
        Some(Token::Null) => Tmp::ProdConstant(prod_constant(parse_state)),
        Some(Token::Number(d)) => Tmp::ProdNumeric(prod_numeric(parse_state)),
        Some(Token::SquareOpen) => Tmp::ProdArray(prod_array(parse_state)),
        Some(Token::String(s)) => Tmp::ProdConstant(prod_constant(parse_state)),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    match value {
        Tmp::ProdObj(optObj) => match optObj {
            Some(obj) => return Some(ProdValue::ProdObj(obj)),
            None => return None,
        },
        Tmp::ProdArray(optArray) => match optArray {
            Some(array) => return Some(ProdValue::ProdArray(array)),
            None => return None,
        },
        Tmp::ProdNumeric(optNumeric) => match optNumeric {
            Some(numeric) => return Some(ProdValue::ProdNumeric(numeric)),
            None => return None,
        },
        Tmp::ProdConstant(optConstant) => match optConstant {
            Some(constant) => return Some(ProdValue::ProdConstant(constant)),
            None => return None,
        },
        _ => panic!("Invalid value matched, program is invalid"),
    };
}

pub struct ProdArray<'a> {
    values: Vec<ProdArrayDataEnum<'a>>,
}
fn prod_array<'a>(mut parse_state: &ParseState) -> Option<ProdArray<'a>> {
    match parse_state.peekable.peek() {
        Some(Token::SquareOpen) => parse_state.peekable.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    let arrayData = match parse_state.peekable.peek() {
        Some(Token::SquareClose) => return Some(ProdArray {
            values: Vec::new(),
        }),
        Some(Token::Identifier(s)) => prod_array_data(parse_state),
        Some(Token::Null) => prod_array_data(parse_state),
        Some(Token::False) => prod_array_data(parse_state),
        Some(Token::True) => prod_array_data(parse_state),
        Some(Token::String(s)) => prod_array_data(parse_state),
        Some(Token::Number(d)) => prod_array_data(parse_state),
        Some(Token::SquareOpen) => prod_array_data(parse_state),
        Some(Token::CurlyOpen) => prod_array_data(parse_state),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    if arrayData.is_none() {
        return None;
    }
    match parse_state.peekable.peek() {
        Some(Token::SquareClose) => {}
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    return Some(ProdArray {
        values: arrayData.unwrap().values
    });
}

pub struct ProdArrayData<'a> {
    values: Vec<ProdArrayDataEnum<'a>>,
}
pub enum ProdArrayDataEnum<'a> {
    Identifier(&'a String),
    ProdValue(ProdValue<'a>),
}
fn prod_array_data<'a>(mut parse_state: &ParseState) -> Option<ProdArrayData<'a>> {
    enum Tmp<'a> {
        Identifier(&'a String),
        ProdValue(Option<ProdValue<'a>>),
    }
    let tmp = match parse_state.peekable.peek() {
        Some(Token::Identifier(s)) => {parse_state.peekable.next(); Tmp::Identifier(s)},
        Some(Token::String(s)) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::Null) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::False) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::True) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::Number(d)) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::SquareOpen) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::CurlyOpen) => Tmp::ProdValue(prod_value(parse_state)),
        None => panic!("Invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    if match tmp {
        Tmp::Identifier(_) => false,
        Tmp::ProdValue(prodValue) => prodValue.is_none(),
        _ => panic!("Invalid program"),
    } {
        return None;
    }
    match parse_state.peekable.peek() {
        Some(Token::Comma) => {},
        _ => {
            let vec: Vec<ProdArrayDataEnum> = Vec::new();
            vec.push(match tmp {
                Tmp::Identifier(ident) => ProdArrayDataEnum::Identifier(ident),
                Tmp::ProdValue(prodValue) => ProdArrayDataEnum::ProdValue(prodValue.unwrap()),
                _ => panic!("Invalid program"),
            });
            return Some(ProdArrayData {
                values: vec,
            });
        },
    };
    let prodArrData = match parse_state.peekable.peek() {
        Some(Token::Identifier(s)) => prod_array_data(parse_state),
        Some(Token::String(s)) => prod_array_data(parse_state),
        Some(Token::Null) => prod_array_data(parse_state),
        Some(Token::False) => prod_array_data(parse_state),
        Some(Token::True) => prod_array_data(parse_state),
        Some(Token::Number(d)) => prod_array_data(parse_state),
        Some(Token::SquareOpen) => prod_array_data(parse_state),
        Some(Token::CurlyOpen) => prod_array_data(parse_state),
        None => {
            let vec: Vec<ProdArrayDataEnum> = Vec::new();
            vec.push(match tmp {
                Tmp::Identifier(ident) => ProdArrayDataEnum::Identifier(ident),
                Tmp::ProdValue(prodValue) => ProdArrayDataEnum::ProdValue(prodValue.unwrap()),
                _ => panic!("Invalid program"),
            });
            return Some(ProdArrayData {
                values: vec,
            });
        },
        tok => {error_token(parse_state); return None;},
    };
    if prodArrData.is_none() {
        return None;
    }
    let vec: Vec<ProdArrayDataEnum> = Vec::new();
    vec.push(match tmp {
        Tmp::Identifier(ident) => ProdArrayDataEnum::Identifier(ident),
        Tmp::ProdValue(prodValue) => ProdArrayDataEnum::ProdValue(prodValue.unwrap()),
        _ => panic!("Invalid program"),
    });
    for it in prodArrData.unwrap().values {
        vec.push(it);
    }
    return Some(ProdArrayData {
        values: vec,
    });
}

pub struct ProdNumeric {
    value: f64,
    end: Option<f64>,
}
fn prod_numeric(mut parse_state: &ParseState) -> Option<ProdNumeric> {
    let start = *match parse_state.peekable.peek() {
        Some(Token::Number(d)) => {parse_state.peekable.next(); d},
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };

    match parse_state.peekable.peek() {
        Some(Token::DotDot) => parse_state.peekable.next(),
        None => {error_eof(parse_state); return None},
        tok => {return Some(ProdNumeric {
             value: start,
             end: None
            })
        },
    };
    
    let end = *match parse_state.peekable.peek() {
        Some(Token::Number(d)) => {parse_state.peekable.next(); d},
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    return Some(ProdNumeric {
        value: start,
        end: Some(end),
    });
}

pub enum ProdConstant<'a> {
    Null,
    Boolean(bool),
    String(&'a String),
}
fn prod_constant<'a>(mut parse_state: &ParseState) -> Option<ProdConstant<'a>> {
    // null | string | true | false
    let value = match parse_state.peekable.peek() {
        Some(Token::Null) => ProdConstant::Null,
        Some(Token::String(s)) => {parse_state.peekable.next(); ProdConstant::String(s)},
        Some(Token::True) => {parse_state.peekable.next(); ProdConstant::Boolean(false)},
        Some(Token::False) => {parse_state.peekable.next(); ProdConstant::Boolean(false)},
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    return Some(value);
}

pub struct ProdObj<'a> {
    properties: Vec<ProdObjProp<'a>>,
}
fn prod_obj<'a>(mut parse_state: &ParseState) -> Option<ProdObj<'a>> {
    match parse_state.peekable.peek() {
        Some(Token::CurlyOpen) => parse_state.peekable.next(),
        None => panic!("statement reached none, indicating invalid program"),
        _ => {error_token(parse_state); return None;},
    };
    let objData = match parse_state.peekable.peek() {
        Some(Token::CurlyClose) => return Some(ProdObj {
            properties: Vec::new(),
        }),
        Some(Token::String(s)) => prod_obj_data(parse_state),
        None => {error_eof(parse_state); return None;},
        _ => {error_token(parse_state); return None;},
    };
    if objData.is_none() {
        return None;
    }
    match parse_state.peekable.peek() {
        Some(Token::CurlyClose) => {}
        None => {error_eof(parse_state); return None;},
        _ => {error_token(parse_state); return None;},
    };
    return Some(ProdObj {
         properties: objData.unwrap().properties
    });
}

pub struct ProdObjData<'a> {
    properties: Vec<ProdObjProp<'a>>,
}
fn prod_obj_data<'a>(mut parse_state: &ParseState) -> Option<ProdObjData<'a>> {
    let prodObjProp = match parse_state.peekable.peek() {
        Some(Token::String(s)) => prod_obj_prop(parse_state),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    if prodObjProp.is_none() {
        return None;
    }
    match parse_state.peekable.peek() {
        Some(Token::Comma) => {},
        None => {
            let vec: Vec<ProdObjProp> = Vec::new();
            vec.push(prodObjProp.unwrap());
            return Some(ProdObjData {
                properties: vec,
            });
        },
        tok => {error_token(parse_state); return None;},
    };
    let prodObjData = match parse_state.peekable.peek() {
        Some(Token::String(s)) => prod_obj_data(parse_state),
        None => {
            let vec: Vec<ProdObjProp> = Vec::new();
            vec.push(prodObjProp.unwrap());
            return Some(ProdObjData {
                properties: vec,
            });
        },
        tok => {error_token(parse_state); return None;},
    };
    if prodObjData.is_none() {
        return None;
    }
    let vec: Vec<ProdObjProp> = Vec::new();
    vec.push(prodObjProp.unwrap());
    for it in prodObjData.unwrap().properties {
        vec.push(it);
    }
    return Some(ProdObjData {
         properties: vec
     });
}

pub struct ProdObjProp<'a> {
    label: &'a String,
    value: ProdObjPropData<'a>,
}
pub enum ProdObjPropData<'a> {
    String(&'a String),
    ProdValue(ProdValue<'a>),
}
fn prod_obj_prop<'a>(mut parse_state: &ParseState) -> Option<ProdObjProp<'a>> {
    enum Tmp<'a> {
        String(&'a String),
        ProdValue(Option<ProdValue<'a>>),
    }
    let label = match parse_state.peekable.peek() {
        Some(Token::String(s)) => {parse_state.peekable.next(); s},
        None => panic!("Invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    match parse_state.peekable.peek() {
        Some(Token::Colon) => parse_state.peekable.next(),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    let value = match parse_state.peekable.peek() {
        Some(Token::Identifier(s)) => {parse_state.peekable.next(); Tmp::String(s)},
        Some(Token::False) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::False) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::True) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::Null) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::Number(d)) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::SquareOpen) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::String(s)) => Tmp::ProdValue(prod_value(parse_state)),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    match value {
        Tmp::String(string) => return Some(ProdObjProp {
            label: label,
            value: ProdObjPropData::String(string),
        }),
        Tmp::ProdValue(optProdValue) => match optProdValue {
            Some(prodValue) =>return Some(ProdObjProp {
                label: label,
                value: ProdObjPropData::ProdValue(prodValue),
            }), 
            _ => return None,
        },
        _ => panic!("invalid program"),
    }
}

fn prod_s_exit(mut parse_state: &ParseState) -> Option<ProdExit> {
    let val = match parse_state.peekable.peek() {
        Some(Token::Exit) => prod_exit(parse_state),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    if val.is_none(){
        return None;
    }
    match parse_state.peekable.peek() {
        Some(Token::Semicolon) => {let _ = parse_state.peekable.next();},
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    }
    return val;
}

fn prod_s_start<'a>(mut parse_state: &ParseState) -> Option<ProdStart<'a>> {
    let val = match parse_state.peekable.peek() {
        Some(Token::Start) => prod_start(parse_state),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    if val.is_none(){
        return None;
    }
    match parse_state.peekable.peek() {
        Some(Token::Semicolon) => {let _ = parse_state.peekable.next();},
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    return val;
}

fn prod_s_await<'a>(mut parse_state: &ParseState) -> Option<ProdAwait<'a>> {
    let val = match parse_state.peekable.peek() {
        Some(Token::Await) => prod_await(parse_state),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    if val.is_none(){
        return None;
    }
    match parse_state.peekable.peek() {
        Some(Token::Semicolon) => {let _ = parse_state.peekable.next();},
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    return val;
}

fn prod_s_abort<'a>(mut parse_state: &ParseState) -> Option<ProdAbort<'a>> {
    let val = match parse_state.peekable.peek() {
        Some(Token::Abort) => prod_abort(parse_state),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    if val.is_none(){
        return None;
    }
    match parse_state.peekable.peek() {
        Some(Token::Semicolon) => {let _ = parse_state.peekable.next();},
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    }
    return val;
}

pub struct ProdExit {

}
fn prod_exit(mut parse_state: &ParseState) -> Option<ProdExit> {
    match parse_state.peekable.peek() {
        Some(Token::Exit) => prod_abort(parse_state),
        None => panic!("Invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    match parse_state.peekable.peek() {
        Some(Token::Semicolon) => _ = parse_state.peekable.next(),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    return Some(ProdExit {  });
}

pub enum ProdAwait<'a> {
    ProdAwaitAny(ProdAwaitAny<'a>),
    ProdAwaitAll(ProdAwaitAll<'a>),
    ProdAwaitCallOrIdent(ProdAwaitCallOrIdent<'a>),
}
fn prod_await<'a>(mut parse_state: &ParseState) -> Option<ProdAwait<'a>>{
    enum Tmp<'a> {
        ProdAwaitAny(Option<ProdAwaitAny<'a>>),
        ProdAwaitAll(Option<ProdAwaitAll<'a>>),
        ProdAwaitCallOrIdent(Option<ProdAwaitCallOrIdent<'a>>),
    }
    match parse_state.peekable.peek() {
        Some(Token::Await) => _ = parse_state.peekable.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
    let tmp = match parse_state.peekable.peek() {
        Some(Token::All) => Tmp::ProdAwaitAll(prod_await_all(parse_state)),
        Some(Token::Any) => Tmp::ProdAwaitAny(prod_await_any(parse_state)),
        Some(Token::Identifier(s)) => Tmp::ProdAwaitCallOrIdent(prod_await_call_or_ident(parse_state)),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None},
    };
    match tmp {
        Tmp::ProdAwaitAny(optProdAwaitAny) => match optProdAwaitAny {
            Some(prodAwaitAny) => return Some(ProdAwait::ProdAwaitAny(prodAwaitAny)),
            None => return None,
        }
        Tmp::ProdAwaitAll(optProdAwaitAll) => match optProdAwaitAll {
            Some(prodAwaitAll) => return Some(ProdAwait::ProdAwaitAll(prodAwaitAll)),
            None => return None,
        }
        Tmp::ProdAwaitCallOrIdent(optProdAwaitCallOrIdent) => match optProdAwaitCallOrIdent {
            Some(prodAwaitCallOrIdent) => return Some(ProdAwait::ProdAwaitCallOrIdent(prodAwaitCallOrIdent)),
            None => return None,
        }
    };
}

pub struct ProdAwaitAll<'a> {
    identifier: &'a String,
}
fn prod_await_all<'a>(mut parse_state: &ParseState) -> Option<ProdAwaitAll<'a>> {
    let identifier = match parse_state.peekable.peek() {
        Some(Token::Identifier(s)) => {parse_state.peekable.next(); s},
        None => panic!("Invalid program."),
        tok => {error_token(parse_state); return None;},
    };
    return Some(ProdAwaitAll {
        identifier: identifier,
    });
}

pub struct ProdAwaitAny<'a> {
    identifier: &'a String,
}
fn prod_await_any<'a>(mut parse_state: &ParseState) -> Option<ProdAwaitAny<'a>> {
    let identifier = match parse_state.peekable.peek() {
        Some(Token::Identifier(s)) => {parse_state.peekable.next(); s},
        None => panic!("Invalid program."),
        tok => {error_token(parse_state); return None;},
    };
    return Some(ProdAwaitAny {
        identifier: identifier,
    });
}
pub struct ProdAwaitCallOrIdent<'a> {
    identifier: &'a String,
    value: Option<ProdValue<'a>>,
    is_func: bool,
}
fn prod_await_call_or_ident<'a>(mut parse_state: &ParseState) -> Option<ProdAwaitCallOrIdent<'a>> {
    let identifier = match parse_state.peekable.peek() {
        Some(Token::Identifier(s)) => {parse_state.peekable.next(); s},
        None => panic!("Invalid program."),
        tok => {error_token(parse_state); return None;},
    };
    match parse_state.peekable.peek() {
        Some(Token::RoundOpen) => _ = parse_state.peekable.next(),
        None => error_eof(parse_state),
        tok => return Some(ProdAwaitCallOrIdent {
            identifier: identifier,
            value: None,
            is_func: false,
        }),
    };
    let mut is_hit = false;
    let value: Option<ProdValue> = match parse_state.peekable.peek() {
        Some(Token::RoundClose) => { is_hit = true; None },
        Some(Token::CurlyOpen) => prod_value(parse_state),
        Some(Token::SquareOpen) => prod_value(parse_state),
        Some(Token::Number(d)) => prod_value(parse_state),
        Some(Token::Null) => prod_value(parse_state),
        Some(Token::String(s)) => prod_value(parse_state),
        Some(Token::True) => prod_value(parse_state),
        Some(Token::False) => prod_value(parse_state),
        None => {error_eof(parse_state); return None},
        tok => {error_token(parse_state); return None},
    };
    match parse_state.peekable.peek() {
        Some(Token::RoundClose) => {},
        None => {error_eof(parse_state); return None},
        tok => {error_token(parse_state); return None},
    };
    return Some(ProdAwaitCallOrIdent {
        identifier: identifier,
        value: value,
        is_func: true,
    });
}

pub struct ProdAbort<'a> {
    identifier: &'a String,
}
fn prod_abort<'a>(mut parse_state: &ParseState) -> Option<ProdAbort<'a>> {
    let identifier = match parse_state.peekable.peek() {
        Some(Token::Identifier(s)) => {parse_state.peekable.next(); s},
        None => panic!("Invalid program."),
        tok => {error_token(parse_state); return None;},
    };
    return Some(ProdAbort {
        identifier: identifier,
    });
}

pub struct ProdFor<'a> {
    identifier: &'a String,
    variant: ProdForVariant<'a>,
    code: ProdCode<'a>,
}
fn prod_for<'a>(mut parse_state: &ParseState) -> Option<ProdFor<'a>> {
    match parse_state.peekable.peek() {
        Some(Token::For) => parse_state.peekable.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    let ident = match parse_state.peekable.peek() {
        Some(Token::Identifier(s)) => {parse_state.peekable.next(); s},
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    match parse_state.peekable.peek() {
        Some(Token::In) => parse_state.peekable.next(),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    let prodForVariant = prod_for_variant(parse_state);
    if prodForVariant.is_none() {
        return None;
    }
    let prodCode = prod_code(parse_state);
    if prodCode.is_none() {
        return None;
    }
    return Some(ProdFor {
        identifier: ident,
        variant: prodForVariant.unwrap(),
        code: prodCode.unwrap(),
    });
}

pub enum ProdForVariant<'a> {
    ProdAwait(ProdAwaitCallOrIdent<'a>),
    String(&'a String),
    ProdArray(ProdArray<'a>),
}
fn prod_for_variant<'a>(mut parse_state: &ParseState) -> Option<ProdForVariant<'a>> {
    enum Tmp<'a> {
        ProdAwaitCallOrIdent(Option<ProdAwaitCallOrIdent<'a>>),
        String(&'a String),
        ProdArray(Option<ProdArray<'a>>),
    }
    let result = match parse_state.peekable.peek() {
        Some(Token::Await) => Tmp::ProdAwaitCallOrIdent(prod_for_variant_await(parse_state)),
        Some(Token::SquareOpen) => Tmp::ProdArray(prod_array(parse_state)),
        Some(Token::Identifier(s)) => Tmp::String(s),
        None => panic!("statement reached none, indicating invalid program"),
        _ => {error_token(parse_state); return None;},
    };
    return match result {
        Tmp::ProdAwaitCallOrIdent(optProdAwait) => match optProdAwait {
            Some(prodAwait) => return Some(ProdForVariant::ProdAwait(prodAwait)),
            _ => return None,
        },
        Tmp::String(s) => return Some(ProdForVariant::String(s)),
        Tmp::ProdArray(optProdArray) => match optProdArray {
            Some(prodArray) => return Some(ProdForVariant::ProdArray(prodArray)),
            _ => return None,
        },
        _ => panic!("Invalid program")
    };
}

fn prod_for_variant_await<'a>(mut parse_state: &ParseState) -> Option<ProdAwaitCallOrIdent<'a>> {
    match parse_state.peekable.peek() {
        Some(Token::Await) => parse_state.peekable.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    return prod_await_call_or_ident(parse_state);
}

pub struct ProdIfElse<'a> {
    prodIf: ProdIf<'a>,
    prodElse: Option<ProdCode<'a>>,
}
fn prod_if_else<'a>(mut parse_state: &ParseState) -> Option<ProdIfElse<'a>> {
    let prodIf = match parse_state.peekable.peek() {
        Some(Token::If) => prod_if(parse_state),
        None => panic!("Invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    let prodCode = match parse_state.peekable.peek() {
        Some(Token::Else) => prod_else(parse_state),
        None => {error_eof(parse_state); return None;},
        tok => None,
    }; 
    return Some(ProdIfElse {
        prodIf: prodIf.unwrap(),
        prodElse: prodCode,
    });
}

fn prod_if<'a>(mut parse_state: &ParseState) -> Option<ProdIf<'a>> {
    match parse_state.peekable.peek() {
        Some(Token::If) => parse_state.peekable.next(),
        None => panic!("Invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    return prod_if_part(parse_state);
}

pub struct ProdIf<'a> {
    condition: ProdIfPartCondition<'a>,
    code: ProdCode<'a>,
}
pub enum ProdIfPartCondition<'a> {
    Identifier(&'a String),
    ProdAwaitCallOrIdent(ProdAwaitCallOrIdent<'a>),
}
fn prod_if_part<'a>(mut parse_state: &ParseState) -> Option<ProdIf<'a>> {
    enum Tmp<'a> {
        Identifer(&'a String),
        ProdAwaitCallOrIdent(Option<ProdAwaitCallOrIdent<'a>>),
    }
    let tmp = match parse_state.peekable.peek() {
        Some(Token::Await) => Tmp::ProdAwaitCallOrIdent(prod_if_part_await(parse_state)),
        Some(Token::Identifier(s)) => {parse_state.peekable.next(); Tmp::Identifer(s)},
        None => panic!("Invalid program"),
        _ => {error_token(parse_state); return None;},
    };
    let code = prod_code(parse_state);
    return Some(ProdIf {
        condition: match tmp {
            Tmp::Identifer(s) => ProdIfPartCondition::Identifier(s),
            Tmp::ProdAwaitCallOrIdent(optProdIfPartAwait) => match optProdIfPartAwait {
                Some(prodIfPartAwait) => ProdIfPartCondition::ProdAwaitCallOrIdent(prodIfPartAwait),
                None => return None,
            },
        },
        code: match code {
            Some(c) => c,
            None => return None,
        },
    });
}

fn prod_if_part_await<'a>(mut parse_state: &ParseState) -> Option<ProdAwaitCallOrIdent<'a>> {
    match parse_state.peekable.peek() {
        Some(Token::Await) => parse_state.peekable.next(),
        _ => {error_token(parse_state); return None;},
    };
    return prod_await_call_or_ident(parse_state);
}

fn prod_else<'a>(mut parse_state: &ParseState) -> Option<ProdCode<'a>> {
    todo!()
}

struct ProdCode<'a> {
    toRemove: &'a String,
}
fn prod_code<'a>(mut parse_state: &ParseState) -> Option<ProdCode<'a>> {
    todo!()
}

fn error_token(mut parse_state: &ParseState) {
    parse_state.is_good = false;
    parse_state.errors.push(ParseError {
        message: "Unmatched token".to_string().borrow(),
    });
}
fn error_eof(mut parse_state: &ParseState) {
    parse_state.is_good = false;
    parse_state.errors.push(ParseError {
        message: "Unexoected EOF".to_string().borrow(),
    });
}
fn error_parse_number(mut parse_state: &ParseState) {
    parse_state.is_good = false;
    parse_state.errors.push(ParseError {
        message: "Failed parsing number".to_string().borrow(),
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