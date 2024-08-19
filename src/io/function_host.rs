use std::io::{Read, Write};

pub trait FunctionHost<'a> : Sync + Send {
    fn writer(&mut self) -> &'a mut dyn Write;
    fn reader(&mut self) -> &'a mut dyn Read;
    fn terminate(&mut self);
}