use std::borrow::{Borrow, BorrowMut};
use crate::machine::{Instruction, InstructionArg, OpCode, VmPair, VmValue};
use crate::VmState;

pub struct VmStack {
    data: Vec<VmValue>,
    variables: Vec<VmPair>,
}

impl VmState {
    fn next_instruction(&mut self) -> Result<Instruction, &'static str> {
        if self.instructions.len() >= self.instruction_index
        { return Err("End of instructions reached"); }

        let instruction = self.instructions[self.instruction_index].clone();
        self.instruction_index += 1;
        return Ok(instruction);
    }
    fn step(&mut self, mut stack: VmStack) -> Result<(), &'static str> {
        let instruction = self.next_instruction()?;
        match instruction.opcode {
            OpCode::NoOp => { /* empty */ }
            OpCode::Exit => {
                self.instruction_index = self.instructions.len();
            }
            OpCode::PushValueU16 => {
                match instruction.arg
                {
                    InstructionArg::Empty => { return Err("PushValueU16 argument is empty."); }
                    InstructionArg::Unsigned(index) => {
                        let data_opt = self.value_list.get(index as usize);
                        if let Some(data) = data_opt {
                            stack.data.push(data.clone());
                        }
                    }
                    InstructionArg::Signed(_) => { return Err("PushValueU16 argument is signed."); }
                    InstructionArg::Type(_) => { return Err("PushValueU16 argument is type."); }
                }
            }
            OpCode::PushTrue => {
                stack.data.push(VmValue::Boolean(true));
            }
            OpCode::PushFalse => {
                stack.data.push(VmValue::Boolean(false));
            }
            OpCode::PushNull => {
                stack.data.push(VmValue::Null);
            }
            OpCode::PushEmptyArray => {
                stack.data.push(VmValue::Array(vec![]));
            }
            OpCode::PushEmptyObject => {
                stack.data.push(VmValue::Object(vec![]));
            }
            OpCode::GetVariable => {
                let var_name_opt = stack.data.pop();
                if var_name_opt.is_none() {
                    return Err("GetVariable could not pop a STRING value from an empty stack");
                }
                match var_name_opt.unwrap() {
                    VmValue::String(s) => {
                        // ToDo
                    },
                    _ => return Err("GetVariable expected STRING value on stack"),
                }
            }
            OpCode::GetVariableOfType => {}
            OpCode::AppendArrayPush => {}
            OpCode::AppendPropertyPush => {}
            OpCode::Assign => {}
            OpCode::Pop => {}
            OpCode::Jump => {}
            OpCode::JumpIfFalse => {}
            OpCode::JumpIfTrue => {}
            OpCode::JumpIterate => {}
            OpCode::Swap2 => {}

            OpCode::Await => {}
            OpCode::Abort => {}
            OpCode::AbortAll => {}
            OpCode::AwaitAny => {}
            OpCode::AwaitAll => {}
            OpCode::Call => {}
            OpCode::CallNoArg => {}
        };
        Ok(())
    }
}