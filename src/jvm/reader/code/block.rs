use crate::{jvm::reader::attribute::{Attributes}, io::BufferReadable, util::code_err::ClassParseError};

use super::{instruction::Instruction, exception_table::ExceptionTable};

#[derive(Debug)]
pub struct CodeBlock {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<Instruction>,
    pub exception_table: ExceptionTable,
    pub attributes: Attributes,
}

impl CodeBlock {
    pub fn load<R: BufferReadable>(buf: &mut R) -> Result<Self, ClassParseError> {
        let max_stack = buf.read_u2()?;
        let max_locals = buf.read_u2()?;
        let code_length = buf.read_u4()?;
        let mut code = Vec::new();
        let start_code = buf.stream_position().unwrap() + 1;
        for _ in 0..code_length {
            code.push(Instruction::load(buf, start_code)?);
        }
        
        let exception_table = ExceptionTable::load(buf)?;

        let attributes = Attributes::load(buf)?;
        Ok(Self {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        })
    }
}