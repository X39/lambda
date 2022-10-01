// Copyright x39

///
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
    /// POP a string and PUSH a variable
    GetVariable,
    /// POP a string and PUSH a variable after checking type::ARG or ERROR if variable is not
    /// of type.
    GetVariableOfType,
    /// POP a job and halt the execution until it completed.
    Await,
    /// POP an array of jobs and halt the execution until one of them completed.
    AwaitAny,
    /// POP an array of jobs and halt the execution until all have completed.
    AwaitAll,
    /// POP a string to interpret as function name and POP a value to pass and PUSH a job,
    /// executing the function, passing the argument.
    Call,
    /// POP a string to interpret as function name and PUSH a job,
    /// executing the function.
    CallVoid,
}
