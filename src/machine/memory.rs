// Copyright x39

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use super::OpCode;

#[derive(Debug)]
#[derive(PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Instruction {
    pub opcode: OpCode,
    pub arg: InstructionArg,
}

impl Instruction {
    pub fn new1(op: OpCode) -> Instruction {
        return Instruction {
            opcode: op,
            arg: InstructionArg::Empty,
        }
    }
    pub fn new(opcode: OpCode, arg: InstructionArg) -> Instruction {
        return Instruction {
            opcode,
            arg,
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
pub enum InstructionArg {
    Empty,
    Unsigned(u16),
    Signed(i16),
    Type(VmValueType),
}

#[derive(Debug)]
#[derive(PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
pub enum VmValueType {
    Null,
    Array,
    ArrayOfJobs,
    Job,
}

#[derive(Debug)]
#[derive(PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct VmPair {
    pub key: String,
    pub value: VmValue,
}

#[derive(Debug)]
#[derive(PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
pub enum VmValue {
    Null,
    String(String),
    Number(f64),
    Array(Vec<VmValue>),
    Boolean(bool),
    Object(Vec<VmPair>),
    Job(Uuid),
}
