// Copyright x39

use uuid::Uuid;
use super::OpCode;

pub struct Instruction {
    opcode: OpCode,
}

// TODO: Don't do this
pub union InstructionArg {
    unsigned: u16,
    signed: i16,
}

pub struct VmPair {
    key: String,
    value: Option<VmValue>,
}

pub struct VmObject(Vec<VmValue>);

pub struct VmArray(Vec<Option<VmValue>>);

pub struct VmNumber(f64);

pub struct VmBoolean(bool);

pub struct VmString(String);

pub enum VmValue {
    String(VmString),
    Number(VmNumber),
    Array(VmArray),
    Boolean(VmBoolean),
    Object(VmObject),
    Job(Uuid),
}

pub struct VmState {
    pub value_list: Vec<Option<VmValue>>,
    pub function_list: Vec<String>,
    pub instructions: Vec<Instruction>,
    pub instruction_index: u32,
}