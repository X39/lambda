// Copyright x39

use uuid::Uuid;
use super::OpCode;

pub struct Instruction {
    pub opcode: OpCode,
    pub arg: InstructionArg,
}

pub union InstructionArg {
    pub unsigned: u16,
    pub signed: i16,
}

pub struct VmPair {
    key: String,
    value: Option<VmValue>,
}

pub struct VmObject(pub Vec<VmValue>);

pub struct VmArray(pub Vec<Option<VmValue>>);

pub struct VmNumber(pub f64);

pub struct VmBoolean(pub bool);

pub struct VmString(pub String);

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