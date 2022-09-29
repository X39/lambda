mod compiler {
    use std::borrow::{Borrow, BorrowMut};
    use crate::assembler::parser::parser::{AssignmentStatement, AwaitStatement, Call, ForLoopStatement, IfElseStatement, Statement, X39File};
    use crate::machine::{Instruction, InstructionArg, OpCode, VirtualMachine, VmState, VmValue};


    pub fn compile(file: X39File) -> VmState {
        let mut vm = VmState{
            value_list: vec![],
            function_list: vec![],
            instructions: vec![],
            instruction_index: 0
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

    fn compile_assignment(assignmentStatement: &AssignmentStatement, vm: &mut VmState) {
        todo!()
    }

    fn compile_for_loop(forLoopStatement: &ForLoopStatement, vm: &mut VmState) {
        todo!()
    }

    fn compile_if_else(ifElseStatement: &IfElseStatement, vm: &mut VmState) {
        todo!()
    }

    fn compile_start(call: &Call, vm: &mut VmState) {
        todo!()
    }

    fn compile_abort(abortIdent: &&str, vm: &mut VmState) {
        todo!()
    }

    fn compile_await(awaitStatement: &AwaitStatement, vm: &mut VmState) {
        todo!()
    }

    fn compile_exit(vm: &mut VmState) {
        let value_index = util_get_value_index(VmValue::Number(0 as f64), vm.borrow_mut());
        vm.instructions.push(Instruction {
            opcode: OpCode::PushValueU16,
            arg: InstructionArg::Unsigned(value_index)
        });
        vm.instructions.push(Instruction {
            opcode: OpCode::Exit,
            arg: InstructionArg::Empty,
        })
    }
}