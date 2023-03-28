use crate::{jvm::reader::attribute::{Attributes}, io::BufferReadable, util::code_err::ClassParseError};

use super::{instruction::Instruction, exception_table::ExceptionTable};


pub struct CodeBlock {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<Instruction>,
    pub exception_table: ExceptionTable,
    pub attributes: Attributes,
}

impl CodeBlock {
    pub fn load(buf: &mut Box<dyn BufferReadable>) -> Result<Self, ClassParseError> {
        let max_stack = buf.read_u2()?;
        let max_locals = buf.read_u2()?;
        let code_length = buf.read_u4()?;
        let mut code = Vec::new();
        for _ in 0..code_length {
            code.push(Instruction::load(buf)?);
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