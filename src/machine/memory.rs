// Copyright x39

use std::fmt::Pointer;
use tracing::trace;
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

#[derive(Serialize, Deserialize)]
pub struct VmState {
    pub value_list: Vec<VmValue>,
    pub function_list: Vec<String>,
    pub instructions: Vec<Instruction>,
    pub instruction_index: usize,
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