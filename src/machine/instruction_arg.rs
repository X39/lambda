use crate::machine::{VmValueType};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
#[derive(PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
pub enum InstructionArg {
    Empty,
    Unsigned(u16),
    Signed(i16),
    Type(VmValueType),
}

impl Into<InstructionArg> for u16 {
    fn into(self) -> InstructionArg {
        InstructionArg::Unsigned(self)
    }
}

impl Into<InstructionArg> for i16 {
    fn into(self) -> InstructionArg {
        InstructionArg::Signed(self)
    }
}

impl Into<InstructionArg> for VmValueType {
    fn into(self) -> InstructionArg {
        InstructionArg::Type(self)
    }
}

impl InstructionArg {
    pub fn get_vm_type(self) -> Result<VmValueType, &'static str> {
        match self {
            InstructionArg::Empty => Err("Instruction arg was expected to be Type arg but arg was Empty"),
            InstructionArg::Unsigned(_) => Err("Instruction arg was expected to be Type arg but arg was Unsigned"),
            InstructionArg::Signed(_) => Err("Instruction arg was expected to be Type arg but arg was Signed"),
            InstructionArg::Type(t) => Ok(t),
        }
    }
    pub fn get_signed(self) -> Result<i16, &'static str> {
        match self {
            InstructionArg::Empty => Err("Instruction arg was expected to be Signed arg but arg was Empty"),
            InstructionArg::Unsigned(_) => Err("Instruction arg was expected to be Signed arg but arg was Unsigned"),
            InstructionArg::Signed(signed) => Ok(signed),
            InstructionArg::Type(_) => Err("Instruction arg was expected to be Signed arg but arg was Type"),
        }
    }
    pub fn get_unsigned(self) -> Result<u16, &'static str> {
        match self {
            InstructionArg::Empty => Err("Instruction arg was expected to be Signed arg but arg was Empty"),
            InstructionArg::Unsigned(unsigned) => Ok(unsigned),
            InstructionArg::Signed(_) => Err("Instruction arg was expected to be Signed arg but arg was Signed"),
            InstructionArg::Type(_) => Err("Instruction arg was expected to be Signed arg but arg was Type"),
        }
    }
}
