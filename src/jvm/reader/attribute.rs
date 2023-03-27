use crate::{io::BufferReadable, util::code_err::ClassParseError};

// TODO: look more into this
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7

pub struct Attributes(pub Vec<AttributeInfo>);
impl Attributes {
    pub fn load(buf: &mut Box<dyn BufferReadable>) -> Result<Self, ClassParseError> {
        let attributes_count = buf.read_u2()?;
        let mut attributes = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::load(buf)?);
        }
        Ok(Self(attributes))
    }
}

pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub info: Vec<u8>,
}

impl AttributeInfo {
    pub fn load(buf: &mut Box<dyn BufferReadable>) -> Result<Self, ClassParseError> {
        let attribute_name_index = buf.read_u2()?;
        let attribute_length = buf.read_u4()?;
        let mut info = Vec::new();
        for _ in 0..attribute_length {
            info.push(buf.read_byte()?);
        }
        Ok(Self {
            attribute_name_index,
            attribute_length,
            info,
        })
    }
}
