// Copyright x39

///
pub enum OpCode {
    /// no operation
    NoOp,
    /// POP 0:string, POP 1:value and set a variable named 0 to value 1
    Assign,
    /// POP 0:array, POP 1:value and append value 1 to array 0
    Append,
    /// PUSH an empty array
    PushEmptyArray,
    /// PUSH a false value to stack
    PushFalse,
    /// PUSH a true value to stack
    PushTrue,
    /// PUSH a null value to stack
    PushNull,
    /// PUSH a value from the value list at index u16:ARG to stack
    PushValueU16,
    /// End processing
    Exit,
    /// POP 0:bool and jump relative to i16:ARG in the byte-code
    /// if 0 is true
    JumpOnTrueI16,
    /// POP 0:bool and jump relative to i16:ARG in the byte-code
    /// if 0 is false
    JumpOnFalseI16,
    /// Jump relative to i16:ARG in the byte-code if 0 is false
    JumpI16,
    /// Start the function named u16:ARG and PUSH job
    StartU16,
    /// POP 0:job and await completion, suspending execution
    /// until completed
    Await,
    /// POP 0:array and await completion of any job inside of it
    /// suspending execution until completed
    AwaitAny,
    /// POP 0:array and await completion of all jobs inside of it
    /// suspending execution until completed.
    AwaitAll,
}
