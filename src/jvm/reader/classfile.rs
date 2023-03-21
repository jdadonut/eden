use log::warn;

use super::{raw_class::RawClass, FileReadUtility};


pub struct ClassFile<'a> {
    reader: FileReadUtility,
    path: String,
    classpath: String,
    metadata: ClassFileMetadata,
    class: RawClass<'a>,
}
pub struct ClassFileMetadata {
    magic: u32,
    minor_version: u16,
    major_version: u16,
}

impl <'a> ClassFile<'a> {
    pub fn new(path: &str) -> Option<Self> {
        let mut reader = match FileReadUtility::new(path) {
            Ok(reader) => reader,
            Err(_) => return None,
        };

        let metadata = match ClassFileMetadata::new(&mut reader) {
            Some(metadata) => metadata,
            None => return None,
        };

        None
    }
}

impl ClassFileMetadata {
    pub fn new(reader: &mut FileReadUtility) -> Option<Self> {
        let magic = match reader.read_u4() {
            Ok(magic) => magic,
            Err(_) => return None,
        };
        if magic != 0xCAFEBABE {
            warn!("Invalid magic number for classfile: {}", magic);
            return None;
        }
        let minor_version = match reader.read_u2() {
            Ok(minor_version) => minor_version,
            Err(_) => return None,
        };
        let major_version = match reader.read_u2() {
            Ok(major_version) => major_version,
            Err(_) => return None,
        };
        Some(Self {
            magic,
            minor_version,
            major_version,
        })
    }
}
