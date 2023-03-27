use crate::{io::BufferReadable, util::code_err::ClassParseError};


pub struct Interfaces(Vec<u16>);

impl Interfaces {
    pub fn load(buf: &mut Box<dyn BufferReadable>) -> Result<Self, ClassParseError> {
        let interfaces_count = buf.read_u2()?;
        let mut interfaces = Vec::new();
        for _ in 0..interfaces_count {
            interfaces.push(buf.read_u2()?);
        }
        Ok(Self(interfaces))
    }
}