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

    #[token("in")]
    In,

    // Or regular expressions.
    #[regex(r#""(.|\\")+""#, |lex| parseString(lex.slice()))]
    String(String),

    #[regex(r"[-+]?[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse(), priority = 2)]
    Number(f64),

    #[regex(r"[-_a-zA-Z][-_a-zA-Z0-9]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"//.*\n", logos::skip)]
    Error,
}

fn parseString(s: &str) -> String {
    let mut ret = String::new();
    let mut escape = false;
    for c in s.chars() {
        if escape {
            match c {
                '\\' => ret.push('\\'),
                'n' => ret.push('n'),
                'r' => ret.push('r'),
                'b' => ret.push('b'),
                't' => ret.push('t'),
                '"' => ret.push('"'),
                _ => ret.push(c),
            };
            escape = false;
        }
        else {
            match c {
                '\\' => escape = true,
                _ => ret.push(c),
            };
        }
    }
    return ret;
}