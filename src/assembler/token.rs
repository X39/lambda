// Copyright x39

use logos::Logos;
use uuid::Uuid;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
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
