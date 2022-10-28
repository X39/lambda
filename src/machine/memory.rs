// Copyright x39

use std::fmt::Pointer;
use tracing::trace;
use uuid::Uuid;
use super::OpCode;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Instruction {
    pub opcode: OpCode,
    pub arg: InstructionArg,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum InstructionArg {
    Empty,
    Unsigned(u16),
    Signed(i16),
    Type(VmValueType),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum VmValueType {
    Null,
    Array,
    ArrayOfJobs,
    Job,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct VmPair {
    key: String,
    value: VmValue,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum VmValue {
    Null,
    String(String),
    Number(f64),
    Array(Vec<VmValue>),
    Boolean(bool),
    Object(Vec<VmPair>),
    Job(Uuid),
}

pub struct VmState {
    pub value_list: Vec<VmValue>,
    pub function_list: Vec<String>,
    pub instructions: Vec<Instruction>,
    pub instruction_index: u32,
}
impl std::fmt::Debug for VmState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Values: {}", self.value_list.len())?;
        for (index, it) in self.value_list.iter().enumerate() {
            write!(f, "    {:04}: ", index)?;
            it.fmt(f)?;
            writeln!(f)?;
        }
        writeln!(f, "Functions: {}", self.function_list.len())?;
        for (index, it) in self.function_list.iter().enumerate() {
            writeln!(f, "    {:04}: {}", index, it)?;
        }
        writeln!(f, "Instructions: {} (at {})", self.value_list.len(), self.instruction_index)?;
        for (index, it) in self.instructions.iter().enumerate() {
            write!(f, "    {:04}: ", index)?;
            it.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}