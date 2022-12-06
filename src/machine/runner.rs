use std::borrow::{Borrow, BorrowMut};
use nom::combinator::value;
use crate::machine::{Instruction, InstructionArg, OpCode, VmPair, VmValue, VmValueType};
use crate::machine::VmValueType::Array;
use crate::VmState;

pub struct VmStack {
    pub data: Vec<VmValue>,
    pub variables: Vec<VmPair>,
}

impl VmValue {
    fn is_job(&self) -> bool {
        match self {
            VmValue::Null => false,
            VmValue::String(_) => false,
            VmValue::Number(_) => false,
            VmValue::Array(_) => false,
            VmValue::Boolean(_) => false,
            VmValue::Object(_) => false,
            VmValue::Job(_) => true,
        }
    }
    fn is_array(&self) -> bool {
        match self {
            VmValue::Null => false,
            VmValue::String(_) => false,
            VmValue::Number(_) => false,
            VmValue::Array(_) => true,
            VmValue::Boolean(_) => false,
            VmValue::Object(_) => false,
            VmValue::Job(_) => false,
        }
    }
    fn is_string(&self) -> bool {
        match self {
            VmValue::Null => false,
            VmValue::String(_) => true,
            VmValue::Number(_) => false,
            VmValue::Array(_) => false,
            VmValue::Boolean(_) => false,
            VmValue::Object(_) => false,
            VmValue::Job(_) => false,
        }
    }
    fn is_object(&self) -> bool {
        match self {
            VmValue::Null => false,
            VmValue::String(_) => false,
            VmValue::Number(_) => false,
            VmValue::Array(_) => false,
            VmValue::Boolean(_) => false,
            VmValue::Object(_) => true,
            VmValue::Job(_) => false,
        }
    }
    fn is_null(&self) -> bool {
        match self {
            VmValue::Null => true,
            VmValue::String(_) => false,
            VmValue::Number(_) => false,
            VmValue::Array(_) => false,
            VmValue::Boolean(_) => false,
            VmValue::Object(_) => false,
            VmValue::Job(_) => false,
        }
    }
    fn is_array_of_jobs(&self) -> bool {
        match self {
            VmValue::Null => false,
            VmValue::String(_) => false,
            VmValue::Number(_) => false,
            VmValue::Array(arr) => {
                let mut allJobs = true;
                for arr_value in arr {
                    if !arr_value.is_job()
                    {
                        allJobs = false;
                        break;
                    }
                }
                allJobs
            }
            VmValue::Boolean(_) => false,
            VmValue::Object(_) => false,
            VmValue::Job(_) => false,
        }
    }
}

impl InstructionArg {
    fn get_vm_type(self) -> Result<VmValueType, &'static str> {
        match self {
            InstructionArg::Empty => Err("Instruction arg was expected to be Type arg but arg was Empty"),
            InstructionArg::Unsigned(_) => Err("Instruction arg was expected to be Type arg but arg was Unsigned"),
            InstructionArg::Signed(_) => Err("Instruction arg was expected to be Type arg but arg was Signed"),
            InstructionArg::Type(t) => Ok(t),
        }
    }
    fn get_signed(self) -> Result<i16, &'static str> {
        match self {
            InstructionArg::Empty => Err("Instruction arg was expected to be Signed arg but arg was Empty"),
            InstructionArg::Unsigned(_) => Err("Instruction arg was expected to be Signed arg but arg was Unsigned"),
            InstructionArg::Signed(signed) => Ok(signed),
            InstructionArg::Type(t) => Err("Instruction arg was expected to be Signed arg but arg was Type"),
        }
    }
    fn get_unsigned(self) -> Result<u16, &'static str> {
        match self {
            InstructionArg::Empty => Err("Instruction arg was expected to be Signed arg but arg was Empty"),
            InstructionArg::Unsigned(unsigned) => Ok(unsigned),
            InstructionArg::Signed(_) => Err("Instruction arg was expected to be Signed arg but arg was Signed"),
            InstructionArg::Type(t) => Err("Instruction arg was expected to be Signed arg but arg was Type"),
        }
    }
}

impl VmStack {
    fn pop_object(&mut self) -> Result<Vec<VmPair>, &'static str> {
        let candidate = self.pop_value()?;
        match candidate {
            VmValue::Object(object) => Ok(object),
            _ => Err("Failed to pop OBJECT value from stack"),
        }
    }

    fn pop_array(&mut self) -> Result<Vec<VmValue>, &'static str> {
        let candidate = self.pop_value()?;
        match candidate {
            VmValue::Array(array) => Ok(array),
            _ => Err("Failed to pop ARRAY value from stack"),
        }
    }

    fn pop_string(&mut self) -> Result<String, &'static str> {
        let candidate = self.pop_value()?;
        match candidate {
            VmValue::String(string) => Ok(string),
            _ => Err("Failed to pop STRING value from stack"),
        }
    }

    fn pop_number(&mut self) -> Result<f64, &'static str> {
        let candidate = self.pop_value()?;
        match candidate {
            VmValue::Number(f) => Ok(f),
            _ => Err("Failed to pop NUMBER value from stack"),
        }
    }

    fn pop_bool(&mut self) -> Result<bool, &'static str> {
        let candidate = self.pop_value()?;
        match candidate {
            VmValue::Boolean(flag) => Ok(flag),
            _ => Err("Failed to pop BOOLEAN value from stack"),
        }
    }

    fn pop_value(&mut self) -> Result<VmValue, &'static str> {
        let value_opt = self.data.pop();
        match value_opt {
            None => Err("Failed to pop value from an empty stack"),
            Some(value) => Ok(value),
        }
    }
}

