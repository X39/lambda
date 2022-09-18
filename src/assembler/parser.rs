use std::{borrow::{Borrow, BorrowMut}, ops::Deref};

use logos::Lexer;
use pest::error;
use crate::{Token, machine::{VmState, VmValue, VmString}};
use crate::machine::{Instruction, InstructionArg, OpCode};

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

    let result = prod_file(peekable.borrow_mut(), errors.borrow_mut());
    if errors.len() > 0 {
        return Err(errors);
    }
    return Ok(result);
}

fn prod_file<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Vec<ProdStatement<'a>> {
    match peekable.peek() {
        Some(_) => prod_statements(peekable, errors),
        None => Vec::new(),
    }
}

fn prod_statements<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Vec<ProdStatement<'a>> {
    let mut vec = Vec::new();
    while let Some(_) = peekable.peek() {
        let result = prod_statement(peekable, errors);
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
fn prod_statement<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdStatement<'a>> {
    enum Tmp<'a> {
        ProdAwait(Option<ProdAwait<'a>>),
        ProdAbort(Option<ProdAbort<'a>>),
        ProdExit(Option<ProdExit>),
        ProdStart(Option<ProdStart<'a>>),
        ProdIfElse(Option<ProdIfElse<'a>>),
        ProdFor(Option<ProdFor<'a>>),
        ProdAssignment(Option<ProdAssignment<'a>>),
    }
    let result = match peekable.peek() {
        Some(Token::Await) => Tmp::ProdAwait(prod_s_await(peekable, errors)),
        Some(Token::Abort) => Tmp::ProdAbort(prod_s_abort(peekable, errors)),
        Some(Token::Exit) => Tmp::ProdExit(prod_s_exit(peekable, errors)),
        Some(Token::Start) => Tmp::ProdStart(prod_s_start(peekable, errors)),
        Some(Token::If) => Tmp::ProdIfElse(prod_if_else(peekable, errors)),
        Some(Token::For) => Tmp::ProdFor(prod_for(peekable, errors)),
        Some(Token::Identifier(s)) => Tmp::ProdAssignment(prod_assignment(peekable, errors)),
        None => panic!("statement reached none, indicating invalid program"),
        _ => {error_token(peekable, errors); return None;},
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
fn prod_assignment<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdAssignment<'a>> {
    let ident = match peekable.peek() {
        Some(Token::Identifier(s)) => {peekable.next(); s},
        None => panic!("statement reached none, indicating invalid program"),
        _ => {error_token(peekable, errors); return None;},
    };
    match peekable.peek() {
        Some(Token::Identifier(s)) => {peekable.next(); s},
        None => {error_eof(peekable, errors); return None;},
        _ => {error_token(peekable, errors); return None;},
    };
    let assignment = match peekable.peek() {
        Some(Token::Await) => prod_assignment_value(peekable, errors),
        Some(Token::CurlyOpen) => prod_assignment_value(peekable, errors),
        Some(Token::False) => prod_assignment_value(peekable, errors),
        Some(Token::True) => prod_assignment_value(peekable, errors),
        Some(Token::Null) => prod_assignment_value(peekable, errors),
        Some(Token::Number(d)) => prod_assignment_value(peekable, errors),
        Some(Token::SquareOpen) => prod_assignment_value(peekable, errors),
        Some(Token::Start) => prod_assignment_value(peekable, errors),
        Some(Token::String(s)) => prod_assignment_value(peekable, errors),
        None => {error_eof(peekable, errors); return None;},
        _ => {error_token(peekable, errors); return None;},
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
fn prod_assignment_value<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdAssignmentValue<'a>> {
    enum Tmp<'a> {
        ProdAwait(Option<ProdAwait<'a>>),
        ProdValue(Option<ProdValue<'a>>),
        ProdStart(Option<ProdStart<'a>>),
    }
    let tmp = match peekable.peek() {
        Some(Token::Await) => Tmp::ProdAwait(prod_await(peekable, errors)),
        Some(Token::Start) => Tmp::ProdStart(prod_start(peekable, errors)),
        Some(Token::CurlyOpen) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::False) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::True) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::Null) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::Number(d)) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::SquareOpen) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::String(s)) => Tmp::ProdValue(prod_value(peekable, errors)),
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
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
fn prod_start<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdStart<'a>> {
    match peekable.peek() {
        Some(Token::Start) => {peekable.next();},
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    let call = match peekable.peek() {
        Some(Token::Identifier(s)) => prod_call(peekable, errors),
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
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
fn prod_call<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdCall<'a>> {
    let ident = match peekable.peek() {
        Some(Token::Identifier(s)) => {peekable.next(); s},
        None => panic!("Invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    match peekable.peek() {
        Some(Token::RoundOpen) => peekable.next(),
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
    };
    let value = match peekable.peek() {
        Some(Token::CurlyOpen) => prod_value(peekable, errors),
        Some(Token::False) => prod_value(peekable, errors),
        Some(Token::True) => prod_value(peekable, errors),
        Some(Token::Null) => prod_value(peekable, errors),
        Some(Token::Number(d)) => prod_value(peekable, errors),
        Some(Token::SquareOpen) => prod_value(peekable, errors),
        Some(Token::String(s)) => prod_value(peekable, errors),
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
    };
    if value.is_none() {
        return None;
    }
    match peekable.peek() {
        Some(Token::RoundClose) => peekable.next(),
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
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
fn prod_value<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdValue<'a>> {
    enum Tmp<'a> {
        ProdObj(Option<ProdObj<'a>>),
        ProdArray(Option<ProdArray<'a>>),
        ProdNumeric(Option<ProdNumeric>),
        ProdConstant(Option<ProdConstant<'a>>),
    }
    let value = match peekable.peek() {
        Some(Token::CurlyOpen) => Tmp::ProdObj(prod_obj(peekable, errors)),
        Some(Token::False) => Tmp::ProdConstant(prod_constant(peekable, errors)),
        Some(Token::True) => Tmp::ProdConstant(prod_constant(peekable, errors)),
        Some(Token::Null) => Tmp::ProdConstant(prod_constant(peekable, errors)),
        Some(Token::Number(d)) => Tmp::ProdNumeric(prod_numeric(peekable, errors)),
        Some(Token::SquareOpen) => Tmp::ProdArray(prod_array(peekable, errors)),
        Some(Token::String(s)) => Tmp::ProdConstant(prod_constant(peekable, errors)),
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
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
fn prod_array<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdArray<'a>> {
    match peekable.peek() {
        Some(Token::SquareOpen) => peekable.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    let arrayData = match peekable.peek() {
        Some(Token::SquareClose) => return Some(ProdArray {
            values: Vec::new(),
        }),
        Some(Token::Identifier(s)) => prod_array_data(peekable, errors),
        Some(Token::Null) => prod_array_data(peekable, errors),
        Some(Token::False) => prod_array_data(peekable, errors),
        Some(Token::True) => prod_array_data(peekable, errors),
        Some(Token::String(s)) => prod_array_data(peekable, errors),
        Some(Token::Number(d)) => prod_array_data(peekable, errors),
        Some(Token::SquareOpen) => prod_array_data(peekable, errors),
        Some(Token::CurlyOpen) => prod_array_data(peekable, errors),
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
    };
    if arrayData.is_none() {
        return None;
    }
    match peekable.peek() {
        Some(Token::SquareClose) => {}
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
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
fn prod_array_data<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdArrayData<'a>> {
    enum Tmp<'a> {
        Identifier(&'a String),
        ProdValue(Option<ProdValue<'a>>),
    }
    let tmp = match peekable.peek() {
        Some(Token::Identifier(s)) => {peekable.next(); Tmp::Identifier(s)},
        Some(Token::String(s)) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::Null) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::False) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::True) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::Number(d)) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::SquareOpen) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::CurlyOpen) => Tmp::ProdValue(prod_value(peekable, errors)),
        None => panic!("Invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    if match tmp {
        Tmp::Identifier(_) => false,
        Tmp::ProdValue(prodValue) => prodValue.is_none(),
        _ => panic!("Invalid program"),
    } {
        return None;
    }
    match peekable.peek() {
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
    let prodArrData = match peekable.peek() {
        Some(Token::Identifier(s)) => prod_array_data(peekable, errors),
        Some(Token::String(s)) => prod_array_data(peekable, errors),
        Some(Token::Null) => prod_array_data(peekable, errors),
        Some(Token::False) => prod_array_data(peekable, errors),
        Some(Token::True) => prod_array_data(peekable, errors),
        Some(Token::Number(d)) => prod_array_data(peekable, errors),
        Some(Token::SquareOpen) => prod_array_data(peekable, errors),
        Some(Token::CurlyOpen) => prod_array_data(peekable, errors),
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
        tok => {error_token(peekable, errors); return None;},
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
fn prod_numeric<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdNumeric> {
    let start = *match peekable.peek() {
        Some(Token::Number(d)) => {peekable.next(); d},
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };

    match peekable.peek() {
        Some(Token::DotDot) => peekable.next(),
        None => {error_eof(peekable, errors); return None},
        tok => {return Some(ProdNumeric {
             value: start,
             end: None
            })
        },
    };
    
    let end = *match peekable.peek() {
        Some(Token::Number(d)) => {peekable.next(); d},
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(peekable, errors); return None;},
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
fn prod_constant<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdConstant<'a>> {
    // null | string | true | false
    let value = match peekable.peek() {
        Some(Token::Null) => ProdConstant::Null,
        Some(Token::String(s)) => {peekable.next(); ProdConstant::String(s)},
        Some(Token::True) => {peekable.next(); ProdConstant::Boolean(false)},
        Some(Token::False) => {peekable.next(); ProdConstant::Boolean(false)},
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
    };
    return Some(value);
}

pub struct ProdObj<'a> {
    properties: Vec<ProdObjProp<'a>>,
}
fn prod_obj<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdObj<'a>> {
    match peekable.peek() {
        Some(Token::CurlyOpen) => peekable.next(),
        None => panic!("statement reached none, indicating invalid program"),
        _ => {error_token(peekable, errors); return None;},
    };
    let objData = match peekable.peek() {
        Some(Token::CurlyClose) => return Some(ProdObj {
            properties: Vec::new(),
        }),
        Some(Token::String(s)) => prod_obj_data(peekable, errors),
        None => {error_eof(peekable, errors); return None;},
        _ => {error_token(peekable, errors); return None;},
    };
    if objData.is_none() {
        return None;
    }
    match peekable.peek() {
        Some(Token::CurlyClose) => {}
        None => {error_eof(peekable, errors); return None;},
        _ => {error_token(peekable, errors); return None;},
    };
    return Some(ProdObj {
         properties: objData.unwrap().properties
    });
}

pub struct ProdObjData<'a> {
    properties: Vec<ProdObjProp<'a>>,
}
fn prod_obj_data<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdObjData<'a>> {
    let prodObjProp = match peekable.peek() {
        Some(Token::String(s)) => prod_obj_prop(peekable, errors),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    if prodObjProp.is_none() {
        return None;
    }
    match peekable.peek() {
        Some(Token::Comma) => {},
        None => {
            let vec: Vec<ProdObjProp> = Vec::new();
            vec.push(prodObjProp.unwrap());
            return Some(ProdObjData {
                properties: vec,
            });
        },
        tok => {error_token(peekable, errors); return None;},
    };
    let prodObjData = match peekable.peek() {
        Some(Token::String(s)) => prod_obj_data(peekable, errors),
        None => {
            let vec: Vec<ProdObjProp> = Vec::new();
            vec.push(prodObjProp.unwrap());
            return Some(ProdObjData {
                properties: vec,
            });
        },
        tok => {error_token(peekable, errors); return None;},
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
fn prod_obj_prop<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdObjProp<'a>> {
    enum Tmp<'a> {
        String(&'a String),
        ProdValue(Option<ProdValue<'a>>),
    }
    let label = match peekable.peek() {
        Some(Token::String(s)) => {peekable.next(); s},
        None => panic!("Invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    match peekable.peek() {
        Some(Token::Colon) => peekable.next(),
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
    };
    let value = match peekable.peek() {
        Some(Token::Identifier(s)) => {peekable.next(); Tmp::String(s)},
        Some(Token::False) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::False) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::True) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::Null) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::Number(d)) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::SquareOpen) => Tmp::ProdValue(prod_value(peekable, errors)),
        Some(Token::String(s)) => Tmp::ProdValue(prod_value(peekable, errors)),
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
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

fn prod_s_exit<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdExit> {
    let val = match peekable.peek() {
        Some(Token::Exit) => prod_exit(peekable, errors),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    if val.is_none(){
        return None;
    }
    match peekable.peek() {
        Some(Token::Semicolon) => {let _ = peekable.next();},
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
    }
    return val;
}

fn prod_s_start<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdStart<'a>> {
    let val = match peekable.peek() {
        Some(Token::Start) => prod_start(peekable, errors),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    if val.is_none(){
        return None;
    }
    match peekable.peek() {
        Some(Token::Semicolon) => {let _ = peekable.next();},
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
    };
    return val;
}

fn prod_s_await<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdAwait<'a>> {
    let val = match peekable.peek() {
        Some(Token::Await) => prod_await(peekable, errors),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    if val.is_none(){
        return None;
    }
    match peekable.peek() {
        Some(Token::Semicolon) => {let _ = peekable.next();},
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
    };
    return val;
}

fn prod_s_abort<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdAbort<'a>> {
    let val = match peekable.peek() {
        Some(Token::Abort) => prod_abort(peekable, errors),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    if val.is_none(){
        return None;
    }
    match peekable.peek() {
        Some(Token::Semicolon) => {let _ = peekable.next();},
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
    }
    return val;
}

pub struct ProdExit {

}
fn prod_exit<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdExit> {
    match peekable.peek() {
        Some(Token::Exit) => prod_abort(peekable, errors),
        None => panic!("Invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    match peekable.peek() {
        Some(Token::Semicolon) => _ = peekable.next(),
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
    };
    return Some(ProdExit {  });
}

pub enum ProdAwait<'a> {
    ProdAwaitAny(ProdAwaitAny<'a>),
    ProdAwaitAll(ProdAwaitAll<'a>),
    ProdAwaitCallOrIdent(ProdAwaitCallOrIdent<'a>),
}
fn prod_await<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdAwait<'a>>{
    enum Tmp<'a> {
        ProdAwaitAny(Option<ProdAwaitAny<'a>>),
        ProdAwaitAll(Option<ProdAwaitAll<'a>>),
        ProdAwaitCallOrIdent(Option<ProdAwaitCallOrIdent<'a>>),
    }
    match peekable.peek() {
        Some(Token::Await) => _ = peekable.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => error_token(peekable, errors),
    }
    let tmp = match peekable.peek() {
        Some(Token::All) => Tmp::ProdAwaitAll(prod_await_all(peekable, errors)),
        Some(Token::Any) => Tmp::ProdAwaitAny(prod_await_any(peekable, errors)),
        Some(Token::Identifier(s)) => Tmp::ProdAwaitCallOrIdent(prod_await_call_or_ident(peekable, errors)),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(peekable, errors); return None},
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
fn prod_await_all<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdAwaitAll<'a>> {
    let identifier = match peekable.peek() {
        Some(Token::Identifier(s)) => {peekable.next(); s},
        None => panic!("Invalid program."),
        tok => {error_token(peekable, errors); return None;},
    };
    return Some(ProdAwaitAll {
        identifier: identifier,
    });
}

pub struct ProdAwaitAny<'a> {
    identifier: &'a String,
}
fn prod_await_any<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdAwaitAny<'a>> {
    let identifier = match peekable.peek() {
        Some(Token::Identifier(s)) => {peekable.next(); s},
        None => panic!("Invalid program."),
        tok => {error_token(peekable, errors); return None;},
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
fn prod_await_call_or_ident<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdAwaitCallOrIdent<'a>> {
    let identifier = match peekable.peek() {
        Some(Token::Identifier(s)) => {peekable.next(); s},
        None => panic!("Invalid program."),
        tok => {error_token(peekable, errors); return None;},
    };
    match peekable.peek() {
        Some(Token::RoundOpen) => _ = peekable.next(),
        None => error_eof(peekable, errors),
        tok => return Some(ProdAwaitCallOrIdent {
            identifier: identifier,
            value: None,
            is_func: false,
        }),
    };
    let mut is_hit = false;
    let value: Option<ProdValue> = match peekable.peek() {
        Some(Token::RoundClose) => { is_hit = true; None },
        Some(Token::CurlyOpen) => prod_value(peekable, errors),
        Some(Token::SquareOpen) => prod_value(peekable, errors),
        Some(Token::Number(d)) => prod_value(peekable, errors),
        Some(Token::Null) => prod_value(peekable, errors),
        Some(Token::String(s)) => prod_value(peekable, errors),
        Some(Token::True) => prod_value(peekable, errors),
        Some(Token::False) => prod_value(peekable, errors),
        None => {error_eof(peekable, errors); return None},
        tok => {error_token(peekable, errors); return None},
    };
    match peekable.peek() {
        Some(Token::RoundClose) => {},
        None => {error_eof(peekable, errors); return None},
        tok => {error_token(peekable, errors); return None},
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
fn prod_abort<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdAbort<'a>> {
    let identifier = match peekable.peek() {
        Some(Token::Identifier(s)) => {peekable.next(); s},
        None => panic!("Invalid program."),
        tok => {error_token(peekable, errors); return None;},
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
fn prod_for<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdFor<'a>> {
    match peekable.peek() {
        Some(Token::For) => peekable.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    let ident = match peekable.peek() {
        Some(Token::Identifier(s)) => {peekable.next(); s},
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
    };
    match peekable.peek() {
        Some(Token::In) => peekable.next(),
        None => {error_eof(peekable, errors); return None;},
        tok => {error_token(peekable, errors); return None;},
    };
    let prodForVariant = prod_for_variant(peekable, errors);
    if prodForVariant.is_none() {
        return None;
    }
    let prodCode = prod_code(peekable, errors);
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
fn prod_for_variant<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdForVariant<'a>> {
    enum Tmp<'a> {
        ProdAwaitCallOrIdent(Option<ProdAwaitCallOrIdent<'a>>),
        String(&'a String),
        ProdArray(Option<ProdArray<'a>>),
    }
    let result = match peekable.peek() {
        Some(Token::Await) => Tmp::ProdAwaitCallOrIdent(prod_for_variant_await(peekable, errors)),
        Some(Token::SquareOpen) => Tmp::ProdArray(prod_array(peekable, errors)),
        Some(Token::Identifier(s)) => Tmp::String(s),
        None => panic!("statement reached none, indicating invalid program"),
        _ => {error_token(peekable, errors); return None;},
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

fn prod_for_variant_await<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdAwaitCallOrIdent<'a>> {
    match peekable.peek() {
        Some(Token::Await) => peekable.next(),
        None => panic!("statement reached none, indicating invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    return prod_await_call_or_ident(peekable, errors);
}

pub struct ProdIfElse<'a> {
    prodIf: ProdIf<'a>,
    prodElse: Option<ProdCode<'a>>,
}
fn prod_if_else<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdIfElse<'a>> {
    let prodIf = match peekable.peek() {
        Some(Token::If) => prod_if(peekable, errors),
        None => panic!("Invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    let prodCode = match peekable.peek() {
        Some(Token::Else) => prod_else(peekable, errors),
        None => {error_eof(peekable, errors); return None;},
        tok => None,
    }; 
    return Some(ProdIfElse {
        prodIf: prodIf.unwrap(),
        prodElse: prodCode,
    });
}

fn prod_if<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdIf<'a>> {
    match peekable.peek() {
        Some(Token::If) => peekable.next(),
        None => panic!("Invalid program"),
        tok => {error_token(peekable, errors); return None;},
    };
    return prod_if_part(peekable, errors);
}

pub struct ProdIf<'a> {
    condition: ProdIfPartCondition<'a>,
    code: ProdCode<'a>,
}
pub enum ProdIfPartCondition<'a> {
    Identifier(&'a String),
    ProdAwaitCallOrIdent(ProdAwaitCallOrIdent<'a>),
}
fn prod_if_part<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdIf<'a>> {
    enum Tmp<'a> {
        Identifer(&'a String),
        ProdAwaitCallOrIdent(Option<ProdAwaitCallOrIdent<'a>>),
    }
    let tmp = match peekable.peek() {
        Some(Token::Await) => Tmp::ProdAwaitCallOrIdent(prod_if_part_await(peekable, errors)),
        Some(Token::Identifier(s)) => {peekable.next(); Tmp::Identifer(s)},
        None => panic!("Invalid program"),
        _ => {error_token(peekable, errors); return None;},
    };
    let code = prod_code(peekable, errors);
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

fn prod_if_part_await<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdAwaitCallOrIdent<'a>> {
    match peekable.peek() {
        Some(Token::Await) => peekable.next(),
        _ => {error_token(peekable, errors); return None;},
    };
    return prod_await_call_or_ident(peekable, errors);
}

fn prod_else<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdCode<'a>> {
    todo!()
}

struct ProdCode<'a> {
    toRemove: &'a String,
}
fn prod_code<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) -> Option<ProdCode<'a>> {
    todo!()
}

fn error_token<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) {
    errors.push(ParseError {
        message: "Unmatched token".to_string().borrow(),
    });
}
fn error_eof<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) {
    errors.push(ParseError {
        message: "Unexoected EOF".to_string().borrow(),
    });
}
fn error_parse_number<'a>(peekable: &mut std::iter::Peekable<Lexer<'a, Token>>, errors: &Vec<ParseError<'a>>) {
    errors.push(ParseError {
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