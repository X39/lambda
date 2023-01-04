pub mod parser {
    /// X39 File
    #[derive(Debug)]
    pub struct X39File<'a> {
        pub statements: Vec<Statement<'a>>,
    }

    /// X39 Statement
    #[derive(Debug)]
    pub enum Statement<'a> {
        Await(AwaitStatement<'a>),
        Abort(&'a str),
        AbortAll(&'a str),
        Exit,
        Comment,
        Start(Call<'a>),
        IfElse(IfElseStatement<'a>),
        ForLoop(ForLoopStatement<'a>),
        Assignment(AssignmentStatement<'a>),
        Print(&'a str),
    }

    #[derive(Debug)]
    pub struct ForLoopStatement<'a> {
        pub ident: &'a str,
        pub over: ForLoopInstruction<'a>,
        pub code: Vec<Statement<'a>>,
    }

    #[derive(Debug)]
    pub enum ForLoopInstruction<'a> {
        Ident(&'a str),
        Await(AwaitCallOrIdentProduction<'a>),
        Value(Value),
    }

    #[derive(Debug)]
    pub enum AssignmentType<'a> {
        Append(AssignStatementData<'a>),
        Assign(AssignStatementData<'a>),
    }

    #[derive(Debug)]
    pub struct AssignmentStatement<'a> {
        pub ident: &'a str,
        pub value: AssignmentType<'a>,
    }

    #[derive(Debug)]
    pub enum AssignStatementData<'a> {
        Value(Value),
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
        pub if_statement: IfStatement<'a>,
        pub else_statement: Option<ElseStatement<'a>>,
    }

    #[derive(Debug)]
    pub struct IfStatement<'a> {
        pub condition: IfStatementCondition<'a>,
        pub code: Vec<Statement<'a>>,
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
    pub enum Value {
        NumericRange(NumericRange),
        Number(f64),
        Null,
        String(String),
        Boolean(bool),
        Object(Vec<Property>),
        Array(Vec<Value>),
    }

    #[derive(Debug)]
    pub struct Property {
        pub key: String,
        pub value: Value,
    }

    #[derive(Debug)]
    pub struct Call<'a> {
        pub ident: &'a str,
        pub value: Option<CallValue<'a>>,
    }

    #[derive(Debug)]
    pub enum CallValue<'a> {
        Ident(&'a str),
        Value(Value),
    }

    #[derive(Debug)]
    pub struct NumericRange {
        pub from: f64,
        pub to: f64,
    }

    use std::str::FromStr;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::alpha1;
    use nom::character::complete::anychar;
    use nom::character::complete::char;
    use nom::character::complete::digit1;
    use nom::character::complete::newline;
    use nom::character::complete::space1;
    use nom::error::ErrorKind;
    use nom::error::ParseError;
    use nom::AsChar;
    use nom::InputTakeAtPosition;
    use nom::IResult;
    use nom::character::complete::alphanumeric0;
    use nom::combinator::{complete, opt};
    use nom::combinator::map_res;
    use nom::combinator::map;
    use nom::combinator::recognize;
    use nom::multi::many0;
    use nom::multi::many_till;
    use nom::multi::separated_list0;
    use nom::sequence::{delimited, pair};
    use nom::sequence::preceded;
    use nom::sequence::separated_pair;
    use nom::sequence::terminated;
    use nom::sequence::tuple;
    use tracing::trace;
    use crate::assembler::parser_string::parse_string;

    #[macro_export]
    macro_rules! delO {
        ($n:expr) => { delimited(whitespace0, $n, whitespace0) }
    }
    #[macro_export]
    macro_rules! delR {
        ($n:expr) => { delimited(whitespace0, $n, space1) }
    }
    #[macro_export]
    macro_rules! semicolon {
        () => { delO!(char(';')) }
    }

    pub fn whitespace0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
        where
            T: InputTakeAtPosition,
            <T as InputTakeAtPosition>::Item: AsChar + Clone,
    {
        input.split_at_position_complete(|item| {
            let c = item.as_char();
            !(c == ' ' || c == '\t' || c == '\r' || c == '\n')
        })
    }

    pub fn whitespace1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
        where
            T: InputTakeAtPosition,
            <T as InputTakeAtPosition>::Item: AsChar + Clone,
    {
        input.split_at_position1_complete(
            |item| {
                let c = item.as_char();
                !(c == ' ' || c == '\t' || c == '\r' || c == '\n')
            },
            ErrorKind::Space,
        )
    }


    pub fn parse_x39file(input: &str) -> IResult<&str, X39File> {
        // file ::= statements |;
        let (input, statements) = complete(parse_statements)(input)?;
        Ok((input, X39File {
            statements,
        }))
    }

    pub fn parse_statements(input: &str) -> IResult<&str, Vec<Statement>> {
        // statements ::= statement statements | statement;
        trace!("Entering parse_statements with {:?}", input);
        let (input, statements) = many0(delO!(parse_statement))(input)?;
        trace!("Exiting parse_statements with {:?}", statements);
        Ok((input, statements))
    }

    pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
        // statement ::= s_await | s_abort | s_exit | s_start | if_else | for | assignment;
        trace!("Entering parse_statement with {:?}", input);
        let (input, statement) = alt((
            parse_comment,
            terminated(parse_await, semicolon!()),
            terminated(parse_print, semicolon!()),
            terminated(parse_abort, semicolon!()),
            terminated(parse_exit, semicolon!()),
            terminated(parse_start, semicolon!()),
            map(parse_if_else, |v| Statement::IfElse(v)),
            map(parse_for, |v| Statement::ForLoop(v)),
            map(terminated(parse_assign, semicolon!()), |v| Statement::Assignment(v)),
        ))(input)?;
        trace!("Exiting parse_statement with {:?}", statement);
        Ok((input, statement))
    }

    pub fn parse_comment(input: &str) -> IResult<&str, Statement> {
        // comment ::= # { ANY } NEWLINE
        trace!("Entering parse_comment with {:?}", input);
        let (input, _) = preceded(char('#'), many_till(anychar, newline))(input)?;
        trace!("Exiting parse_comment");
        Ok((input, Statement::Comment))
    }

    pub fn parse_ident(input: &str) -> IResult<&str, &str> {
        trace!("Entering parse_ident with {:?}", input);
        let (input, value) = recognize(
            pair(
                alpha1,
                alphanumeric0,
            ))(input)?;
        trace!("Exiting parse_ident");
        Ok((input, value))
    }

    pub fn parse_for<'a>(input: &'a str) -> IResult<&str, ForLoopStatement<'a>> {
        // for ::= FOR IDENT IN for_variant code;
        trace!("Entering parse_for with {:?}", input);
        let (input, value) = tuple((
            preceded(delR!(tag("for")), delR!(parse_ident)),
            preceded(delR!(tag("in")), parse_for_instruction),
            parse_code,
        ))(input)?;
        trace!("Exiting parse_for with {:?}", value);
        Ok((input, ForLoopStatement {
            ident: value.0,
            over: value.1,
            code: value.2,
        }))
    }

    pub fn parse_for_instruction<'a>(input: &'a str) -> IResult<&str, ForLoopInstruction<'a>> {
        // for_variant ::=  array | for_variant_await | IDENT;
        // for_variant_await ::= AWAIT await_call_or_ident;
        trace!("Entering parse_for_instruction with {:?}", input);
        let (input, value) = alt((
            map(parse_await_call_or_ident, |v| ForLoopInstruction::Await(match v {
                AwaitStatement::AwaitCallOrIdent(s) => s,
                _ => panic!("Invalid program"),
            })),
            map(parse_ident, |v| ForLoopInstruction::Ident(v)),
            map(parse_value, |v| ForLoopInstruction::Value(v)),
        ))(input)?;
        trace!("Exiting parse_for_instruction with {:?}", value);
        Ok((input, value))
    }

    pub fn parse_assign(input: &str) -> IResult<&str, AssignmentStatement> {
        // assignment ::= IDENT PLUSEQUALS assignment_value | IDENT EQUALS assignment_value;
        trace!("Entering parse_assign with {:?}", input);
        let (input, value) = tuple((
            delO!(parse_ident),
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
        trace!("Entering parse_assign_value with {:?}", input);
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
        trace!("Entering parse_await with {:?}", input);
        let (input, await_statement) = alt((
            parse_await_any,
            parse_await_all,
            parse_await_call_or_ident,
        ))(input)?;
        trace!("Exiting parse_await with {:?}", await_statement);
        Ok((input, Statement::Await(await_statement)))
    }

    pub fn parse_print(input: &str) -> IResult<&str, Statement> {
        // start ::= start call;
        trace!("Entering parse_print with {:?}", input);
        let (input, ident) = preceded(delR!(tag("print")), parse_ident)(input)?;
        trace!("Exiting parse_print with {:?}", ident);
        Ok((input, Statement::Print(ident)))
    }

    pub fn parse_start(input: &str) -> IResult<&str, Statement> {
        // start ::= start call;
        trace!("Entering parse_start with {:?}", input);
        let (input, call) = preceded(
            delR!(tag("start")),
            parse_call)(input)?;
        trace!("Exiting parse_start with {:?}", call);
        Ok((input, Statement::Start(call)))
    }

    pub fn parse_await_any(input: &str) -> IResult<&str, AwaitStatement> {
        // await_any ::= ANY IDENT;
        trace!("Entering parse_await_any with {:?}", input);
        let (input, ident) = preceded(
            tuple((delR!(tag("await")), delR!(tag("any")))),
            parse_ident)(input)?;
        trace!("Exiting parse_await_any with {:?}", ident);
        Ok((input, AwaitStatement::AwaitAny(ident)))
    }

    pub fn parse_await_all(input: &str) -> IResult<&str, AwaitStatement> {
        // await_all ::= ALL IDENT;
        trace!("Entering parse_await_all with {:?}", input);
        let (input, ident) = preceded(
            tuple((delR!(tag("await")), delR!(tag("all")))),
            parse_ident)(input)?;
        trace!("Exiting parse_await_all with {:?}", ident);
        Ok((input, AwaitStatement::AwaitAll(ident)))
    }

    pub fn parse_abort(input: &str) -> IResult<&str, Statement> {
        trace!("Entering parse_abort with {:?}", input);
        let (input, abort) = preceded(delR!(tag("abort")), alt((
            preceded(delR!(tag("all")), map(parse_ident, |ident| Statement::AbortAll(ident))),
            map(parse_ident, |ident| Statement::Abort(ident)),
        )))(input)?;
        trace!("Exiting parse_abort with {:?}", abort);
        Ok((input, abort))
    }

    pub fn parse_exit(input: &str) -> IResult<&str, Statement> {
        trace!("Entering parse_exit with {:?}", input);
        let (input, ident) = tag("exit")(input)?;
        trace!("Exiting parse_exit with {:?}", ident);
        Ok((input, Statement::Exit))
    }

    pub fn parse_await_call_or_ident(input: &str) -> IResult<&str, AwaitStatement> {
        // await_call_or_ident ::= call | IDENT;
        trace!("Entering parse_await_call_or_ident with {:?}", input);
        let (input, await_call_or_ident) = preceded(
            delR!(tag("await")),
            alt((
                parse_await_call,
                parse_await_ident,
            )))(input)?;
        trace!("Exiting parse_await_call_or_ident with {:?}", await_call_or_ident);
        Ok((input, AwaitStatement::AwaitCallOrIdent(await_call_or_ident)))
    }

    pub fn parse_await_call(input: &str) -> IResult<&str, AwaitCallOrIdentProduction> {
        // await_call_or_ident ::= call | IDENT;
        trace!("Entering parse_await_call with {:?}", input);
        let (input, call) = parse_call(input)?;
        trace!("Exiting parse_await_call with {:?}", call);
        Ok((input, AwaitCallOrIdentProduction::Call(call)))
    }

    pub fn parse_call(input: &str) -> IResult<&str, Call> {
        // call ::= IDENT ROUNDOPEN value ROUNDCLOSE | IDENT ROUNDOPEN ROUNDCLOSE;
        trace!("Entering parse_call with {:?}", input);
        let (input, ident) = parse_ident(input)?;
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

    pub fn parse_call_with_value(input: &str) -> IResult<&str, Option<CallValue>> {
        trace!("Entering parse_call_with_value with {:?}", input);
        let (input, value) = delimited(
            delO!(char('(')),
            alt((
                map(parse_value, |v| CallValue::Value(v)),
                map(parse_ident, |v| CallValue::Ident(v)),
            )),
            delO!(char(')')),
        )(input)?;
        trace!("Exiting parse_call_with_value with {:?}", value);
        Ok((input, Some(value)))
    }

    pub fn parse_call_without_value(input: &str) -> IResult<&str, Option<CallValue>> {
        trace!("Entering parse_call_without_value with {:?}", input);
        let (input, _) = tuple((char('('), char(')')))(input)?;
        trace!("Exiting parse_call_without_value");
        Ok((input, None))
    }

    pub fn parse_await_ident(input: &str) -> IResult<&str, AwaitCallOrIdentProduction> {
        trace!("Entering parse_call_ident with {:?}", input);
        let (input, ident) = parse_ident(input)?;
        trace!("Exiting parse_call_ident with {:?}", ident);
        Ok((input, AwaitCallOrIdentProduction::Ident(ident)))
    }

    pub fn parse_value(input: &str) -> IResult<&str, Value> {
        // value ::= obj | array | numeric | constant;
        trace!("Entering parse_value with {:?}", input);
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
        trace!("Entering parse_obj with {:?}", input);
        let (input, value) = delimited(
            delO!(char('{')),
            terminated(
                separated_list0(delO!(char(',')), parse_obj_data),
                opt(char(','))),
            delO!(char('}')))(input)?;
        trace!("Exiting parse_obj with {:?}", value);
        Ok((input, Value::Object(value)))
    }

    pub fn parse_obj_data(input: &str) -> IResult<&str, Property> {
        // obj_data ::= obj_prop COMMA obj_data | obj_prop COMMA | obj_prop;
        trace!("Entering parse_obj_data with {:?}", input);
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
        trace!("Entering parse_array with {:?}", input);
        let (input, value) =
            delimited(
                delO!(char('[')),
                parse_array_body,
                delO!(char(']')))(input)?;
        trace!("Exiting parse_array with {:?}", value);
        Ok((input, Value::Array(value)))
    }

    pub fn parse_array_body(input: &str) -> IResult<&str, Vec<Value>> {
        // array_data ::= IDENT COMMA array_data | value COMMA array_data | IDENT COMMA | value COMMA | IDENT | value;
        trace!("Entering parse_array_body with {:?}", input);
        let (input, value) =
            terminated(
                separated_list0(delO!(char(',')), parse_value),
                opt(delO!(char(','))))(input)?;
        trace!("Exiting parse_array_body with {:?}", value);
        Ok((input, value))
    }

    pub fn parse_code(input: &str) -> IResult<&str, Vec<Statement>> {
        // code ::= CURLYOPEN statements CURLYCLOSE | CURLYOPEN CURLYCLOSE;
        trace!("Entering parse_code with {:?}", input);
        let (input, value) =
            delimited(
                delO!(char('{')),
                parse_statements,
                delO!(char('}')))(input)?;
        trace!("Exiting parse_code with {:?}", value);
        Ok((input, value))
    }

    pub fn parse_numeric_literal(input: &str) -> IResult<&str, f64> {
        trace!("Entering parse_numeric_literal with {:?}", input);
        let (input, value) = map_res(
            alt((
                parse_numeric_literal_double,
                parse_numeric_literal_integer,
            )),
            |s: &str| f64::from_str(s))(input)?;
        trace!("Exiting parse_numeric_literal with {:?}", value);
        Ok((input, value))
    }

    pub fn parse_numeric_literal_double(input: &str) -> IResult<&str, &str> {
        trace!("Entering parse_numeric_literal_double with {:?}", input);
        let (input, value) = recognize(tuple((digit1, char('.'), digit1)))(input)?;
        trace!("Exiting parse_numeric_literal_double with {:?}", value);
        Ok((input, value))
    }

    pub fn parse_numeric_literal_integer(input: &str) -> IResult<&str, &str> {
        trace!("Entering parse_numeric_literal_integer with {:?}", input);
        let (input, value) = recognize(digit1)(input)?;
        trace!("Exiting parse_numeric_literal_integer with {:?}", value);
        Ok((input, value))
    }

    fn parse_numeric_range(input: &str) -> IResult<&str, Value> {
        trace!("Entering parse_numeric_range with {:?}", input);
        let (input, from_to) = separated_pair(
            parse_numeric_literal,
            tag(".."),
            parse_numeric_literal)(input)?;
        trace!("Exiting parse_numeric_range with {:?}", from_to);
        return Ok((input, Value::NumericRange(NumericRange {
            from: from_to.0,
            to: from_to.1,
        })));
    }

    pub fn parse_numeric(input: &str) -> IResult<&str, Value> {
        // numeric ::= NUMBER DOTDOT NUMBER | NUMBER
        trace!("Entering parse_numeric with {:?}", input);
        let (input, value) = delO!(alt((
            parse_numeric_range,
            map(parse_numeric_literal, |v| Value::Number(v)),
        )))(input)?;
        trace!("Exiting parse_numeric with {:?}", value);
        Ok((input, value))
    }

    pub fn parse_constant(input: &str) -> IResult<&str, Value> {
        // constant ::= NULL | STRING | TRUE | FALSE;
        trace!("Entering parse_constant with {:?}", input);
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
        trace!("Entering parse_constant_null with {:?}", input);
        let (input, value) = tag("null")(input)?;
        trace!("Exiting parse_constant_null with {:?}", value);
        Ok((input, Value::Null))
    }

    pub fn parse_constant_string(input: &str) -> IResult<&str, Value> {
        trace!("Entering parse_constant_string with {:?}", input);
        let (input, value) = parse_string(input)?;
        trace!("Exiting parse_constant_string with {:?}", value);
        Ok((input, Value::String(value)))
    }

    pub fn parse_constant_true(input: &str) -> IResult<&str, Value> {
        trace!("Entering parse_constant_true with {:?}", input);
        let (input, value) = tag("true")(input)?;
        trace!("Exiting parse_constant_true with {:?}", value);
        Ok((input, Value::Boolean(true)))
    }

    pub fn parse_constant_false(input: &str) -> IResult<&str, Value> {
        trace!("Entering parse_constant_false with {:?}", input);
        let (input, value) = tag("false")(input)?;
        trace!("Exiting parse_constant_false with {:?}", value);
        Ok((input, Value::Boolean(false)))
    }

    pub fn parse_if_else(input: &str) -> IResult<&str, IfElseStatement> {
        // if_else ::= if else | if;
        trace!("Entering parse_if_else with {:?}", input);
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
        trace!("Entering parse_if with {:?}", input);
        let (input, value) = tuple((
            preceded(delR!(tag("if")), alt((
                map(parse_await_call_or_ident, |v| IfStatementCondition::Await(match v {
                    AwaitStatement::AwaitCallOrIdent(d) => d,
                    _ => panic!("Invalid program"),
                })),
                map(parse_ident, |v| IfStatementCondition::Ident(v)),
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
        trace!("Entering parse_else with {:?}", input);
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

    const TEST_FILE3: &str = r#"
    if await conditionFunc(result) {
        exit;
    }
    else if await conditionFunc2(result) {
        exit;
    }
    else {
        await generateFunc(12);
    }
    "#;

    const TEST_FILE4: &str = r#"
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
        abort all list;"#;

    #[test]
    #[traced_test]
    fn test_file1() -> Result<(), Box<dyn std::error::Error>> {
        // cargo test -- --nocapture
        let file = super::parser::parse_x39file(TEST_FILE1)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_file2() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_x39file(TEST_FILE2)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_file3() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_x39file(TEST_FILE3)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_file4() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_x39file(TEST_FILE4)?;
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_comment_with_contents() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_comment("#asdasdasdasd\n")?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_comment_empty() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_comment("#\n")?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_obj_empty_1() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_obj("{}")?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_obj_empty_2() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_obj("{ }")?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_obj_single_data() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_obj(r#"{ "foo": "bar" }"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_obj_multi_data() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_obj(r#"{ "foo": "bar", "bar" :"foo" }"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_obj_multi_data_comma_terminated() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_obj(r#"{ "foo": "bar", "bar" :"foo" ,}"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_array_empty_1() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_array(r#"[]"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_array_empty_2() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_array(r#"[ ]"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_array_single_value_1() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_array(r#"[ 1 ]"#)?;
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_array_single_value_2() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_array(r#"[2]"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_array_multi_value() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_array(r#"[1,2]"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_array_multi_value_comma_terminated() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_array(r#"[1, 2 , 3,]"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_array_body_single_value_1() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_array_body(r#"1"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_array_body_single_value_2() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_array_body(r#" 2 "#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_array_body_multi_value() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_array_body(r#"1,2"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_array_body_multi_value_comma_terminated() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_array_body(r#"1, 2 , 3,"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_numeric_literal_int() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_numeric_literal(r#"1"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_numeric_literal_float() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_numeric_literal(r#"1.5"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_value_numeric_literal_int() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_value(r#"1"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_value_numeric_literal_float() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_value(r#"1.5"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_value_numeric_range() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_value(r#"1..5"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_await_call_or_ident_with_ident() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_await_call_or_ident(r#"await ident"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_await_with_ident() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_await(r#"await ident"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_await_with_call_alpha() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_await(r#"await call({})"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_await_with_call_alphanumeric() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_await(r#"await call123({})"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_assign_await_call_alpha() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_assign(r#"ident = await call({})"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_assign_await_call_alphanumeric() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_assign(r#"ident123 = await call123({})"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }


    #[test]
    #[traced_test]
    fn test_parse_statement_with_await_ident() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_statement(r#"await ident;"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_statements_with_double_await_ident() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_statements(r#"await ident; await ident;"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_statements_with_assign_start_and_await() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_statements(r#"ident = start foo({}); await ident;"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_statement_with_assign_await_call() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_statement(r#"ident = await foo({});"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_statements_with_assign_await_call_twice() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_statements(r#"ident = await foo({}); ident = await foo({});"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_statements_with_if_else_chain_twice() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_statements(r#"if await foo {} else if await bar {} else {} if await foo {} else if await bar {} else {}"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_statement_with_if_else_chain() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_statements(r#"if await foo {} else if await bar {} else {}"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_statement_with_if_else_both_content() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_statements(r#"if await foo { exit; } else {exit;}"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_call_with_value_from_ident() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_call(r#"foo(ident)"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_call_with_no_value() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_call(r#"foo()"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_if_else_chain() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_if_else(r#"if await foo {} else if await bar {} else {}"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_if_else_chain_exit() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_if_else(r#"if await foo {exit;} else if await bar {exit;} else {exit;}"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_ident_alpha() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_ident(r#"abcdefghijklmnopqrstuvwxyz"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_ident_alphanumeric() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_ident(r#"abcdefghijklmnopqrstuvwxyz0123456789"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_constant_string() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_constant_string(r#""foobar""#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_constant_with_true() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_constant(r#"true"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_constant_with_false() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_constant(r#"false"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_constant_with_null() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_constant(r#"null"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_constant_with_string() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_constant(r#""foobar""#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }

    #[test]
    #[traced_test]
    fn test_parse_for_in_range() -> Result<(), Box<dyn std::error::Error>> {
        let file = super::parser::parse_for(r#"for it in 0..20 { list += start handleIt(it); }"#)?;
        if !file.0.is_empty()
        { return Err(Box::from("File not fully yielded")); }
        println!("{:?}", file.1);
        Ok(())
    }
}
