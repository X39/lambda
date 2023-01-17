use std::borrow::{Borrow};
use crate::machine::{Instruction, InstructionArg, OpCode, VmPair, VmStack, VmValue};
use serde::{Serialize, Deserialize};
use uuid::{Uuid};
use crate::controllers::VmController;

#[derive(Serialize, Deserialize)]
pub struct VmState {
    id: Uuid,
    value_list: Vec<VmValue>,
    function_list: Vec<String>,
    instructions: Vec<Instruction>,
    instruction_index: usize,
}

pub enum VmExecResult {
    Empty,
    Suspended,
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

impl VmState
{
    pub fn new() -> VmState {
        return VmState {
            id: Uuid::new_v4(),
            instructions: vec!(),
            function_list: vec!(),
            value_list: vec!(),
            instruction_index: 0,
        };
    }

    pub fn value_index(&mut self, value: VmValue) -> u16 {
        let mut ret: Option<usize> = None;
        for (index, val) in self.value_list.iter().enumerate() {
            if !value.eq(val) {
                continue;
            }
            ret = Some(index);
            break;
        }
        if ret.is_none() {
            ret = Some(self.value_list.len());
            self.value_list.push(value);
        }
        ret.unwrap() as u16
    }
    pub fn push_instruction(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }
    pub fn instructions(&self) -> &[Instruction] {
        return self.instructions.borrow();
    }
    pub fn get_instruction(&mut self, index: usize) -> Option<&mut Instruction> {
        match self.instructions.get_mut(index) {
            None => None,
            Some(v) => Some(v)
        }
    }
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
    pub fn step<'a, 'b>(
        &'a mut self,
        stack: &'b mut VmStack,
        controller: &dyn VmController)
        -> Result<VmExecResult, Box<dyn std::error::Error>>
    {
        let instruction = self.next_instruction()?;
        match instruction.opcode {
            OpCode::NoOp => { /* empty */ }
            OpCode::Exit => {
                self.instruction_index = self.instructions.len();
            }
            OpCode::PushValueU16 => {
                match instruction.arg
                {
                    InstructionArg::Empty => { return Err("PushValueU16 argument is empty.".into()); }
                    InstructionArg::Unsigned(index) => {
                        let data_opt = self.value_list.get(index as usize);
                        if let Some(data) = data_opt {
                            let cloned = data.clone();
                            stack.push_value(cloned);
                        }
                    }
                    InstructionArg::Signed(_) => { return Err("PushValueU16 argument is signed.".into()); }
                    InstructionArg::Type(_) => { return Err("PushValueU16 argument is type.".into()); }
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
                let variable = match stack.get_variable(key) {
                    Some(v) => v,
                    None => return Err("GetVariable found no variable with the name provided.".into()),
                };
                stack.push_value(variable);
            }
            OpCode::GetVariableOfType => {
                let expected_type = instruction.arg.get_vm_type()?;
                let key = stack.pop_string()?;
                let variable = match stack.get_variable(key) {
                    Some(v) => v,
                    None => return Err("GetVariableOfType found no variable with the name provided.".into()),
                };
                if !variable.is_type(expected_type) {
                    return Err("GetVariableOfType found variable but the type was not matching the expected.".into());
                }
                stack.push_value(variable);
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
                stack.set_variable(key, value);
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
                    _ => return Err("JumpIterate failed to pop either array or object from stack.".into()),
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

            OpCode::Await => {
                let job_uuid = stack.pop_job()?;

                let optional_value = controller.get_and_remove_result_of(job_uuid)?;
                if let Some(value) = optional_value {
                    stack.push_value(value);
                } else {
                    controller.suspend_until_any(self, vec!(job_uuid))?;
                    return Ok(VmExecResult::Suspended);
                }
            }
            OpCode::Abort => {
                let job_uuid = stack.pop_job()?;
                controller.abort(vec!(job_uuid))?;
            }
            OpCode::AbortAll => {
                let jobs = stack.pop_array_of_jobs()?;
                controller.abort(jobs)?;
            }
            OpCode::AwaitAny => {
                let jobs = stack.pop_array_of_jobs()?;
                controller.suspend_until_any(self, jobs)?;
            }
            OpCode::AwaitAll => {
                let jobs = stack.pop_array_of_jobs()?;
                controller.suspend_until_all(self, jobs)?;
            }
            OpCode::Call => {
                let function_name = stack.pop_string()?;
                let value = stack.pop_value()?;
                let job = controller.call(function_name, Some(value))?;
                stack.push_value(VmValue::Job(job));
            }
            OpCode::CallNoArg => {
                let function_name = stack.pop_string()?;
                let job = controller.call(function_name, None)?;
                stack.push_value(VmValue::Job(job));
            }
        };
        Ok(VmExecResult::Empty)
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