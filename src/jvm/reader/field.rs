use crate::{io::BufferReadable, util::code_err::ClassParseError};

use super::attribute::Attributes;


pub struct Fields(pub Vec<FieldInfo>);

impl Fields {
    pub fn load(buf: &mut Box<dyn BufferReadable>) -> Result<Self, ClassParseError> {
        let fields_count = buf.read_u2()?;
        let mut fields = Vec::new();
        for _ in 0..fields_count {
            fields.push(FieldInfo::load(buf)?);
        }
        Ok(Self(fields))
    }
}


pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Attributes,
}
impl FieldInfo {
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

