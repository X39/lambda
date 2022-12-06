pub mod compiler {
    use std::borrow::{Borrow, BorrowMut};
    use tracing::trace;

    use crate::assembler::parser::parser::{AssignmentStatement, AssignmentType, AssignStatementData, AwaitCallOrIdentProduction, AwaitStatement, Call, CallValue, ElseStatement, ForLoopInstruction, ForLoopStatement, IfElseStatement, IfStatementCondition, NumericRange, Property, Statement, Value, X39File};
    use crate::machine::{Instruction, InstructionArg, OpCode, VmState, VmValue, VmValueType};


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
                Statement::Await(await_statement) => compile_await(await_statement, vm.borrow_mut()),
                Statement::Abort(abort_ident) => compile_abort(abort_ident, vm.borrow_mut()),
                Statement::AbortAll(abort_ident) => compile_abort_all(abort_ident, vm.borrow_mut()),
                Statement::Exit => compile_exit(vm.borrow_mut()),
                Statement::Comment => {}
                Statement::Start(call) => {
                    compile_start(call, vm.borrow_mut());
                    vm.instructions.push(Instruction {
                        opcode: OpCode::Pop,
                        arg: InstructionArg::Empty,
                    });
                }
                Statement::IfElse(if_else_statement) => compile_if_else(if_else_statement, vm.borrow_mut()),
                Statement::ForLoop(for_loop_statement) => compile_for_loop(for_loop_statement, vm.borrow_mut()),
                Statement::Assignment(assignment_statement) => compile_assignment(assignment_statement, vm.borrow_mut()),
                Statement::Print(ident) => compile_print(ident, vm.borrow_mut()),
            }
        }
    }

    fn util_get_value_index(value: VmValue, vm: &mut VmState) -> u16 {
        let mut ret: Option<usize> = None;
        for (index, val) in vm.value_list.iter().enumerate() {
            if !value.eq(val) {
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
        trace!("Entering compile_assignment with {} instructions", vm.instructions.len());
        let key = assignment_statement.ident.to_string();
        match assignment_statement.value.borrow() {
            AssignmentType::Append(append) => compile_assignment_append(append, key, vm),
            AssignmentType::Assign(assign) => compile_assignment_assign(assign, key, vm),
        }
        trace!("Exiting compile_assignment with {} instructions", vm.instructions.len());
    }

    fn compile_assignment_assign(assign: &AssignStatementData, ident: String, vm: &mut VmState) {
        trace!("Entering compile_assignment_assign with {} instructions", vm.instructions.len());
        // Reserve variable name index
        let value_index = util_get_value_index(VmValue::String(ident), vm.borrow_mut());
        // PUSH the value to append on the stack
        match assign {
            AssignStatementData::Value(value) => compile_value(value, vm),
            AssignStatementData::Await(await_call_or_ident) => compile_await_call_or_ident(await_call_or_ident, vm),
            AssignStatementData::Start(start) => compile_start(start, vm),
        }
        // PUSH variable name to stack for assignment in the end
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        // Assign array to variable
        vm.instructions.push(Instruction {
            opcode: OpCode::Assign,
            arg: InstructionArg::Empty,
        });
        trace!("Exiting compile_assignment_assign with {} instructions", vm.instructions.len());
    }

    fn compile_assignment_append(append: &AssignStatementData, ident: String, vm: &mut VmState) {
        trace!("Entering compile_assignment_append with {} instructions", vm.instructions.len());
        // Reserve variable name value index
        let value_index = util_get_value_index(VmValue::String(ident), vm.borrow_mut());
        // PUSH array in variable to stack
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::GetVariableOfType,
            arg: InstructionArg::Type(VmValueType::Array),
        });
        // PUSH the value to append on the stack
        match append {
            AssignStatementData::Value(value) => compile_value(value, vm),
            AssignStatementData::Await(await_call_or_ident) => compile_await_call_or_ident(await_call_or_ident, vm),
            AssignStatementData::Start(start) => compile_start(start, vm),
        }
        // Append the value to the array
        vm.instructions.push(Instruction {
            opcode: OpCode::AppendArrayPush,
            arg: InstructionArg::Empty,
        });
        // PUSH variable name to stack for assignment in the end
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        // Assign array to variable
        vm.instructions.push(Instruction {
            opcode: OpCode::Assign,
            arg: InstructionArg::Empty,
        });
        trace!("Exiting compile_assignment_append with {} instructions", vm.instructions.len());
    }

    fn compile_for_loop(for_loop_statement: &ForLoopStatement, vm: &mut VmState) {
        trace!("Entering compile_for_loop with {} instructions", vm.instructions.len());
        // PUSH value to iterate over
        match for_loop_statement.over.borrow() {
            ForLoopInstruction::Ident(ident) => compile_ident(ident, vm),
            ForLoopInstruction::Await(await_call_or_ident) => compile_await_call_or_ident(await_call_or_ident, vm),
            ForLoopInstruction::Value(value) => compile_value(value, vm),
        }
        // PUSH index
        let value_index = util_get_value_index(VmValue::Number(0.0), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        // Prepare jump instruction
        let jump_offset = vm.instructions.len();
        vm.instructions.push(Instruction {
            opcode: OpCode::JumpIterate,
            arg: InstructionArg::Signed(0),
        });
        // PUSH variable name to stack for assignment in the end
        let ident = for_loop_statement.ident.to_string();
        let value_index = util_get_value_index(VmValue::String(ident), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        // Assign iterated element to variable
        vm.instructions.push(Instruction {
            opcode: OpCode::Assign,
            arg: InstructionArg::Empty,
        });
        // Emit code
        compile_statements(for_loop_statement.code.borrow(), vm);
        // Emit jump back to loop
        let break_jump_offset = vm.instructions.len();
        vm.instructions.push(Instruction {
            opcode: OpCode::Jump,
            arg: InstructionArg::Signed(-((break_jump_offset - jump_offset) as i16)),
        });
        // Update skip
        let next_offset = vm.instructions.len();
        vm.instructions[jump_offset].arg = InstructionArg::Signed((next_offset - jump_offset) as i16);
        trace!("Exiting compile_for_loop with {} instructions", vm.instructions.len());
    }

    fn compile_if_else(if_else_statement: &IfElseStatement, vm: &mut VmState) {
        trace!("Entering compile_if_else with {} instructions", vm.instructions.len());
        // PUSH condition
        compile_if_statement_condition(if_else_statement.if_statement.condition.borrow(), vm);
        // Prepare jump instruction
        let true_offset = vm.instructions.len();
        vm.instructions.push(Instruction {
            opcode: OpCode::JumpIfFalse,
            arg: InstructionArg::Signed(0),
        });
        // Write out if code
        compile_statements(if_else_statement.if_statement.code.borrow(), vm);

        if let Some(else_statement) = if_else_statement.else_statement.borrow() {
            // Prepare else skip-jump
            let skip_offset = vm.instructions.len();
            vm.instructions.push(Instruction {
                opcode: OpCode::Jump,
                arg: InstructionArg::Signed(0),
            });
            // Modify prepared jump instruction to correct offset
            let after_true_code_offset = vm.instructions.len();
            vm.instructions[true_offset].arg = InstructionArg::Signed((after_true_code_offset - true_offset - 1) as i16);
            // Write out else code
            match else_statement {
                ElseStatement::Code(else_code) => compile_statements(else_code, vm),
                ElseStatement::IfElse(if_else) => compile_if_else(if_else, vm),
            }
            // Modify prepared jump instruction to correct offset
            let after_else_code_offset = vm.instructions.len();
            vm.instructions[skip_offset].arg = InstructionArg::Signed((after_else_code_offset - skip_offset - 1) as i16);
        } else {
            // Modify prepared jump instruction to correct offset
            let after_true_code_offset = vm.instructions.len();
            vm.instructions[true_offset].arg = InstructionArg::Signed((after_true_code_offset - true_offset - 1) as i16);
        }
        trace!("Exiting compile_if_else with {} instructions", vm.instructions.len());
    }

    fn compile_if_statement_condition(condition: &IfStatementCondition, vm: &mut VmState) {
        trace!("Entering compile_if_statement_condition with {} instructions", vm.instructions.len());
        match condition {
            IfStatementCondition::Await(await_call_or_ident) => compile_await_call_or_ident(await_call_or_ident, vm),
            IfStatementCondition::Ident(ident) => compile_ident(*ident, vm),
        }
        trace!("Exiting compile_if_statement_condition with {} instructions", vm.instructions.len());
    }

    fn compile_start(call: &Call, vm: &mut VmState) {
        trace!("Entering compile_start with {} instructions", vm.instructions.len());
        compile_call(call, vm);
        trace!("Exiting compile_start with {} instructions", vm.instructions.len());
    }

    fn compile_abort(abort_ident: &&str, vm: &mut VmState) {
        trace!("Entering compile_abort with {} instructions", vm.instructions.len());
        let value_index = util_get_value_index(VmValue::String(abort_ident.to_string()), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::GetVariableOfType,
            arg: InstructionArg::Type(VmValueType::Job),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::Abort,
            arg: InstructionArg::Empty,
        });
        trace!("Exiting compile_abort with {} instructions", vm.instructions.len());
    }

    fn compile_print(ident: &&str, vm: &mut VmState) {
        trace!("Entering compile_print with {} instructions", vm.instructions.len());
        let value_index = util_get_value_index(VmValue::String(ident.to_string()), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::GetVariable,
            arg: InstructionArg::Empty,
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::PrintToConsole,
            arg: InstructionArg::Empty,
        });
        trace!("Exiting compile_print with {} instructions", vm.instructions.len());
    }

    fn compile_abort_all(abort_ident: &&str, vm: &mut VmState) {
        trace!("Entering compile_abort with {} instructions", vm.instructions.len());
        let value_index = util_get_value_index(VmValue::String(abort_ident.to_string()), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::GetVariableOfType,
            arg: InstructionArg::Type(VmValueType::Job),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::AbortAll,
            arg: InstructionArg::Empty,
        });
        trace!("Exiting compile_abort with {} instructions", vm.instructions.len());
    }

    fn compile_await(await_statement: &AwaitStatement, vm: &mut VmState) {
        trace!("Entering compile_await with {} instructions", vm.instructions.len());
        match await_statement {
            AwaitStatement::AwaitAny(await_any) => compile_await_any(await_any, vm),
            AwaitStatement::AwaitAll(await_all) => compile_await_all(await_all, vm),
            AwaitStatement::AwaitCallOrIdent(await_call_or_ident) => compile_await_call_or_ident(await_call_or_ident, vm),
        }
        trace!("Exiting compile_await with {} instructions", vm.instructions.len());
    }

    fn compile_await_call_or_ident(await_call_or_ident: &AwaitCallOrIdentProduction, vm: &mut VmState) {
        trace!("Entering compile_await_call_or_ident with {} instructions", vm.instructions.len());
        match await_call_or_ident {
            AwaitCallOrIdentProduction::Call(call) => compile_call(call, vm),
            AwaitCallOrIdentProduction::Ident(ident) => compile_ident_job(ident, vm),
        }
        vm.instructions.push(Instruction {
            opcode: OpCode::Await,
            arg: InstructionArg::Empty,
        });
        trace!("Exiting compile_await_call_or_ident with {} instructions", vm.instructions.len());
    }

    fn compile_call(call: &Call, vm: &mut VmState) {
        trace!("Entering compile_call with {} instructions", vm.instructions.len());
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
                opcode: OpCode::CallNoArg,
                arg: InstructionArg::Empty,
            })
        }
        trace!("Exiting compile_call with {} instructions", vm.instructions.len());
    }

    fn compile_call_value(call_value: &CallValue, vm: &mut VmState) {
        trace!("Entering compile_call_value with {} instructions", vm.instructions.len());
        match call_value {
            CallValue::Ident(ident) => compile_ident(ident, vm),
            CallValue::Value(value) => compile_value(value, vm),
        }
        trace!("Exiting compile_call_value with {} instructions", vm.instructions.len());
    }

    fn compile_value(value: &Value, vm: &mut VmState) {
        trace!("Entering compile_value with {} instructions", vm.instructions.len());
        match value {
            Value::NumericRange(numeric_range) => compile_numeric_range(numeric_range, vm),
            Value::Number(number) => compile_number(*number, vm),
            Value::Null => compile_null(vm),
            Value::String(string) => compile_string(string.to_string(), vm),
            Value::Boolean(boolean) => compile_boolean(*boolean, vm),
            Value::Object(object) => compile_object(object, vm),
            Value::Array(array) => compile_array(array, vm),
        }
        trace!("Exiting compile_value with {} instructions", vm.instructions.len());
    }

    fn compile_array(array: &Vec<Value>, vm: &mut VmState) {
        trace!("Entering compile_array with {} instructions", vm.instructions.len());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushEmptyArray,
            arg: InstructionArg::Empty,
        });

        for it in array.iter() {
            compile_value(it, vm);
            vm.instructions.push(Instruction {
                opcode: OpCode::AppendArrayPush,
                arg: InstructionArg::Empty,
            });
        }
        trace!("Exiting compile_array with {} instructions", vm.instructions.len());
    }

    fn compile_object(object: &Vec<Property>, vm: &mut VmState) {
        trace!("Entering compile_object with {} instructions", vm.instructions.len());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushEmptyObject,
            arg: InstructionArg::Empty,
        });

        for it in object.iter() {
            // Push Key
            let key = it.key.to_string();
            let value_index = util_get_value_index(VmValue::String(key), vm.borrow_mut());
            vm.instructions.push(Instruction {
                opcode: OpCode::PushValueU16,
                arg: InstructionArg::Unsigned(value_index),
            });

            // Push Value
            let value = it.value.borrow();
            compile_value(value, vm);
            vm.instructions.push(Instruction {
                opcode: OpCode::AppendPropertyPush,
                arg: InstructionArg::Empty,
            });
        }
        trace!("Exiting compile_object with {} instructions", vm.instructions.len());
    }

    fn compile_numeric_range(numeric_range: &NumericRange, vm: &mut VmState) {
        trace!("Entering compile_numeric_range with {} instructions", vm.instructions.len());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushEmptyArray,
            arg: InstructionArg::Empty,
        });

        for i in numeric_range.from as i64..numeric_range.to as i64 {
            let value_index = util_get_value_index(VmValue::Number(i as f64), vm.borrow_mut());
            vm.instructions.push(Instruction {
                opcode: OpCode::PushValueU16,
                arg: InstructionArg::Unsigned(value_index),
            });
            vm.instructions.push(Instruction {
                opcode: OpCode::AppendArrayPush,
                arg: InstructionArg::Empty,
            });
        }
        trace!("Exiting compile_numeric_range with {} instructions", vm.instructions.len());
    }

    fn compile_number(number: f64, vm: &mut VmState) {
        trace!("Entering compile_number with {} instructions", vm.instructions.len());
        let value_index = util_get_value_index(VmValue::Number(number), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        trace!("Exiting compile_number with {} instructions", vm.instructions.len());
    }

    fn compile_string(string: String, vm: &mut VmState) {
        trace!("Entering compile_string with {} instructions", vm.instructions.len());
        let value_index = util_get_value_index(VmValue::String(string), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        trace!("Exiting compile_string with {} instructions", vm.instructions.len());
    }

    fn compile_null(vm: &mut VmState) {
        trace!("Entering compile_null with {} instructions", vm.instructions.len());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushNull,
            arg: InstructionArg::Empty,
        });
        trace!("Exiting compile_null with {} instructions", vm.instructions.len());
    }

    fn compile_boolean(boolean: bool, vm: &mut VmState) {
        trace!("Entering compile_boolean with {} instructions", vm.instructions.len());
        vm.instructions.push(Instruction {
            opcode: if boolean { OpCode::PushTrue } else { OpCode::PushFalse },
            arg: InstructionArg::Empty,
        });
        trace!("Exiting compile_boolean with {} instructions", vm.instructions.len());
    }

    fn compile_ident(ident: &str, vm: &mut VmState) {
        trace!("Entering compile_ident with {} instructions", vm.instructions.len());
        let value_index = util_get_value_index(VmValue::String(ident.to_string()), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::GetVariable,
            arg: InstructionArg::Empty,
        });
        trace!("Exiting compile_ident with {} instructions", vm.instructions.len());
    }

    fn compile_ident_job(ident: &str, vm: &mut VmState) {
        trace!("Entering compile_ident_job with {} instructions", vm.instructions.len());
        let value_index = util_get_value_index(VmValue::String(ident.to_string()), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index),
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::GetVariableOfType,
            arg: InstructionArg::Type(VmValueType::Job),
        });
        trace!("Exiting compile_ident_job with {} instructions", vm.instructions.len());
    }

    fn compile_await_all(await_all: &str, vm: &mut VmState) {
        trace!("Entering compile_await_all with {} instructions", vm.instructions.len());
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
        });
        trace!("Exiting compile_await_all with {} instructions", vm.instructions.len());
    }

    fn compile_await_any(await_any: &str, vm: &mut VmState) {
        trace!("Entering compile_await_any with {} instructions", vm.instructions.len());
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
        });
        trace!("Exiting compile_await_any with {} instructions", vm.instructions.len());
    }

    fn compile_exit(vm: &mut VmState) {
        trace!("Entering compile_exit with {} instructions", vm.instructions.len());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushNull,
            arg: InstructionArg::Empty,
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::Exit,
            arg: InstructionArg::Empty,
        });
        trace!("Exiting compile_exit with {} instructions", vm.instructions.len());
    }
}

