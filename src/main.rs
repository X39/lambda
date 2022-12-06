#![allow(dead_code)]

extern crate core;

use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use crate::machine::{VmPair, VmStack, VmState, VmValue};

mod machine;
mod assembler;

// use crate::assembler::Token;

fn create_vm_state(s: &str) -> Result<VmState, &str> {
    let parse_result = crate::assembler::parser::parser::parse_x39file(s);
    if parse_result.is_err()
    { return Err("Failed to parse input"); }
    let (remainder, cst) = parse_result.unwrap();
    if !remainder.is_empty()
    { return Err("Failed to fully parse input"); }
    let vm_state = crate::assembler::compiler::compiler::compile(cst);
    return Ok(vm_state);
}

fn vm_state_step<'a>(state: &'a mut VmState, stack: &'a mut VmStack) {
    while !state.is_done() {
        match state.step(stack) {
            Ok(_) => {}
            Err(s) => {
                println!("{}", s);
                return;
            },
        }
    }
}

fn main() {
    let mut vm_state = create_vm_state("\
    a = [1,2];\
    print a;\
    a += 3;\
    print a;").unwrap();
    let mut vm_stack = VmStack {
        variables: vec![],
        data: vec![],
    };
    vm_state_step(&mut vm_state, &mut vm_stack);
}
