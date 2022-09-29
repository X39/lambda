// Copyright x39

use uuid::Uuid;
use super::OpCode;

pub struct Instruction {
    pub opcode: OpCode,
    pub arg: InstructionArg,
}

pub enum InstructionArg {
    Empty,
    Unsigned(u16),
    Signed(i16),
}

pub struct VmPair {
    key: String,
    value: VmValue,
}

pub enum VmValue {
    Null,
    String(String),
    Number(f64),
    Array(Vec<VmValue>),
    Boolean(bool),
    Object(Vec<VmPair>),
    Job(Uuid),
}

impl PartialEq<Self> for VmPair {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}
impl PartialEq<Self> for VmValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            VmValue::Null => match other {
                VmValue::Null => true,
                _ => false
            }
            VmValue::String(l) => match other {
                VmValue::String(r) => l == r,
                _ => false
            }
            VmValue::Number(l) => match other {
                VmValue::Number(r) => l == r,
                _ => false
            }
            VmValue::Array(l) => match other {
                VmValue::Array(r) => l == r,
                _ => false
            }
            VmValue::Boolean(l) => match other {
                VmValue::Boolean(r) => l == r,
                _ => false
            }
            VmValue::Object(l) => match other {
                VmValue::Object(r) => l == r,
                _ => false
            }
            VmValue::Job(l) => match other {
                VmValue::Job(r) => l == r,
                _ => false
            }
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

pub struct VmState {
    pub value_list: Vec<VmValue>,
    pub function_list: Vec<String>,
    pub instructions: Vec<Instruction>,
    pub instruction_index: u32,
}