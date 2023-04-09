use crate::{io::BufferReadable, util::code_err::ClassParseError};

#[derive(Debug)]
pub struct Interfaces(Vec<u16>);

impl Interfaces {
    pub fn load<R: BufferReadable>(buf: &mut R) -> Result<Self, ClassParseError> {
        let interfaces_count = buf.read_u2()?;
        let mut interfaces = Vec::new();
        for _ in 0..interfaces_count {
            interfaces.push(buf.read_u2()?);
        }
        Ok(Self(interfaces))
    }
}