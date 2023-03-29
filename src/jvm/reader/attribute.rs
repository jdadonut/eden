use crate::{io::BufferReadable, util::code_err::ClassParseError};

use super::constant_pool_info::{ConstantPool, ConstantPoolInfo};

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
    pub fn find_by_name(&self, name: &str, pool: &ConstantPool) -> Result<Option<&AttributeInfo>, ClassParseError> {
        for attribute in &self.0 {
            if attribute.name(pool)? == name {
                return Ok(Some(attribute));
            }
        }
        Ok(None)
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
    pub fn name(&self, constant_pool: &ConstantPool) -> Result<String, ClassParseError> {
        match constant_pool.get_java_aligned(self.attribute_name_index as usize) {
            Some(name) => {
                if let ConstantPoolInfo::Utf8(val) = name.info.clone() {
                    Ok(val.clone())
                } else {
                    Err(
                        ClassParseError::BadValue { 
                            expected: format!("Utf8 val in CP @ {} (jvm index: {})", self.attribute_name_index-1, self.attribute_name_index).to_string(),
                            got: format!("{:?}", name.info).to_string(),
                            for_what: format!("Attribute Name").to_string() 
                        }
                    )
                }
            }
            None => Err(
                ClassParseError::BadValue { 
                    expected: format!("Utf8 val in CP @ {} (jvm index: {})", self.attribute_name_index-1, self.attribute_name_index).to_string(),
                    got: format!("None @ lookup").to_string(),
                    for_what: format!("Attribute Name").to_string() 
                }
            ),
        }
    }
}
