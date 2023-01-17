pub mod compiler {
    use std::borrow::{Borrow, BorrowMut};
    use tracing::trace;

    use crate::assembler::parser::parser::{AssignmentStatement, AssignmentType, AssignStatementData, AwaitCallOrIdentProduction, AwaitStatement, Call, CallValue, ElseStatement, ForLoopInstruction, ForLoopStatement, IfElseStatement, IfStatementCondition, NumericRange, Property, Statement, Value, X39File};
    use crate::machine::{Instruction, InstructionArg, VmState, VmValue, VmValueType};


    pub fn compile(file: X39File) -> VmState {
        let mut vm = VmState::new();
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
                    vm.push_instruction(Instruction::op_pop());
                }
                Statement::IfElse(if_else_statement) => compile_if_else(if_else_statement, vm.borrow_mut()),
                Statement::ForLoop(for_loop_statement) => compile_for_loop(for_loop_statement, vm.borrow_mut()),
                Statement::Assignment(assignment_statement) => compile_assignment(assignment_statement, vm.borrow_mut()),
                Statement::Print(ident) => compile_print(ident, vm.borrow_mut()),
            }
        }
    }

    fn compile_assignment(assignment_statement: &AssignmentStatement, vm: &mut VmState) {
        trace!("Entering compile_assignment with {} instructions", vm.instructions().len());
        let key = assignment_statement.ident.to_string();
        match assignment_statement.value.borrow() {
            AssignmentType::Append(append) => compile_assignment_append(append, key, vm),
            AssignmentType::Assign(assign) => compile_assignment_assign(assign, key, vm),
        }
        trace!("Exiting compile_assignment with {} instructions", vm.instructions().len());
    }

    fn compile_assignment_assign(assign: &AssignStatementData, ident: String, vm: &mut VmState) {
        trace!("Entering compile_assignment_assign with {} instructions", vm.instructions().len());
        // Reserve variable name index
        let value_index = vm.value_index(VmValue::String(ident));
        // PUSH the value to append on the stack
        match assign {
            AssignStatementData::Value(value) => compile_value(value, vm),
            AssignStatementData::Await(await_call_or_ident) => compile_await_call_or_ident(await_call_or_ident, vm),
            AssignStatementData::Start(start) => compile_start(start, vm),
        }
        // PUSH variable name to stack for assignment in the end
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        // Assign array to variable
        vm.push_instruction(Instruction::op_assign());
        trace!("Exiting compile_assignment_assign with {} instructions", vm.instructions().len());
    }

    fn compile_assignment_append(append: &AssignStatementData, ident: String, vm: &mut VmState) {
        trace!("Entering compile_assignment_append with {} instructions", vm.instructions().len());
        // Reserve variable name value index
        let value_index = vm.value_index(VmValue::String(ident));
        // PUSH array in variable to stack
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        vm.push_instruction(Instruction::op_get_variable_of_type(VmValueType::Array));
        // PUSH the value to append on the stack
        match append {
            AssignStatementData::Value(value) => compile_value(value, vm),
            AssignStatementData::Await(await_call_or_ident) => compile_await_call_or_ident(await_call_or_ident, vm),
            AssignStatementData::Start(start) => compile_start(start, vm),
        }
        // Append the value to the array
        vm.push_instruction(Instruction::op_append_array_push());
        // PUSH variable name to stack for assignment in the end
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        // Assign array to variable
        vm.push_instruction(Instruction::op_assign());
        trace!("Exiting compile_assignment_append with {} instructions", vm.instructions().len());
    }

    fn compile_for_loop(for_loop_statement: &ForLoopStatement, vm: &mut VmState) {
        trace!("Entering compile_for_loop with {} instructions", vm.instructions().len());
        // PUSH value to iterate over
        match for_loop_statement.over.borrow() {
            ForLoopInstruction::Ident(ident) => compile_ident(ident, vm),
            ForLoopInstruction::Await(await_call_or_ident) => compile_await_call_or_ident(await_call_or_ident, vm),
            ForLoopInstruction::Value(value) => compile_value(value, vm),
        }
        // PUSH index
        let value_index = vm.value_index(VmValue::Number(0.0));
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        // Prepare jump instruction
        let jump_offset = vm.instructions().len();
        vm.push_instruction(Instruction::op_jump_iterate(0));
        // PUSH variable name to stack for assignment in the end
        let ident = for_loop_statement.ident.to_string();
        let value_index = vm.value_index(VmValue::String(ident));
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        // Assign iterated element to variable
        vm.push_instruction(Instruction::op_assign());
        // Emit code
        compile_statements(for_loop_statement.code.borrow(), vm);
        // Emit jump back to loop
        let break_jump_offset = vm.instructions().len();
        vm.push_instruction(Instruction::op_jump(-((break_jump_offset - jump_offset) as i16)));
        // Update skip
        let next_offset = vm.instructions().len();
        vm.get_instruction(jump_offset).unwrap().arg = InstructionArg::Signed((next_offset - jump_offset) as i16);
        trace!("Exiting compile_for_loop with {} instructions", vm.instructions().len());
    }

    fn compile_if_else(if_else_statement: &IfElseStatement, vm: &mut VmState) {
        trace!("Entering compile_if_else with {} instructions", vm.instructions().len());
        // PUSH condition
        compile_if_statement_condition(if_else_statement.if_statement.condition.borrow(), vm);
        // Prepare jump instruction
        let true_offset = vm.instructions().len();
        vm.push_instruction(Instruction::op_jump_if_false(0));
        // Write out if code
        compile_statements(if_else_statement.if_statement.code.borrow(), vm);

        if let Some(else_statement) = if_else_statement.else_statement.borrow() {
            // Prepare else skip-jump
            let skip_offset = vm.instructions().len();
            vm.push_instruction(Instruction::op_jump(0));
            // Modify prepared jump instruction to correct offset
            let after_true_code_offset = vm.instructions().len();
            vm.get_instruction(true_offset).unwrap().arg = InstructionArg::Signed((after_true_code_offset - true_offset - 1) as i16);
            // Write out else code
            match else_statement {
                ElseStatement::Code(else_code) => compile_statements(else_code, vm),
                ElseStatement::IfElse(if_else) => compile_if_else(if_else, vm),
            }
            // Modify prepared jump instruction to correct offset
            let after_else_code_offset = vm.instructions().len();
            vm.get_instruction(skip_offset).unwrap().arg = InstructionArg::Signed((after_else_code_offset - skip_offset - 1) as i16);
        } else {
            // Modify prepared jump instruction to correct offset
            let after_true_code_offset = vm.instructions().len();
            vm.get_instruction(true_offset).unwrap().arg = InstructionArg::Signed((after_true_code_offset - true_offset - 1) as i16);
        }
        trace!("Exiting compile_if_else with {} instructions", vm.instructions().len());
    }

    fn compile_if_statement_condition(condition: &IfStatementCondition, vm: &mut VmState) {
        trace!("Entering compile_if_statement_condition with {} instructions", vm.instructions().len());
        match condition {
            IfStatementCondition::Await(await_call_or_ident) => compile_await_call_or_ident(await_call_or_ident, vm),
            IfStatementCondition::Ident(ident) => compile_ident(*ident, vm),
        }
        trace!("Exiting compile_if_statement_condition with {} instructions", vm.instructions().len());
    }

    fn compile_start(call: &Call, vm: &mut VmState) {
        trace!("Entering compile_start with {} instructions", vm.instructions().len());
        compile_call(call, vm);
        trace!("Exiting compile_start with {} instructions", vm.instructions().len());
    }

    fn compile_abort(abort_ident: &&str, vm: &mut VmState) {
        trace!("Entering compile_abort with {} instructions", vm.instructions().len());
        let value_index = vm.value_index(VmValue::String(abort_ident.to_string()));
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        vm.push_instruction(Instruction::op_get_variable_of_type(VmValueType::Job));
        vm.push_instruction(Instruction::op_abort());
        trace!("Exiting compile_abort with {} instructions", vm.instructions().len());
    }

    fn compile_print(ident: &&str, vm: &mut VmState) {
        trace!("Entering compile_print with {} instructions", vm.instructions().len());
        let value_index = vm.value_index(VmValue::String(ident.to_string()));
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        vm.push_instruction(Instruction::op_get_variable());
        vm.push_instruction(Instruction::op_print_to_console());
        trace!("Exiting compile_print with {} instructions", vm.instructions().len());
    }

    fn compile_abort_all(abort_ident: &&str, vm: &mut VmState) {
        trace!("Entering compile_abort with {} instructions", vm.instructions().len());
        let value_index = vm.value_index(VmValue::String(abort_ident.to_string()));
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        vm.push_instruction(Instruction::op_get_variable_of_type(VmValueType::Job));
        vm.push_instruction(Instruction::op_abort_all());
        trace!("Exiting compile_abort with {} instructions", vm.instructions().len());
    }

    fn compile_await(await_statement: &AwaitStatement, vm: &mut VmState) {
        trace!("Entering compile_await with {} instructions", vm.instructions().len());
        match await_statement {
            AwaitStatement::AwaitAny(await_any) => compile_await_any(await_any, vm),
            AwaitStatement::AwaitAll(await_all) => compile_await_all(await_all, vm),
            AwaitStatement::AwaitCallOrIdent(await_call_or_ident) => compile_await_call_or_ident(await_call_or_ident, vm),
        }
        trace!("Exiting compile_await with {} instructions", vm.instructions().len());
    }

    fn compile_await_call_or_ident(await_call_or_ident: &AwaitCallOrIdentProduction, vm: &mut VmState) {
        trace!("Entering compile_await_call_or_ident with {} instructions", vm.instructions().len());
        match await_call_or_ident {
            AwaitCallOrIdentProduction::Call(call) => compile_call(call, vm),
            AwaitCallOrIdentProduction::Ident(ident) => compile_ident_job(ident, vm),
        }
        vm.push_instruction(Instruction::op_await());
        trace!("Exiting compile_await_call_or_ident with {} instructions", vm.instructions().len());
    }

    fn compile_call(call: &Call, vm: &mut VmState) {
        trace!("Entering compile_call with {} instructions", vm.instructions().len());
        let value_index = vm.value_index(VmValue::String(call.ident.to_string()));
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        if let Some(value) = call.value.borrow() {
            compile_call_value(value, vm);
            vm.push_instruction(Instruction::op_call())
        } else {
            vm.push_instruction(Instruction::op_call_no_arg())
        }
        trace!("Exiting compile_call with {} instructions", vm.instructions().len());
    }

    fn compile_call_value(call_value: &CallValue, vm: &mut VmState) {
        trace!("Entering compile_call_value with {} instructions", vm.instructions().len());
        match call_value {
            CallValue::Ident(ident) => compile_ident(ident, vm),
            CallValue::Value(value) => compile_value(value, vm),
        }
        trace!("Exiting compile_call_value with {} instructions", vm.instructions().len());
    }

    fn compile_value(value: &Value, vm: &mut VmState) {
        trace!("Entering compile_value with {} instructions", vm.instructions().len());
        match value {
            Value::NumericRange(numeric_range) => compile_numeric_range(numeric_range, vm),
            Value::Number(number) => compile_number(*number, vm),
            Value::Null => compile_null(vm),
            Value::String(string) => compile_string(string.to_string(), vm),
            Value::Boolean(boolean) => compile_boolean(*boolean, vm),
            Value::Object(object) => compile_object(object, vm),
            Value::Array(array) => compile_array(array, vm),
        }
        trace!("Exiting compile_value with {} instructions", vm.instructions().len());
    }

    fn compile_array(array: &Vec<Value>, vm: &mut VmState) {
        trace!("Entering compile_array with {} instructions", vm.instructions().len());
        vm.push_instruction(Instruction::op_push_empty_array());

        for it in array.iter() {
            compile_value(it, vm);
            vm.push_instruction(Instruction::op_append_array_push());
        }
        trace!("Exiting compile_array with {} instructions", vm.instructions().len());
    }

    fn compile_object(object: &Vec<Property>, vm: &mut VmState) {
        trace!("Entering compile_object with {} instructions", vm.instructions().len());
        vm.push_instruction(Instruction::op_push_empty_object());

        for it in object.iter() {
            // Push Key
            let key = it.key.to_string();
            let value_index = vm.value_index(VmValue::String(key));
            vm.push_instruction(Instruction::op_push_value_u16(value_index));

            // Push Value
            let value = it.value.borrow();
            compile_value(value, vm);
            vm.push_instruction(Instruction::op_append_array_push());
        }
        trace!("Exiting compile_object with {} instructions", vm.instructions().len());
    }

    fn compile_numeric_range(numeric_range: &NumericRange, vm: &mut VmState) {
        trace!("Entering compile_numeric_range with {} instructions", vm.instructions().len());
        vm.push_instruction(Instruction::op_push_empty_array());

        for i in numeric_range.from as i64..numeric_range.to as i64 {
            let value_index = vm.value_index(VmValue::Number(i as f64));
            vm.push_instruction(Instruction::op_push_value_u16(value_index));
            vm.push_instruction(Instruction::op_append_array_push());
        }
        trace!("Exiting compile_numeric_range with {} instructions", vm.instructions().len());
    }

    fn compile_number(number: f64, vm: &mut VmState) {
        trace!("Entering compile_number with {} instructions", vm.instructions().len());
        let value_index = vm.value_index(VmValue::Number(number));
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        trace!("Exiting compile_number with {} instructions", vm.instructions().len());
    }

    fn compile_string(string: String, vm: &mut VmState) {
        trace!("Entering compile_string with {} instructions", vm.instructions().len());
        let value_index = vm.value_index(VmValue::String(string));
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        trace!("Exiting compile_string with {} instructions", vm.instructions().len());
    }

    fn compile_null(vm: &mut VmState) {
        trace!("Entering compile_null with {} instructions", vm.instructions().len());
        vm.push_instruction(Instruction::op_push_null());
        trace!("Exiting compile_null with {} instructions", vm.instructions().len());
    }

    fn compile_boolean(boolean: bool, vm: &mut VmState) {
        trace!("Entering compile_boolean with {} instructions", vm.instructions().len());
        vm.push_instruction(Instruction::op_push_boolean(boolean));
        trace!("Exiting compile_boolean with {} instructions", vm.instructions().len());
    }

    fn compile_ident(ident: &str, vm: &mut VmState) {
        trace!("Entering compile_ident with {} instructions", vm.instructions().len());
        let value_index = vm.value_index(VmValue::String(ident.to_string()));
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        vm.push_instruction(Instruction::op_get_variable());
        trace!("Exiting compile_ident with {} instructions", vm.instructions().len());
    }

    fn compile_ident_job(ident: &str, vm: &mut VmState) {
        trace!("Entering compile_ident_job with {} instructions", vm.instructions().len());
        let value_index = vm.value_index(VmValue::String(ident.to_string()));
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        vm.push_instruction(Instruction::op_get_variable_of_type(VmValueType::Job));
        trace!("Exiting compile_ident_job with {} instructions", vm.instructions().len());
    }

    fn compile_await_all(await_all: &str, vm: &mut VmState) {
        trace!("Entering compile_await_all with {} instructions", vm.instructions().len());
        let value_index = vm.value_index(VmValue::String(await_all.to_string()));
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        vm.push_instruction(Instruction::op_get_variable_of_type(VmValueType::ArrayOfJobs));
        vm.push_instruction(Instruction::op_await_all());
        trace!("Exiting compile_await_all with {} instructions", vm.instructions().len());
    }

    fn compile_await_any(await_any: &str, vm: &mut VmState) {
        trace!("Entering compile_await_any with {} instructions", vm.instructions().len());
        let value_index = vm.value_index(VmValue::String(await_any.to_string()));
        vm.push_instruction(Instruction::op_push_value_u16(value_index));
        vm.push_instruction(Instruction::op_get_variable_of_type(VmValueType::ArrayOfJobs));
        vm.push_instruction(Instruction::op_await_any());
        trace!("Exiting compile_await_any with {} instructions", vm.instructions().len());
    }

    fn compile_exit(vm: &mut VmState) {
        trace!("Entering compile_exit with {} instructions", vm.instructions().len());
        vm.push_instruction(Instruction::op_push_null());
        vm.push_instruction(Instruction::op_exit());
        trace!("Exiting compile_exit with {} instructions", vm.instructions().len());
    }
}

