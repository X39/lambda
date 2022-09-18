use std::borrow::{Borrow, BorrowMut};

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
    errors: Vec<ParseError>,
    peekable: std::iter::Peekable<Lexer<'a, Token>>,
    lexer: Lexer<'a, Token>,
    vm: VmState,
}

pub struct ParseError {
    message: String,
}
pub fn parse(mut lexer: &Lexer<Token>) -> Result<Vec<ProdStatement>, Vec<ParseError>>
{
    let mut vm_state = VmState {
        function_list: Vec::new(),
        instruction_index: 0,
        instructions: Vec::new(),
        value_list: Vec::new(),
    };
    let mut state = ParseState {
        is_good: true,
        errors: Vec::new(),
        peekable: lexer.peekable(),
        lexer: lexer,
        vm: vm_state,
    };
    let result = prod_file(state);
    if state.errors.len() > 0 {
        return Err(state.errors);
    }
    return Ok(result);
}

fn prod_file(mut parse_state: ParseState) -> Vec<ProdStatement> {
    match parse_state.peekable.peek() {
        Some(_) => prod_statements(parse_state),
        None => Vec::new(),
    }
}

fn prod_statements(mut parse_state: ParseState) -> Vec<ProdStatement> {
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
pub enum ProdStatement {
    ProdAwait(ProdAwait),
    ProdAbort(ProdAbort),
    ProdExit(ProdExit),
    ProdStart(ProdStart),
    ProdIf(ProdIf),
    ProdFor(ProdFor),
    ProdAssignment(ProdAssignment),
}
fn prod_statement(mut parse_state: ParseState) -> Option<ProdStatement> {
    enum Tmp {
        ProdAwait(Option<ProdAwait>),
        ProdAbort(Option<ProdAbort>),
        ProdExit(Option<ProdExit>),
        ProdStart(Option<ProdStart>),
        ProdIf(Option<ProdIf>),
        ProdFor(Option<ProdFor>),
        ProdAssignment(Option<ProdAssignment>),
    }
    let result = match parse_state.peekable.peek() {
        Some(Token::Await) => Tmp::ProdAwait(prod_s_await(parse_state)),
        Some(Token::Abort) => Tmp::ProdAbort(prod_s_abort(parse_state)),
        Some(Token::Exit) => Tmp::ProdExit(prod_s_exit(parse_state)),
        Some(Token::Start) => Tmp::ProdStart(prod_s_start(parse_state)),
        Some(Token::If) => Tmp::ProdIf(prod_if(parse_state)),
        Some(Token::For) => Tmp::ProdFor(prod_for(parse_state)),
        Some(Token::Identifier) => Tmp::ProdAssignment(prod_assignment(parse_state)),
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
        Tmp::ProdIf(optProdIf) => match optProdIf {
            Some(prodIf) => return Some(ProdStatement::ProdIf(prodIf)),
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

pub struct ProdAssignment {
    identifier: String,
    value: ProdAssignmentValue,
}
fn prod_assignment(mut parse_state: ParseState) -> Option<ProdAssignment> {
    let ident = match parse_state.peekable.peek() {
        Some(Token::Identifier) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    match parse_state.peekable.peek() {
        Some(Token::Identifier) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    let assignment = match parse_state.peekable.peek() {
        Some(Token::Await) => prod_assignment_value(parse_state),
        Some(Token::CurlyOpen) => prod_assignment_value(parse_state),
        Some(Token::False) => prod_assignment_value(parse_state),
        Some(Token::True) => prod_assignment_value(parse_state),
        Some(Token::Null) => prod_assignment_value(parse_state),
        Some(Token::Number) => prod_assignment_value(parse_state),
        Some(Token::SquareOpen) => prod_assignment_value(parse_state),
        Some(Token::Start) => prod_assignment_value(parse_state),
        Some(Token::String) => prod_assignment_value(parse_state),
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

pub enum ProdAssignmentValue {
    ProdAwait(ProdAwait),
    ProdValue(ProdValue),
    ProdStart(ProdStart),
}
fn prod_assignment_value(mut parse_state: ParseState) -> Option<ProdAssignmentValue> {
    enum Tmp {
        ProdAwait(Option<ProdAwait>),
        ProdValue(Option<ProdValue>),
        ProdStart(Option<ProdStart>),
    }
    let tmp = match parse_state.peekable.peek() {
        Some(Token::Await) => Tmp::ProdAwait(prod_await(parse_state)),
        Some(Token::Start) => Tmp::ProdStart(prod_start(parse_state)),
        Some(Token::CurlyOpen) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::False) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::True) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::Null) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::Number) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::SquareOpen) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::String) => Tmp::ProdValue(prod_value(parse_state)),
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

pub enum ProdStart {
    ProdCall(ProdCall),
}
fn prod_start(mut parse_state: ParseState) -> Option<ProdStart> {
    match parse_state.peekable.peek() {
        Some(Token::Start) => {parse_state.peekable.next();},
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    let call = match parse_state.peekable.peek() {
        Some(Token::Identifier) => prod_call(parse_state),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    if call.is_none() {
        return None;
    }
    return Some(ProdStart::ProdCall(call.unwrap()));
}

pub struct ProdCall {
    ident: String,
    value: ProdValue,
}
fn prod_call(mut parse_state: ParseState) -> Option<ProdCall>
{
    let ident = match parse_state.peekable.peek() {
        Some(Token::Identifier) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
        None => panic!("Invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    match parse_state.peekable.peek() {
        Some(Token::RoundOpen) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    let value = match parse_state.peekable.peek() {
        Some(Token::CurlyOpen) => prod_value(parse_state),
        Some(Token::False) => prod_value(parse_state),
        Some(Token::True) => prod_value(parse_state),
        Some(Token::Null) => prod_value(parse_state),
        Some(Token::Number) => prod_value(parse_state),
        Some(Token::SquareOpen) => prod_value(parse_state),
        Some(Token::String) => prod_value(parse_state),
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    if value.is_none() {
        return None;
    }
    match parse_state.peekable.peek() {
        Some(Token::RoundClose) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    return Some(ProdCall {
        ident: ident,
        value: value.unwrap(),
    });
}

pub enum ProdValue {
    ProdObj(ProdObj),
    ProdArray(ProdArray),
    ProdNumeric(ProdNumeric),
    ProdConstant(ProdConstant),
}
fn prod_value(mut parse_state: ParseState) -> Option<ProdValue>
{
    enum Tmp {
        ProdObj(Option<ProdObj>),
        ProdArray(Option<ProdArray>),
        ProdNumeric(Option<ProdNumeric>),
        ProdConstant(Option<ProdConstant>),
    }
    let value = match parse_state.peekable.peek() {
        Some(Token::CurlyOpen) => Tmp::ProdObj(prod_obj(parse_state)),
        Some(Token::False) => Tmp::ProdConstant(prod_constant(parse_state)),
        Some(Token::True) => Tmp::ProdConstant(prod_constant(parse_state)),
        Some(Token::Null) => Tmp::ProdConstant(prod_constant(parse_state)),
        Some(Token::Number) => Tmp::ProdNumeric(prod_numeric(parse_state)),
        Some(Token::SquareOpen) => Tmp::ProdArray(prod_array(parse_state)),
        Some(Token::String) => Tmp::ProdConstant(prod_constant(parse_state)),
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

pub struct ProdArray {
    values: Vec<ProdArrayDataEnum>,
}
fn prod_array(mut parse_state: ParseState) -> Option<ProdArray> {
    match parse_state.peekable.peek() {
        Some(Token::SquareOpen) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    let arrayData = match parse_state.peekable.peek() {
        Some(Token::SquareClose) => return Some(ProdArray {
            values: Vec::new(),
        }),
        Some(Token::Identifier) => prod_array_data(parse_state),
        Some(Token::Null) => prod_array_data(parse_state),
        Some(Token::False) => prod_array_data(parse_state),
        Some(Token::True) => prod_array_data(parse_state),
        Some(Token::String) => prod_array_data(parse_state),
        Some(Token::Number) => prod_array_data(parse_state),
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

pub struct ProdArrayData {
    values: Vec<ProdArrayDataEnum>,
}
pub enum ProdArrayDataEnum {
    Identifier(String),
    ProdValue(ProdValue),
}
fn prod_array_data(mut parse_state: ParseState) -> Option<ProdArrayData> {
    enum Tmp {
        Identifier(String),
        ProdValue(Option<ProdValue>),
    }
    let tmp = match parse_state.peekable.peek() {
        Some(Token::Identifier) => {parse_state.peekable.next(); Tmp::Identifier(parse_state.lexer.slice().to_string())},
        Some(Token::String) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::Null) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::False) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::True) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::Number) => Tmp::ProdValue(prod_value(parse_state)),
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
        Some(Token::Identifier) => prod_array_data(parse_state),
        Some(Token::String) => prod_array_data(parse_state),
        Some(Token::Null) => prod_array_data(parse_state),
        Some(Token::False) => prod_array_data(parse_state),
        Some(Token::True) => prod_array_data(parse_state),
        Some(Token::Number) => prod_array_data(parse_state),
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
fn prod_numeric(mut parse_state: ParseState) -> Option<ProdNumeric> {
    let start = match parse_state.peekable.peek() {
        Some(Token::Number) => {parse_state.peekable.next(); parse_state.lexer.slice().parse::<f64>()},
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    if start.is_err() {
        error_parse_number(parse_state);
        return None;
    }

    match parse_state.peekable.peek() {
        Some(Token::DotDot) => parse_state.peekable.next(),
        None => {error_eof(parse_state); return None},
        tok => {return Some(ProdNumeric {
             value: start.unwrap(),
             end: None
            })
        },
    };
    
    let end = match parse_state.peekable.peek() {
        Some(Token::Number) => {parse_state.peekable.next(); parse_state.lexer.slice().parse::<f64>()},
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    if end.is_err() {
        error_parse_number(parse_state);
        return None;
    }
    return Some(ProdNumeric {
        value: start.unwrap(),
        end: Some(end.unwrap()),
    });
}

pub enum ProdConstant {
    Null,
    Boolean(bool),
    String(String),
}
fn prod_constant(mut parse_state: ParseState) -> Option<ProdConstant> {
    // null | string | true | false
    let value = match parse_state.peekable.peek() {
        Some(Token::Null) => ProdConstant::Null,
        Some(Token::String) => {parse_state.peekable.next(); ProdConstant::String(parse_state.lexer.slice().to_string())},
        Some(Token::True) => {parse_state.peekable.next(); ProdConstant::Boolean(false)},
        Some(Token::False) => {parse_state.peekable.next(); ProdConstant::Boolean(false)},
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    return Some(value);
}

pub struct ProdObj {
    properties: Vec<ProdObjProp>,
}
fn prod_obj(mut parse_state: ParseState) -> Option<ProdObj> {
    match parse_state.peekable.peek() {
        Some(Token::CurlyOpen) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
        None => panic!("statement reached none, indicating invalid program"),
        _ => {error_token(parse_state); return None;},
    };
    let objData = match parse_state.peekable.peek() {
        Some(Token::CurlyClose) => return Some(ProdObj {
            properties: Vec::new(),
        }),
        Some(Token::String) => prod_obj_data(parse_state),
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

pub struct ProdObjData {
    properties: Vec<ProdObjProp>,
}
fn prod_obj_data(mut parse_state: ParseState) -> Option<ProdObjData> {
    let prodObjProp = match parse_state.peekable.peek() {
        Some(Token::String) => prod_obj_prop(parse_state),
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
        Some(Token::String) => prod_obj_data(parse_state),
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

pub struct ProdObjProp {
    label: String,
    value: ProdObjPropData,
}
pub enum ProdObjPropData {
    String(String),
    ProdValue(ProdValue),
}
fn prod_obj_prop(mut parse_state: ParseState) -> Option<ProdObjProp> {
    enum Tmp {
        String(String),
        ProdValue(Option<ProdValue>),
    }
    let label = match parse_state.peekable.peek() {
        Some(Token::String) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
        None => panic!("Invalid program"),
        tok => {error_token(parse_state); return None;},
    };
    match parse_state.peekable.peek() {
        Some(Token::Colon) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
        None => {error_eof(parse_state); return None;},
        tok => {error_token(parse_state); return None;},
    };
    let value = match parse_state.peekable.peek() {
        Some(Token::Identifier) => {parse_state.peekable.next(); Tmp::String(parse_state.lexer.slice().to_string())},
        Some(Token::False) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::False) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::True) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::Null) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::Number) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::SquareOpen) => Tmp::ProdValue(prod_value(parse_state)),
        Some(Token::String) => Tmp::ProdValue(prod_value(parse_state)),
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

fn prod_s_exit(mut parse_state: ParseState) -> Option<ProdExit> {
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

fn prod_s_start(mut parse_state: ParseState) -> Option<ProdStart> {
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

fn prod_s_await(mut parse_state: ParseState) -> Option<ProdAwait> {
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

fn prod_s_abort(mut parse_state: ParseState) -> Option<ProdAbort> {
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
fn prod_exit(mut parse_state: ParseState) -> Option<ProdExit> {
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

pub enum ProdAwait {
    ProdAwaitAny(ProdAwaitAny),
    ProdAwaitAll(ProdAwaitAll),
    ProdAwaitCallOrIdent(ProdAwaitCallOrIdent),
}
fn prod_await(mut parse_state: ParseState) -> Option<ProdAwait>{
    enum Tmp {
        ProdAwaitAny(Option<ProdAwaitAny>),
        ProdAwaitAll(Option<ProdAwaitAll>),
        ProdAwaitCallOrIdent(Option<ProdAwaitCallOrIdent>),
    }
    match parse_state.peekable.peek() {
        Some(Token::Await) => _ = parse_state.peekable.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(parse_state),
    }
    let tmp = match parse_state.peekable.peek() {
        Some(Token::All) => Tmp::ProdAwaitAll(prod_await_all(parse_state)),
        Some(Token::Any) => Tmp::ProdAwaitAny(prod_await_any(parse_state)),
        Some(Token::Identifier) => Tmp::ProdAwaitCallOrIdent(prod_await_call_or_ident(parse_state)),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(parse_state); return None},
    };
    todo!()
}

pub struct ProdAwaitAll {
    identifier: String,
}
fn prod_await_all(mut parse_state: ParseState) -> Option<ProdAwaitAll> {
    let identifier = match parse_state.peekable.peek() {
        Some(Token::Identifier) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
        None => panic!("Invalid program."),
        tok => {error_token(parse_state); return None;},
    };
    return Some(ProdAwaitAll {
        identifier: identifier,
    });
}

pub struct ProdAwaitAny {
    identifier: String,
}
fn prod_await_any(mut parse_state: ParseState) -> Option<ProdAwaitAny> {
    let identifier = match parse_state.peekable.peek() {
        Some(Token::Identifier) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
        None => panic!("Invalid program."),
        tok => {error_token(parse_state); return None;},
    };
    return Some(ProdAwaitAny {
        identifier: identifier,
    });
}
pub struct ProdAwaitCallOrIdent {
    identifier: String,
    value: Option<ProdValue>,
    is_func: bool,
}
fn prod_await_call_or_ident(mut parse_state: ParseState) -> Option<ProdAwaitCallOrIdent> {
    let identifier = match parse_state.peekable.peek() {
        Some(Token::Identifier) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
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
        Some(Token::Number) => prod_value(parse_state),
        Some(Token::Null) => prod_value(parse_state),
        Some(Token::String) => prod_value(parse_state),
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

pub struct ProdAbort {
    identifier: String,
}
fn prod_abort(mut parse_state: ParseState) -> Option<ProdAbort> {
    let identifier = match parse_state.peekable.peek() {
        Some(Token::Identifier) => {parse_state.peekable.next(); parse_state.lexer.slice().to_string()},
        None => panic!("Invalid program."),
        tok => {error_token(parse_state); return None;},
    };
    return Some(ProdAbort {
        identifier: identifier,
    });
}

pub struct ProdFor {

}
fn prod_for(mut parse_state: ParseState) -> Option<ProdFor> {
    todo!()
}

pub struct ProdIf {

}
fn prod_if(mut parse_state: ParseState) -> Option<ProdIf> {
    todo!()
}

fn error_token(mut parse_state: ParseState) {
    parse_state.is_good = false;
    parse_state.errors.push(ParseError {
        message: "Unmatched token".to_string(),
    });
}
fn error_eof(mut parse_state: ParseState) {
    parse_state.is_good = false;
    parse_state.errors.push(ParseError {
        message: "Unexoected EOF".to_string(),
    });
}
fn error_parse_number(mut parse_state: ParseState) {
    parse_state.is_good = false;
    parse_state.errors.push(ParseError {
        message: "Failed parsing number".to_string(),
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