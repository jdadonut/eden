use crate::{io::BufferReadable, util::code_err::ClassParseError};

use super::attribute::Attributes;


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
}

pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Attributes
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
        })
    }
}