#[cfg(test)]
mod tests {
    use std::io::empty;
    use tracing::trace;
    use tracing_test::traced_test;
    use crate::machine::{Instruction, InstructionArg, OpCode};
    use crate::machine::InstructionArg::Empty;

    const TEST_FILE1: &str = r#"
    # comment
    variable = start func({
        "foo": false,
        "bar": "test",
        "foobar": 1.2,
        "other": null,
        "array": [1,2 , 3, 4 ,5,6]
    });
    await variable;
    result = await func2(variable);
    if await conditionFunc(result) {
        exit;
    }
    else if await conditionFunc2(result) {
        exit;
    }
    else {
        # Something ain't working here
        collection = await generateFunc(12);
        list = [];
        for it in collection {
            list += start handleIt(it);
        }
        await all list;
        list = [];
        for it in 0..20 {
            list += start handleIt(it);
        }
        await any list;
        abort all list;
    }
    "#;

    #[test]
    #[traced_test]
    fn test_file1() -> Result<(), Box<dyn std::error::Error>> {
        let (input, file) = crate::assembler::parser::parser::parse_x39file(TEST_FILE1)?;
        let vm_state = super::compiler::compile(file);
        trace!("{:?}", vm_state);
        Ok(())
    }

    const TEST_FILE2: &str = r#"
        list = [];
        for it in 0..20 {
            list += start handleIt(it);
        }
    "#;

