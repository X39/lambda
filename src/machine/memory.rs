// Copyright x39

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::machine::OpCode::Pop;
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
        };
    }
    pub fn new<A>(opcode: OpCode, arg: A) -> Instruction where A: Into<InstructionArg> {
        return Instruction {
            opcode,
            arg: arg.into(),
        };
    }

    pub fn op_pop() -> Instruction {
        return Instruction {
            opcode: Pop,
            arg: InstructionArg::Empty,
        };
    }

    pub fn op_push_value_u16(value_index: u16) -> Instruction {
        return Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        };
    }

    pub fn op_assign() -> Instruction {
        return Instruction {
            opcode: OpCode::Assign,
            arg: InstructionArg::Empty,
        };
    }

    pub fn op_get_variable_of_type(value_type: VmValueType) -> Instruction {
        return Instruction {
            opcode: OpCode::Assign,
            arg: InstructionArg::Type(value_type),
        };
    }

    pub fn op_append_array_push() -> Instruction {
        return Instruction {
            opcode: OpCode::AppendArrayPush,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_jump_iterate(index: i16) -> Instruction {
        return Instruction {
            opcode: OpCode::JumpIterate,
            arg: InstructionArg::Signed(index),
        };
    }
    pub fn op_jump(index: i16) -> Instruction {
        return Instruction {
            opcode: OpCode::Jump,
            arg: InstructionArg::Signed(index),
        };
    }
    pub fn op_jump_if_false(index: i16) -> Instruction {
        return Instruction {
            opcode: OpCode::JumpIfFalse,
            arg: InstructionArg::Signed(index),
        };
    }
    pub fn op_jump_if_true(index: i16) -> Instruction {
        return Instruction {
            opcode: OpCode::JumpIfFalse,
            arg: InstructionArg::Signed(index),
        };
    }
    pub fn op_abort() -> Instruction {
        return Instruction {
            opcode: OpCode::Abort,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_get_variable() -> Instruction {
        return Instruction {
            opcode: OpCode::GetVariable,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_print_to_console() -> Instruction {
        return Instruction {
            opcode: OpCode::PrintToConsole,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_abort_all() -> Instruction {
        return Instruction {
            opcode: OpCode::AbortAll,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_await() -> Instruction {
        return Instruction {
            opcode: OpCode::Await,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_await_all() -> Instruction {
        return Instruction {
            opcode: OpCode::AwaitAll,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_await_any() -> Instruction {
        return Instruction {
            opcode: OpCode::AwaitAny,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_call() -> Instruction {
        return Instruction {
            opcode: OpCode::Call,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_call_no_arg() -> Instruction {
        return Instruction {
            opcode: OpCode::CallNoArg,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_push_empty_array() -> Instruction {
        return Instruction {
            opcode: OpCode::PushEmptyArray,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_push_empty_object() -> Instruction {
        return Instruction {
            opcode: OpCode::PushEmptyObject,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_push_null() -> Instruction {
        return Instruction {
            opcode: OpCode::PushNull,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_push_boolean(flag: bool) -> Instruction {
        if flag {
            Instruction::op_push_true()
        } else {
            Instruction::op_push_false()
        }
    }
    pub fn op_push_true() -> Instruction {
        return Instruction {
            opcode: OpCode::PushTrue,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_push_false() -> Instruction {
        return Instruction {
            opcode: OpCode::PushFalse,
            arg: InstructionArg::Empty,
        };
    }
    pub fn op_exit() -> Instruction {
        return Instruction {
            opcode: OpCode::Exit,
            arg: InstructionArg::Empty,
        };
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
