pub struct LambdaFile {
    functions: Vec<LambdaFunction>,
}
pub struct LambdaFunction {
    identifier: String,
    disabled: bool,
}
// ToDo: Implement parsing and create protocol classes