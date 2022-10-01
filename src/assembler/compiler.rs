mod compiler {
    use std::borrow::{Borrow, BorrowMut};
    use crate::assembler::parser::parser::{AssignmentStatement, AwaitCallOrIdentProduction, AwaitStatement, Call, CallValue, ForLoopStatement, IfElseStatement, NumericRange, Property, Statement, Value, X39File};
    use crate::machine::{Instruction, InstructionArg, OpCode, VirtualMachine, VmState, VmValue, VmValueType};


    pub fn compile(file: X39File) -> VmState {
        let mut vm = VmState {
            value_list: vec![],
            function_list: vec![],
            instructions: vec![],
            instruction_index: 0,
        };
        compile_statements(file.statements.borrow(), vm.borrow_mut());
        vm
    }

    fn compile_statements(statements: &[Statement], vm: &mut VmState) {
        for statement in statements {
            match statement {
                Statement::Await(awaitStatement) => compile_await(awaitStatement, vm.borrow_mut()),
                Statement::Abort(abortIdent) => compile_abort(abortIdent, vm.borrow_mut()),
                Statement::Exit => compile_exit(vm.borrow_mut()),
                Statement::Comment => {}
                Statement::Start(call) => compile_start(call, vm.borrow_mut()),
                Statement::IfElse(ifElseStatement) => compile_if_else(ifElseStatement, vm.borrow_mut()),
                Statement::ForLoop(forLoopStatement) => compile_for_loop(forLoopStatement, vm.borrow_mut()),
                Statement::Assignment(assignmentStatement) => compile_assignment(assignmentStatement, vm.borrow_mut()),
            }
        }
    }

    fn util_get_value_index(value: VmValue, vm: &mut VmState) -> u16 {
        let mut ret: Option<usize> = None;
        for (index, val) in vm.value_list.iter().enumerate() {
            if value.eq(val) {
                continue;
            }
            ret = Some(index);
            break;
        }
        if ret.is_none() {
            ret = Some(vm.value_list.len());
            vm.value_list.push(value);
        }
        ret.unwrap() as u16
    }

    fn compile_assignment(assignment_statement: &AssignmentStatement, vm: &mut VmState) {
        todo!()
    }

    fn compile_for_loop(for_loop_statement: &ForLoopStatement, vm: &mut VmState) {
        todo!()
    }

    fn compile_if_else(if_else_statement: &IfElseStatement, vm: &mut VmState) {
        todo!()
    }

    fn compile_start(call: &Call, vm: &mut VmState) {
        todo!()
    }

    fn compile_abort(abort_ident: &&str, vm: &mut VmState) {
        todo!()
    }

    fn compile_await(await_statement: &AwaitStatement, vm: &mut VmState) {
        match await_statement {
            AwaitStatement::AwaitAny(awaitAny) => compile_await_any(awaitAny, vm),
            AwaitStatement::AwaitAll(awaitAll) => compile_await_all(awaitAll, vm),
            AwaitStatement::AwaitCallOrIdent(awaitCallOrIdent) => compile_await_call_or_ident(awaitCallOrIdent, vm),
        }
    }

    fn compile_await_call_or_ident(await_call_or_ident: &AwaitCallOrIdentProduction, vm: &mut VmState) {
        match await_call_or_ident {
            AwaitCallOrIdentProduction::Call(call) => compile_call(call, vm),
            AwaitCallOrIdentProduction::Ident(ident) => compile_ident_job(ident, vm),
        }
        vm.instructions.push(Instruction {
            opcode: OpCode::Await,
            arg: InstructionArg::Empty,
        })
    }

    fn compile_call(call: &Call, vm: &mut VmState) {
        let value_index = util_get_value_index(VmValue::String(call.ident.to_string()), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        if let Some(value) = call.value.borrow() {
            compile_call_value(value, vm);
            vm.instructions.push(Instruction {
                opcode: OpCode::Call,
                arg: InstructionArg::Empty,
            })
        } else {
            vm.instructions.push(Instruction {
                opcode: OpCode::CallVoid,
                arg: InstructionArg::Empty,
            })
        }
    }

    fn compile_call_value(call_value: &CallValue, vm: &mut VmState) {
        match call_value {
            CallValue::Ident(ident) => compile_ident(ident, vm),
            CallValue::Value(value) => compile_value(value, vm),
        }
    }

    fn compile_value(value: &Value, vm: &mut VmState) {
        match value {
            Value::NumericRange(numeric_range) => compile_numeric_range(numeric_range, vm),
            Value::Number(number) => compile_number(*number, vm),
            Value::Null => compile_null(vm),
            Value::String(string) => compile_string(string.to_string(), vm),
            Value::Boolean(boolean) => compile_boolean(*boolean, vm),
            Value::Object(object) => compile_object(object, vm),
            Value::Array(array) => compile_array(array, vm),
        }
        todo!()
    }

    fn compile_array(array: &Vec<Value>, vm: &mut VmState) {
        todo!()
    }

    fn compile_object(object: &Vec<Property>, vm: &mut VmState) {
        todo!()
    }

    fn compile_numeric_range(numeric_range: &NumericRange, vm: &mut VmState) {
        todo!()
    }

    fn compile_number(number: f64, vm: &mut VmState) {
        let value_index = util_get_value_index(VmValue::Number(number), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
    }

    fn compile_string(string: String, vm: &mut VmState) {
        let value_index = util_get_value_index(VmValue::String(string), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
    }

    fn compile_null(vm: &mut VmState) {
        vm.instructions.push(Instruction {
            opcode: OpCode::PushNull,
            arg: InstructionArg::Empty,
        });
    }

    fn compile_boolean(boolean: bool, vm: &mut VmState) {
        vm.instructions.push(Instruction {
            opcode: if boolean { OpCode::PushTrue } else { OpCode::PushFalse },
            arg: InstructionArg::Empty,
        });
    }

    fn compile_ident(ident: &str, vm: &mut VmState) {
        let value_index = util_get_value_index(VmValue::String(ident.to_string()), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::GetVariable,
            arg: InstructionArg::Empty,
        });
    }

    fn compile_ident_job(ident: &str, vm: &mut VmState) {
        let value_index = util_get_value_index(VmValue::String(ident.to_string()), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::GetVariableOfType,
            arg: InstructionArg::Type(VmValueType::Job),
        });
    }

    fn compile_await_all(await_all: &str, vm: &mut VmState) {
        let value_index = util_get_value_index(VmValue::String(await_all.to_string()), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::GetVariableOfType,
            arg: InstructionArg::Type(VmValueType::ArrayOfJobs),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::AwaitAll,
            arg: InstructionArg::Empty,
        })
    }

    fn compile_await_any(await_any: &str, vm: &mut VmState) {
        let value_index = util_get_value_index(VmValue::String(await_any.to_string()), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::GetVariableOfType,
            arg: InstructionArg::Type(VmValueType::ArrayOfJobs),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::AwaitAny,
            arg: InstructionArg::Empty,
        })
    }

    fn compile_exit(vm: &mut VmState) {
        let value_index = util_get_value_index(VmValue::Number(0 as f64), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::Exit,
            arg: InstructionArg::Empty,
        })
    }
}