    #[test]
    #[traced_test]
    fn test_file2() -> Result<(), Box<dyn std::error::Error>> {
        let (input, file) = crate::assembler::parser::parser::parse_x39file(TEST_FILE2)?;
        let vm_state = super::compiler::compile(file);
        trace!("{:?}", vm_state);
        Ok(())
    }

    const TEST_FILE3: &str = r#"
        if await fancy1() {
            exit;
        }
        else if await fancy2() {
            exit;
        }
        else {
            exit;
        }
    "#;

    #[test]
    #[traced_test]
    fn test_file3() -> Result<(), Box<dyn std::error::Error>> {
        let (input, file) = crate::assembler::parser::parser::parse_x39file(TEST_FILE3)?;
        let vm_state = super::compiler::compile(file);
        trace!("{:?}", vm_state);
        Ok(())
    }

    const TEST_FILE4: &str = r#"
    # comment
    if await conditionFunc() {
        exit;
    }
    else {
        # Something ain't working here
        collection = await generateFunc(12);
        list = [];
        for it in collection {
            list += start handleIt(it);
        }
        await all list;
        list = [];
        for it in 0..20 {
            list += start handleIt(it);
        }
        await any list;
        abort all list;
    }
    "#;

    #[test]
    #[traced_test]
    fn test_file4() -> Result<(), Box<dyn std::error::Error>> {
        let (input, file) = crate::assembler::parser::parser::parse_x39file(TEST_FILE4)?;
        let vm_state = super::compiler::compile(file);
        trace!("{:?}", vm_state);
        Ok(())
    }

