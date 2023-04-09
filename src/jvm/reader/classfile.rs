use log::warn;

use crate::{io::{BufferReadable, Prebuffer}, util::code_err::ClassParseError};

use super::{raw_class::RawClass};

#[derive(Debug)]
pub struct ClassFile {
    pub path: String,
    pub classpath: String,
    pub metadata: ClassFileMetadata,
    pub class: RawClass,
}
#[derive(Debug)]
pub struct ClassFileMetadata {
    magic: u32,
    minor_version: u16,
    major_version: u16,
}

impl ClassFile {
    pub fn open_from(path: &str) -> Result<Self, ClassParseError> {
        match Prebuffer::load_file(path) {
            Ok(reader) => Self::new(reader),
            Err(_) => Err(ClassParseError::Silly("cant open file, fix this error message later :3".to_owned())),
        } 
    }
    pub fn new<R: BufferReadable>(mut reader:  R) -> Result<Self, ClassParseError> {
        let metadata = ClassFileMetadata::new(&mut reader)?;
        let class = RawClass::load(&mut reader)?;
        Ok(Self {
            path: String::new(),
            classpath: String::new(),
            metadata,
            class,
        })
    }
}

impl ClassFileMetadata {
    pub fn new<R: BufferReadable>(reader: &mut R) -> Result<Self, ClassParseError> {
        let magic = reader.read_u4()?;
        if magic != 0xCAFEBABE {
            warn!("Invalid magic number for classfile: {}", magic);
            return Err(ClassParseError::Silly(format!("Invalid magic number for classfile: 0x{:x}", magic)));
        }
        let minor_version = reader.read_u2()?;
        let major_version = reader.read_u2()?;
        Ok(Self {
            magic,
            minor_version,
            major_version,
        })
    }
}