impl VmState {
    pub fn is_done(&self) -> bool {
        self.instructions.len() <= self.instruction_index
    }
    fn next_instruction(&mut self) -> Result<Instruction, &'static str> {
        if self.is_done()
        { return Err("End of instructions reached"); }

        let instruction = self.instructions[self.instruction_index].clone();
        self.instruction_index += 1;
        return Ok(instruction);
    }
    pub fn step<'a, 'b>(&'a mut self, stack: &'b mut VmStack) -> Result<(), &'static str> {
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
                            let cloned = data.clone();
                            stack.data.push(cloned);
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
                let key = stack.pop_string()?;
                let mut found = false;
                for vm_pair in stack.variables.iter() {
                    if vm_pair.key == key {
                        stack.data.push(vm_pair.value.clone());
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Err("GetVariable found no variable with the name provided");
                }
            }
            OpCode::GetVariableOfType => {
                let key = stack.pop_string()?;
                let mut found = false;
                let expected_type = instruction.arg.get_vm_type()?;
                for vm_pair in stack.variables.iter() {
                    if vm_pair.key == key {
                        if !match expected_type {
                            VmValueType::Null => vm_pair.value.is_null(),
                            VmValueType::Array => vm_pair.value.is_array(),
                            VmValueType::ArrayOfJobs => vm_pair.value.is_array_of_jobs(),
                            VmValueType::Job => vm_pair.value.is_job(),
                        } {
                            return Err("GetVariableOfType found variable but the type was not matching the expected");
                        }
                        stack.data.push(vm_pair.value.clone());
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Err("GetVariableOfType found no variable with the name provided");
                }
            }
            OpCode::AppendArrayPush => {
                let value = stack.pop_value()?;
                let mut array = stack.pop_array()?;
                array.push(value);
                stack.data.push(VmValue::Array(array));
            }
            OpCode::AppendPropertyPush => {
                let value = stack.pop_value()?;
                let key = stack.pop_string()?;
                let mut object = stack.pop_object()?;
                object.push(VmPair {
                    key,
                    value,
                });
                stack.data.push(VmValue::Object(object));
            }
            OpCode::Assign => {
                let key = stack.pop_string()?;
                let value = stack.pop_value()?;
                for mut vm_pair in stack.variables.iter_mut() {
                    if vm_pair.key == key {
                        vm_pair.value = value;
                        return Ok(());
                    }
                }
                stack.variables.push(VmPair {
                    key,
                    value,
                });
            }
            OpCode::Pop => { stack.pop_value()?; }
            OpCode::Jump => {
                let i = instruction.arg.get_signed()?;
                self.jump_instruction_index(i)?;
            }
            OpCode::JumpIfFalse => {
                let flag = stack.pop_bool()?;
                if !flag {
                    let i = instruction.arg.get_signed()?;
                    self.jump_instruction_index(i)?;
                }
            }
            OpCode::JumpIfTrue => {
                let flag = stack.pop_bool()?;
                if flag {
                    let i = instruction.arg.get_signed()?;
                    self.jump_instruction_index(i)?;
                }
            }
            OpCode::JumpIterate => {
                let index = stack.pop_number()?;
                let array_or_object = stack.pop_value()?;
                match array_or_object {
                    VmValue::Array(array) => {
                        if array.len() > (index + 1.0) as usize {
                            let el_opt = array.get(index as usize);
                            match el_opt {
                                None => {}
                                Some(el) => {
                                    let clone = el.clone();
                                    stack.data.push(VmValue::Array(array));
                                    stack.data.push(VmValue::Number(index + 1.0));
                                    stack.data.push(clone);
                                }
                            }
                        }
                    }
                    VmValue::Object(object) => {
                        if object.len() > (index + 1.0) as usize {
                            let el_opt = object.get(index as usize);
                            match el_opt {
                                None => {}
                                Some(el) => {
                                    let clone = el.clone();
                                    stack.data.push(VmValue::Object(object));
                                    stack.data.push(VmValue::Number(index + 1.0));
                                    stack.data.push(clone.value);
                                }
                            }
                        }
                    }
                    _ => return Err("JumpIterate failed to pop either array or object from stack"),
                }
            }
            OpCode::Swap2 => {
                let value1 = stack.pop_value()?;
                let value2 = stack.pop_value()?;
                stack.data.push(value1);
                stack.data.push(value2);
            }
            OpCode::PrintToConsole => {
                let value = stack.pop_value()?;
                println!("{:?}", value);
            }

            OpCode::Await => { return Err("Async opcodes are not implemented (yet)"); }
            OpCode::Abort => { return Err("Async opcodes are not implemented (yet)"); }
            OpCode::AbortAll => { return Err("Async opcodes are not implemented (yet)"); }
            OpCode::AwaitAny => { return Err("Async opcodes are not implemented (yet)"); }
            OpCode::AwaitAll => { return Err("Async opcodes are not implemented (yet)"); }
            OpCode::Call => { return Err("Async opcodes are not implemented (yet)"); }
            OpCode::CallNoArg => { return Err("Async opcodes are not implemented (yet)"); }
        };
        Ok(())
    }

    fn jump_instruction_index(&mut self, i: i16) -> Result<(), &'static str> {
        if i.is_negative() {
            let new_index_opt = self.instruction_index.checked_sub((-i.abs()) as usize);
            match new_index_opt {
                Some(new_index) => { self.instruction_index = new_index; }
                None => { return Err("Jump failed because the resulting index would be out of range"); }
            }
        } else {
            let new_index_opt = self.instruction_index.checked_add(i as usize);
            match new_index_opt {
                Some(new_index) => { self.instruction_index = new_index; }
                None => { return Err("Jump failed because the resulting index would be out of range"); }
            }
        }
        Ok(())
    }
}