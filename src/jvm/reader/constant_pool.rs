use std::error::Error;
use crate::jvm::reader::constant_pool_info::ConstantPoolEntry;

pub struct ConstantPool(Vec<ConstantPoolEntry>);

impl ConstantPool {
    /// read constant pool from a buffer
    /// returns constant pool and number of bytes read
    pub fn from_buffer(buf: &[u8]) -> Result<(Self, usize), Box<dyn Error>> {
        let len = u16::from_be_bytes(buf.try_into()?);
        let mut offset = 2;
        let mut cp = Vec::with_capacity(len as usize);
        for i in 1..len {
            let (cpe, bytes) = ConstantPoolEntry::from_buffer(&buf[offset..]);
            cp.push(cpe);
            offset += bytes;
        }
        Ok((Self(cp), offset))
    }

    pub fn get_zero_indexed() {

    }
}
