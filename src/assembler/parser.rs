/// X39 File
#[derive(Debug)]
pub struct X39File<'a>(Vec<Statement<'a>>);

/// X39 Statement
#[derive(Debug)]
pub enum Statement<'a> {
    Await(AwaitStatement<'a>),
    Abort(&'a str),
    Exit,
    Start(Call<'a>),
    IfElse(IfElseStatement<'a>),
    ForLoopStatement,
    Assignment(AssignmentStatement<'a>),
}

#[derive(Debug)]
pub enum AssignmentType<'a> {
    Append(AssignStatementData<'a>),
    Assign(AssignStatementData<'a>),
}

#[derive(Debug)]
pub struct AssignmentStatement<'a> {
    ident: &'a str,
    value: AssignmentType<'a>,
}

#[derive(Debug)]
pub enum AssignStatementData<'a> {
    Value(Value<'a>),
    Await(AwaitCallOrIdentProduction<'a>),
    Start(Call<'a>),
}

#[derive(Debug)]
pub enum AwaitStatement<'a> {
    AwaitAny(&'a str),
    AwaitAll(&'a str),
    AwaitCallOrIdent(AwaitCallOrIdentProduction<'a>),
}

#[derive(Debug)]
pub enum AwaitCallOrIdentProduction<'a> {
    Call(Call<'a>),
    Ident(&'a str),
}

#[derive(Debug)]
pub struct IfElseStatement<'a> {
    if_statement: IfStatement<'a>,
    else_statement: Option<ElseStatement<'a>>,
}

#[derive(Debug)]
pub struct IfStatement<'a> {
    condition: IfStatementCondition<'a>,
    code: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub enum IfStatementCondition<'a> {
    Await(AwaitCallOrIdentProduction<'a>),
    Ident(&'a str),
}

#[derive(Debug)]
pub enum ElseStatement<'a> {
    Code(Vec<Statement<'a>>),
    IfElse(Box<IfElseStatement<'a>>),
}

#[derive(Debug)]
pub enum Value<'a> {
    NumericRange(NumericRange),
    Number(f64),
    Null,
    String(&'a str),
    Boolean(bool),
    Object(Vec<Property<'a>>),
    Array(Vec<Value<'a>>),
}

#[derive(Debug)]
pub struct Property<'a> {
    key: String,
    value: Value<'a>,
}

#[derive(Debug)]
pub struct Call<'a> {
    ident: &'a str,
    value: Option<Value<'a>>,
}

#[derive(Debug)]
pub struct NumericRange {
    from: f64,
    to: f64,
}

mod parser {
    use std::ops::Index;
    use std::str::FromStr;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, digit1, space0, space1};
    use nom::error::ParseError;
    use nom::{IResult, Map, Parser};
    use nom::character::streaming::alphanumeric1;
    use nom::combinator::{map, map_res, opt, recognize};
    use nom::multi::{fold_many0, many0, many1, separated_list0, separated_list1};
    use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
    use tracing::trace;
    use tracing_test::traced_test;
    use crate::assembler::parser_string::parse_string;
    use super::*;

    #[macro_export]
    macro_rules! delO {
        ($n:expr) => { delimited(space0, $n, space0) }
    }
    #[macro_export]
    macro_rules! delR {
        ($n:expr) => { delimited(space0, $n, space1) }
    }
    #[macro_export]
    macro_rules! semicolon {
        () => { preceded(space0, char(';')) }
    }

    pub fn parse_x39file(input: &str) -> IResult<&str, X39File> {
        // file ::= statements |;
        // statements ::= statement statements | statement;
        let (input, statements) = parse_statements(input)?;
        Ok((input, X39File(statements)))
    }

    pub fn parse_statements(input: &str) -> IResult<&str, Vec<Statement>> {
        // file ::= statements |;
        // statements ::= statement statements | statement;
        let (input, statements) = many0(delO!(parse_statement))(input)?;
        Ok((input, statements))
    }

    pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
        // statement ::= s_await | s_abort | s_exit | s_start | if_else | for | assignment;
        trace!("Entering parse_statement");
        let (input, statement) = alt((
            terminated(parse_await, semicolon!()),
            terminated(parse_abort, semicolon!()),
            terminated(parse_exit, semicolon!()),
            terminated(parse_start, semicolon!()),
            map(parse_if_else, |v| Statement::IfElse(v)),
            parse_for,
            parse_assign,
        ))(input)?;
        trace!("Exiting parse_statement with {:?}", statement);
        Ok((input, statement))
    }

    pub fn parse_assign(input: &str) -> IResult<&str, AssignmentStatement> {
        // assignment ::= IDENT PLUSEQUALS assignment_value | IDENT EQUALS assignment_value;
        trace!("Entering parse_assign");
        let (input, value) = tuple((
            delO!(alpha1),
            alt((
                preceded(tag("+="), map(parse_assign_value, |v| AssignmentType::Append(v))),
                preceded(tag("="), map(parse_assign_value, |v| AssignmentType::Assign(v))),
            ))))(input)?;
        trace!("Exiting parse_assign with {:?}", value);
        Ok((input, AssignmentStatement {
            ident: value.0,
            value: value.1,
        }))
    }

    pub fn parse_assign_value(input: &str) -> IResult<&str, AssignStatementData> {
        // assignment_value ::= value | AWAIT await_call_or_ident | start;
        trace!("Entering parse_assign_value");
        let (input, data) = alt((
            map(parse_await_call_or_ident, |v| AssignStatementData::Await(match v {
                AwaitStatement::AwaitCallOrIdent(s) => s,
                _ => panic!("Invalid program"),
            })),
            map(parse_start, |v| AssignStatementData::Start(match v {
                Statement::Start(s) => s,
                _ => panic!("Invalid program"),
            })),
            map(parse_value, |v| AssignStatementData::Value(v)),
        ))(input)?;
        trace!("Exiting parse_assign_value with {:?}", data);
        Ok((input, data))
    }

    pub fn parse_await(input: &str) -> IResult<&str, Statement> {
        // await ::= await await_any | await await_all | await await_call_or_ident;
        trace!("Entering parse_await");
        let (input, await_statement) = preceded(delR!(tag("await")), alt((
            parse_await_any,
            parse_await_all,
            parse_await_call_or_ident,
        )))(input)?;
        trace!("Exiting parse_await with {:?}", await_statement);
        Ok((input, Statement::Await(await_statement)))
    }

    pub fn parse_start(input: &str) -> IResult<&str, Statement> {
        // start ::= start call;
        trace!("Entering parse_start");
        let (input, call) = preceded(delR!(tag("start")), parse_call)(input)?;
        trace!("Exiting parse_start with {:?}", call);
        Ok((input, Statement::Start(call)))
    }

    pub fn parse_await_any(input: &str) -> IResult<&str, AwaitStatement> {
        // await_any ::= ANY IDENT;
        trace!("Entering parse_await_any");
        let (input, ident) = preceded(delR!(tag("any")), alpha1)(input)?;
        trace!("Exiting parse_await_any with {:?}", ident);
        Ok((input, AwaitStatement::AwaitAny(ident)))
    }

    pub fn parse_await_all(input: &str) -> IResult<&str, AwaitStatement> {
        // await_all ::= ALL IDENT;
        trace!("Entering parse_await_all");
        let (input, ident) = preceded(delR!(tag("all")), alpha1)(input)?;
        trace!("Exiting parse_await_all with {:?}", ident);
        Ok((input, AwaitStatement::AwaitAll(ident)))
    }

    pub fn parse_abort(input: &str) -> IResult<&str, Statement> {
        trace!("Entering parse_abort");
        let (input, ident) = preceded(delR!(tag("abort")), alpha1)(input)?;
        trace!("Exiting parse_abort with {:?}", ident);
        Ok((input, Statement::Abort(ident)))
    }

    pub fn parse_exit(input: &str) -> IResult<&str, Statement> {
        trace!("Entering parse_exit");
        let (input, ident) = tag("exit")(input)?;
        trace!("Exiting parse_exit with {:?}", ident);
        Ok((input, Statement::Exit))
    }

    pub fn parse_await_call_or_ident(input: &str) -> IResult<&str, AwaitStatement> {
        // await_call_or_ident ::= call | IDENT;
        trace!("Entering parse_await_call_or_ident");
        let (input, await_call_or_ident) = preceded(delR!(tag("await")), alt((
            parse_await_call,
            parse_await_ident,
        )))(input)?;
        trace!("Exiting parse_await_call_or_ident with {:?}", await_call_or_ident);
        Ok((input, AwaitStatement::AwaitCallOrIdent(await_call_or_ident)))
    }

    pub fn parse_await_call(input: &str) -> IResult<&str, AwaitCallOrIdentProduction> {
        // await_call_or_ident ::= call | IDENT;
        trace!("Entering parse_await_call");
        let (input, call) = parse_call(input)?;
        trace!("Exiting parse_await_call with {:?}", call);
        Ok((input, AwaitCallOrIdentProduction::Call(call)))
    }

    pub fn parse_call(input: &str) -> IResult<&str, Call> {
        // call ::= IDENT ROUNDOPEN value ROUNDCLOSE | IDENT ROUNDOPEN ROUNDCLOSE;
        trace!("Entering parse_call");
        let (input, ident) = alpha1(input)?;
        let (input, value) = alt((
            parse_call_with_value,
            parse_call_without_value,
        ))(input)?;
        trace!("Exiting parse_call with {:?} and {:?}", ident, value);
        Ok((input, Call {
            ident,
            value,
        }))
    }

    pub fn parse_call_with_value(input: &str) -> IResult<&str, Option<Value>> {
        trace!("Entering parse_call_with_value");
        let (input, value) = delimited(char('('), parse_value, char(')'))(input)?;
        trace!("Exiting parse_call_with_value with {:?}", value);
        Ok((input, Some(value)))
    }

    pub fn parse_call_without_value(input: &str) -> IResult<&str, Option<Value>> {
        trace!("Entering parse_call_without_value");
        let (input, _) = tuple((char('('), char(')')))(input)?;
        trace!("Exiting parse_call_without_value");
        Ok((input, None))
    }

    pub fn parse_await_ident(input: &str) -> IResult<&str, AwaitCallOrIdentProduction> {
        trace!("Entering parse_call_ident");
        let (input, ident) = alpha1(input)?;
        trace!("Exiting parse_call_ident with {:?}", ident);
        Ok((input, AwaitCallOrIdentProduction::Ident(ident)))
    }

    pub fn parse_value(input: &str) -> IResult<&str, Value> {
        // value ::= obj | array | numeric | constant;
        trace!("Entering parse_value");
        let (input, value) = delO!(alt((
            parse_obj,
            parse_array,
            parse_numeric,
            parse_constant,
        )))(input)?;
        trace!("Exiting parse_value with {:?}", value);
        Ok((input, value))
    }

    pub fn parse_obj(input: &str) -> IResult<&str, Value> {
        // obj ::= CURLYOPEN obj_data CURLYCLOSE | CURLYOPEN CURLYCLOSE;
        trace!("Entering parse_obj");
        let (input, value) = delimited(
            delO!(char('{')),
            terminated(
                separated_list0(char(','), parse_obj_data),
                opt(char(','))),
            delO!(char('}')))(input)?;
        trace!("Exiting parse_obj with {:?}", value);
        Ok((input, Value::Object(value)))
    }

    pub fn parse_obj_data(input: &str) -> IResult<&str, Property> {
        // obj_data ::= obj_prop COMMA obj_data | obj_prop COMMA | obj_prop;
        trace!("Entering parse_obj_data");
        let (input, value) =
            separated_pair(
                parse_string,
                delO!(char(':')),
                parse_value)(input)?;
        trace!("Exiting parse_obj_data with {:?}", value);
        Ok((input, Property {
            key: value.0,
            value: value.1,
        }))
    }

    pub fn parse_array(input: &str) -> IResult<&str, Value> {
        // array ::= SQUAREOPEN array_data SQUARECLOSE | SQUAREOPEN SQUARECLOSE;
        // array_data ::= IDENT COMMA array_data | value COMMA array_data | IDENT COMMA | value COMMA | IDENT | value;
        trace!("Entering parse_array");
        let (input, value) =
            delimited(
                delO!(char('[')),
                terminated(
                    separated_list0(char(','), parse_value),
                    opt(char(','))),
                delO!(char(']')))(input)?;
        trace!("Exiting parse_array with {:?}", value);
        Ok((input, Value::Array(value)))
    }

    pub fn parse_code(input: &str) -> IResult<&str, Vec<Statement>> {
        // code ::= CURLYOPEN statements CURLYCLOSE | CURLYOPEN CURLYCLOSE;
        trace!("Entering parse_code");
        let (input, value) =
            delimited(
                delO!(char('{')),
                parse_statements,
                delO!(char('}')))(input)?;
        trace!("Exiting parse_code with {:?}", value);
        Ok((input, value))
    }

    pub fn parse_numeric(input: &str) -> IResult<&str, Value> {
        // numeric ::= NUMBER DOTDOT NUMBER | NUMBER
        fn numeric_literal(input: &str) -> IResult<&str, f64> {
            let result = map_res(
                recognize(
                    tuple((digit1, char('.'), digit1))),
                |s: &str| f64::from_str(s))(input)?;
            Ok(result)
        }
        fn parse_range(input: &str) -> IResult<&str, Value> {
            let (input, from_to) = separated_pair(numeric_literal, tag(".."), numeric_literal)(input)?;
            return Ok((input, Value::NumericRange(NumericRange {
                from: from_to.0,
                to: from_to.1,
            })));
        }
        fn parse_number(input: &str) -> IResult<&str, Value> {
            let (input, value) = numeric_literal(input)?;
            return Ok((input, Value::Number(value)));
        }
        trace!("Entering parse_numeric");
        let (input, value) = delO!(alt((
            parse_range,
            parse_number,
        )))(input)?;
        trace!("Exiting parse_numeric with {:?}", value);
        Ok((input, value))
    }

    pub fn parse_constant(input: &str) -> IResult<&str, Value> {
        // constant ::= NULL | STRING | TRUE | FALSE;
        trace!("Entering parse_constant");
        let (input, value) = delO!(alt((
            parse_constant_null,
            parse_constant_string,
            parse_constant_true,
            parse_constant_false,
        )))(input)?;
        trace!("Exiting parse_constant with {:?}", value);
        Ok((input, value))
    }

    pub fn parse_constant_null(input: &str) -> IResult<&str, Value> {
        trace!("Entering parse_constant_null");
        let (input, value) = tag("null")(input)?;
        trace!("Exiting parse_constant_null with {:?}", value);
        Ok((input, Value::Null))
    }

    pub fn parse_constant_string(input: &str) -> IResult<&str, Value> {
        trace!("Entering parse_constant_string");
        let (input, value) = parse_string(input)?;
        trace!("Exiting parse_constant_string with {:?}", value);
        Ok((input, Value::Null))
    }

    pub fn parse_constant_true(input: &str) -> IResult<&str, Value> {
        trace!("Entering parse_constant_true");
        let (input, value) = tag("true")(input)?;
        trace!("Exiting parse_constant_true with {:?}", value);
        Ok((input, Value::Boolean(true)))
    }

    pub fn parse_constant_false(input: &str) -> IResult<&str, Value> {
        trace!("Entering parse_constant_false");
        let (input, value) = tag("false")(input)?;
        trace!("Exiting parse_constant_false with {:?}", value);
        Ok((input, Value::Boolean(false)))
    }

    pub fn parse_if_else(input: &str) -> IResult<&str, IfElseStatement> {
        // if_else ::= if else | if;
        trace!("Entering parse_if_else");
        let (input, value) = tuple((parse_if, opt(parse_else)))(input)?;
        trace!("Exiting parse_if_else with {:?}", value);
        Ok((input, IfElseStatement {
            if_statement: value.0,
            else_statement: value.1,
        }))
    }

    pub fn parse_if(input: &str) -> IResult<&str, IfStatement> {
        // if ::= if if_part;
        // if_part ::= if_part_await code | IDENT code;
        // if_part_await ::= await await_call_or_ident;
        trace!("Entering parse_if");
        let (input, value) = tuple((
            preceded(delR!(tag("if")), alt((
                map(parse_await_call_or_ident, |v| IfStatementCondition::Await(match v {
                    AwaitStatement::AwaitCallOrIdent(d) => d,
                    _ => panic!("Invalid program"),
                })),
                map(alpha1, |v| IfStatementCondition::Ident(v)),
            ))),
            parse_code,
        ))(input)?;
        trace!("Exiting parse_if with {:?}", value);
        Ok((input, IfStatement {
            code: value.1,
            condition: value.0,
        }))
    }

    pub fn parse_else(input: &str) -> IResult<&str, ElseStatement> {
        // else ::= else else_part;
        // else_part ::= if_else | code;
        trace!("Entering parse_else");
        let (input, value) =
            preceded(delR!(tag("else")), alt((
                map(parse_code, |v| ElseStatement::Code(v)),
                map(parse_if_else, |v| ElseStatement::IfElse(Box::new(IfElseStatement {
                    else_statement: v.else_statement,
                    if_statement: v.if_statement,
                }))),
            )))(input)?;
        trace!("Exiting parse_else with {:?}", value);
        Ok((input, value))
    }
}


#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    const TEST_FILE1: &str = r#"
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
    "#;

    const TEST_FILE2: &str = r#"
    await any foobar;
    "#;

    //#[test]
    fn test_file1() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_x39file(TEST_FILE1)?;
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_file2() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_x39file(TEST_FILE2)?;
        println!("{:?}", file.1);
        Ok(())
    }
}
