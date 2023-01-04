use crate::machine::{Instruction, InstructionArg, OpCode, VmPair, VmStack, VmValue, VmValueType};
use serde::{Serialize, Deserialize};

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
                            stack.push_value(cloned);
                        }
                    }
                    InstructionArg::Signed(_) => { return Err("PushValueU16 argument is signed."); }
                    InstructionArg::Type(_) => { return Err("PushValueU16 argument is type."); }
                }
            }
            OpCode::PushTrue => {
                stack.push_value(VmValue::Boolean(true));
            }
            OpCode::PushFalse => {
                stack.push_value(VmValue::Boolean(false));
            }
            OpCode::PushNull => {
                stack.push_value(VmValue::Null);
            }
            OpCode::PushEmptyArray => {
                stack.push_value(VmValue::Array(vec![]));
            }
            OpCode::PushEmptyObject => {
                stack.push_value(VmValue::Object(vec![]));
            }
            OpCode::GetVariable => {
                let key = stack.pop_string()?;
                let mut found = false;
                for vm_pair in stack.variables.iter() {
                    if vm_pair.key == key {
                        stack.push_value(vm_pair.value.clone());
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
                        stack.push_value(vm_pair.value.clone());
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
                stack.push_value(VmValue::Array(array));
            }
            OpCode::AppendPropertyPush => {
                let value = stack.pop_value()?;
                let key = stack.pop_string()?;
                let mut object = stack.pop_object()?;
                object.push(VmPair {
                    key,
                    value,
                });
                stack.push_value(VmValue::Object(object));
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
                                    stack.push_value(VmValue::Array(array));
                                    stack.push_value(VmValue::Number(index + 1.0));
                                    stack.push_value(clone);
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
                                    stack.push_value(VmValue::Object(object));
                                    stack.push_value(VmValue::Number(index + 1.0));
                                    stack.push_value(clone.value);
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
                stack.push_value(value1);
                stack.push_value(value2);
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