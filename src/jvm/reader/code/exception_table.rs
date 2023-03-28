use crate::{io::BufferReadable, util::code_err::ClassParseError};


pub struct ExceptionTable(pub Vec<ExceptionTableEntry>);

impl ExceptionTable {
    pub fn load(buf: &mut Box<dyn BufferReadable>) -> Result<Self, ClassParseError> {
        let exception_table_length = buf.read_u2()?;
        let mut exception_table = Vec::new();
        for _ in 0..exception_table_length {
            exception_table.push(ExceptionTableEntry::load(buf)?);
        }
        Ok(Self(exception_table))
    }
}

pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExceptionTableEntry {
    pub fn load(buf: &mut Box<dyn BufferReadable>) -> Result<Self, ClassParseError> {
        let start_pc = buf.read_u2()?;
        let end_pc = buf.read_u2()?;
        let handler_pc = buf.read_u2()?;
        let catch_type = buf.read_u2()?;
        Ok(Self {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }
}