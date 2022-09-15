// Copyright x39

mod token;

pub use self::token::Token;

#[cfg(test)]
mod tests {
    use super::*;
    const SRC: &str = r#"
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
    }"#;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