    const TEST_FILE_IF_ELSE: &str = r#"
        exit;
        exit;
        exit;
        exit;
        if await fancy1() {
            exit;
        }
        else {
            exit;
        }
        exit;
    "#;

    #[test]
    #[traced_test]
    fn test_if_else() -> Result<(), Box<dyn std::error::Error>> {
        let (input, file) = crate::assembler::parser::parser::parse_x39file(TEST_FILE_IF_ELSE)?;
        let vm_state = super::compiler::compile(file);
        let expected_code = vec![
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // await fancy1()
            Instruction {
                opcode: OpCode::PushValueU16,
                arg: InstructionArg::Unsigned(0),
            },
            Instruction {
                opcode: OpCode::CallNoArg,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Await,
                arg: InstructionArg::Empty,
            },
            // if ... { ... }
            Instruction {
                opcode: OpCode::JumpIfFalse,
                arg: InstructionArg::Signed(3),
            },
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // if ... { ... }
            Instruction {
                opcode: OpCode::Jump,
                arg: InstructionArg::Signed(2),
            },
            // else { ... }
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
        ];
        assert_eq!(vm_state.instructions, expected_code);
        trace!("{:?}", vm_state);
        Ok(())
    }

    const TEST_FILE_IF_ELSE_IF_ELSE_IF_ELSE: &str = r#"
        exit;
        exit;
        exit;
        exit;
        if await fancy1() {
            exit;
        }
        else if await fancy1() {
            exit;
        }
        else if await fancy1() {
            exit;
        }
        else {
            exit;
        }
        exit;
    "#;

