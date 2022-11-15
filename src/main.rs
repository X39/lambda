#![allow(dead_code)]

extern crate core;

use std::error::Error;
use crate::machine::VmState;

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

fn vm_state_step(mut state: &VmState) {

}

fn main() {
    let vm_state = create_vm_state("").unwrap();

}
