#![allow(dead_code)]
use logos::Logos;
use uuid::Uuid;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("..")]
    DotDot,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token("=")]
    Equal,

    #[token("+=")]
    PlusEqual,

    #[token("!")]
    Not,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("{")]
    CurlyOpen,

    #[token("}")]
    CurlyClose,

    #[token("(")]
    RoundOpen,

    #[token(")")]
    RoundClose,

    #[token("[")]
    SquareOpen,

    #[token("]")]
    SquareClose,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("for")]
    For,

    #[token("start")]
    Start,

    #[token("abort")]
    Abort,

    #[token("await")]
    Await,

    #[token("any")]
    Any,

    #[token("all")]
    All,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("exit")]
    Exit,

    #[token("break")]
    Break,

    #[token("null")]
    Null,

    // Or regular expressions.
    #[regex(r#""(.|\\")+""#)]
    String,

    #[regex(r"[-+]?[0-9]+(\.[0-9]+)?", priority = 2)]
    Number,

    #[regex(r"[-_a-zA-Z][-_a-zA-Z0-9]*")]
    Identifier,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"//.*\n", logos::skip)]
    Error,
}

struct VirtualMachine {

}
enum OpCode {
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
union InstructionArg {
    unsigned: u16,
    signed: i16,
}
struct Instruction {
    opcode: OpCode,
}
struct VmPair {
    key: String,
    value: Option<VmValue>,
}
struct VmObject {
    values: Vec<VmValue>,
}
struct VmArray {
    values: Vec<Option<VmValue>>,
}
struct VmNumber {
    value: f64,
}
struct VmBoolean {
    value: bool,
}
struct VmString {
    value: String,
}
enum VmValue {
    String(VmString),
    Number(VmNumber),
    Array(VmArray),
    Boolean(VmBoolean),
    Object(VmObject),
    Job(Uuid),
}
struct VmState {
    value_list: Vec<Option<VmValue>>,
    function_list: Vec<String>,
    instructions: Vec<Instruction>,
    instruction_index: u32,
}

fn main() {
    let mut lexer = Token::lexer("");
}
/*
    // comment
    variable = start func({
        "foo": false,
        "bar": "test",
        "foobar": 1.2,
        "other": null,
        "array": [1,2,3,4]
    });
    await variable;
    result = await func2(variable);
    if await conditionFunc(result) {
        exit;
    }
    else if conditionFunc2(result) {
        exit;
    }
    else {
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
 */
