// Copyright x39
use serde::{Serialize, Deserialize};

#[derive(Debug)]
#[derive(PartialEq, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub enum OpCode {
    /// no operation.
    NoOp,
    /// End processing.
    Exit,
    /// PUSH a value from the value list at index u16:ARG to stack.
    PushValueU16,
    /// PUSH a true value to stack.
    PushTrue,
    /// PUSH a false value to stack.
    PushFalse,
    /// PUSH a null value to stack.
    PushNull,
    /// PUSH a new, empty array to the stack.
    PushEmptyArray,
    /// PUSH a new, empty object to the stack.
    PushEmptyObject,
    /// POP a string and PUSH a variable
    GetVariable,
    /// POP a string and PUSH a variable after checking type::ARG or ERROR if variable is not
    /// of type.
    GetVariableOfType,
    /// POP a job and halt the execution until it completed.
    Await,
    /// POP a job and abort its scheduled execution if possible.
    Abort,
    /// POP an array of jobs and abort the scheduled execution of all if possible.
    AbortAll,
    /// POP an array of jobs and halt the execution until one of them completed.
    AwaitAny,
    /// POP an array of jobs and halt the execution until all have completed.
    AwaitAll,
    /// POP a string to interpret as function name and POP a value to pass and PUSH a job,
    /// executing the function, passing the argument.
    Call,
    /// POP a string to interpret as function name and PUSH a job,
    /// executing the function.
    CallNoArg,
    /// POP a value from the stack and POP an array from the stack and append the value
    /// to the array and PUSH the array back onto the stack.
    AppendArrayPush,
    /// POP a value from the stack and POP a string from the stack and POP an object from the stack
    /// and append a property to the object with the string as key and the value as value and
    /// PUSH the object back onto the stack.
    AppendPropertyPush,
    /// POP a string and POP a value and assign the value to a variable named as the string.
    Assign,
    /// POP a value from the stack and dispose of it immediate.
    Pop,
    /// Jump i16::ARG instructions.
    Jump,
    /// POP a boolean value from the stack and jump i16::ARG instructions if it is false.
    JumpIfFalse,
    /// POP a boolean value from the stack and jump i16::ARG instructions if it is true.
    JumpIfTrue,
    /// Specialized jump instruction for foreach support.
    /// -0: POP an index
    /// -1: POP an array or object
    /// If array or object has index elements:
    /// 0: PUSH array or object
    /// 1: PUSH index + 1
    /// 2: PUSH value at index of array or object
    /// If index out of range:
    /// Jump i16::ARG instructions.
    JumpIterate,
    /// POP 2 elements and PUSH them in reverse order.
    Swap2,
    /// POP a value and print it to console
    PrintToConsole,
}