    #[test]
    #[traced_test]
    fn test_if_else_if_else_if_else() -> Result<(), Box<dyn std::error::Error>> {
        let (input, file) = crate::assembler::parser::parser::parse_x39file(TEST_FILE_IF_ELSE_IF_ELSE_IF_ELSE)?;
        let vm_state = super::compiler::compile(file);
        let expected_code = vec![
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // await fancy1()
            Instruction {
                opcode: OpCode::PushValueU16,
                arg: InstructionArg::Unsigned(0),
            },
            Instruction {
                opcode: OpCode::CallNoArg,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Await,
                arg: InstructionArg::Empty,
            },
            // if ... { ... }
            Instruction {
                opcode: OpCode::JumpIfFalse,
                arg: InstructionArg::Signed(3),
            },
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // if ... { ... }
            Instruction {
                opcode: OpCode::Jump,
                arg: InstructionArg::Signed(16),
            },
            // await fancy1()
            Instruction {
                opcode: OpCode::PushValueU16,
                arg: InstructionArg::Unsigned(0),
            },
            Instruction {
                opcode: OpCode::CallNoArg,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Await,
                arg: InstructionArg::Empty,
            },
            // if ... { ... }
            Instruction {
                opcode: OpCode::JumpIfFalse,
                arg: InstructionArg::Signed(3),
            },
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // if ... { ... }
            Instruction {
                opcode: OpCode::Jump,
                arg: InstructionArg::Signed(9),
            },
            // await fancy1()
            Instruction {
                opcode: OpCode::PushValueU16,
                arg: InstructionArg::Unsigned(0),
            },
            Instruction {
                opcode: OpCode::CallNoArg,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Await,
                arg: InstructionArg::Empty,
            },
            // if ... { ... }
            Instruction {
                opcode: OpCode::JumpIfFalse,
                arg: InstructionArg::Signed(3),
            },
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // if ... { ... }
            Instruction {
                opcode: OpCode::Jump,
                arg: InstructionArg::Signed(2),
            },
            // else { ... }
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
            // exit;
            Instruction {
                opcode: OpCode::PushNull,
                arg: InstructionArg::Empty,
            },
            Instruction {
                opcode: OpCode::Exit,
                arg: InstructionArg::Empty,
            },
        ];
        assert_eq!(vm_state.instructions, expected_code);
        trace!("{:?}", vm_state);
        Ok(())
    }
}
