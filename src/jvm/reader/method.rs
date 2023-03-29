use crate::{io::{BufferReadable, Prebuffer}, util::code_err::{ClassParseError, CodeParseError}};

use super::{attribute::Attributes, code::block::CodeBlock, constant_pool_info::ConstantPool};


pub struct Methods(pub Vec<MethodInfo>);

impl Methods {
    pub fn load(buf: &mut Box<dyn BufferReadable>) -> Result<Self, ClassParseError> {
        let methods_count = buf.read_u2()?;
        let mut methods = Vec::new();
        for _ in 0..methods_count {
            methods.push(MethodInfo::load(buf)?);
        }
        Ok(Self(methods))
    }
    pub fn load_code(&mut self, constant_pool: &ConstantPool) -> Result<(), ClassParseError> {
        for method in &mut self.0 {
            method.load_code(constant_pool)?;
        }
        Ok(())
    }
}

pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Attributes,
    pub code: Option<CodeBlock>,
}

impl MethodInfo {
    pub fn load(buf: &mut Box<dyn BufferReadable>) -> Result<Self, ClassParseError> {
        let access_flags = buf.read_u2()?;
        let name_index = buf.read_u2()?;
        let descriptor_index = buf.read_u2()?;
        let attributes = Attributes::load(buf)?;
        Ok(Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
            code: None,
        })
    }
    pub fn load_code(&mut self, constant_pool: &ConstantPool) -> Result<(), ClassParseError> {
        match self.attributes.find_by_name("Code", constant_pool)? {
            Some(attr) => {
                self.code = Some(CodeBlock::load(&mut  ((box Prebuffer::copy_from_vec(&attr.info)) as Box<dyn BufferReadable>))?);
            }
            None => {
                return Err(
                    ClassParseError::CodeParseError {
                        internal: CodeParseError::CodeEntryNotFound,
                        classpath: None,
                        signature: None }
                )
            }
        }
        Ok(())
    }
    
}