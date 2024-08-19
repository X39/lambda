use std::io::{Read, Write};
use crate::io::FunctionHost;

struct ProcessFunctionHost {

}
impl<'a> FunctionHost<'a> for ProcessFunctionHost {
    fn writer(&'_ mut self) -> &'a mut dyn Write {
        panic!()
    }

    fn reader(&'_ mut self) -> &'a mut dyn Read {
        panic!()
    }

    fn terminate(&'_ mut self) {
        panic!()
    }
}