#[cfg(test)]
mod tests {
    use tracing::trace;
    use tracing_test::traced_test;
    use crate::machine::{Instruction};

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
        let (_, file) = crate::assembler::parser::parser::parse_x39file(TEST_FILE1)?;
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
        let (_, file) = crate::assembler::parser::parser::parse_x39file(TEST_FILE2)?;
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
        let (_, file) = crate::assembler::parser::parser::parse_x39file(TEST_FILE3)?;
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
        let (_, file) = crate::assembler::parser::parser::parse_x39file(TEST_FILE4)?;
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
        let (_, file) = crate::assembler::parser::parser::parse_x39file(TEST_FILE_IF_ELSE)?;
        let vm_state = super::compiler::compile(file);
        let expected_code = vec![
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // await fancy1()
            Instruction::op_push_value_u16(0),
            Instruction::op_call_no_arg(),
            Instruction::op_await(),
            // if ... { ... }
            Instruction::op_jump_if_false(3),
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // if ... { ... }
            Instruction::op_jump(2),
            // else { ... }
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
        ];
        assert_eq!(vm_state.instructions(), expected_code);
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
        let (_, file) = crate::assembler::parser::parser::parse_x39file(TEST_FILE_IF_ELSE_IF_ELSE_IF_ELSE)?;
        let vm_state = super::compiler::compile(file);
        let expected_code = vec![
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // await fancy1()
            Instruction::op_push_value_u16(0),
            Instruction::op_call_no_arg(),
            Instruction::op_await(),
            // if ... { ... }
            Instruction::op_jump_if_false(3),
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // if ... { ... }
            Instruction::op_jump(16),
            // await fancy1()
            Instruction::op_push_value_u16(0),
            Instruction::op_call_no_arg(),
            Instruction::op_await(),
            // if ... { ... }
            Instruction::op_jump_if_false(3),
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // if ... { ... }
            Instruction::op_jump(9),
            // await fancy1()
            Instruction::op_push_value_u16(0),
            Instruction::op_call_no_arg(),
            Instruction::op_await(),
            // if ... { ... }
            Instruction::op_jump_if_false(3),
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // if ... { ... }
            Instruction::op_jump(2),
            // else { ... }
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
            // exit;
            Instruction::op_push_null(),
            Instruction::op_exit(),
        ];
        assert_eq!(vm_state.instructions(), expected_code);
        trace!("{:?}", vm_state);
        Ok(())
    }
}
