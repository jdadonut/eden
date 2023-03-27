use log::warn;

use crate::io::{BufferReadable, Prebuffer};

use super::{raw_class::RawClass};


pub struct ClassFile {
    reader: Box<dyn BufferReadable>,
    path: String,
    classpath: String,
    metadata: ClassFileMetadata,
    class: RawClass,
}
pub struct ClassFileMetadata {
    magic: u32,
    minor_version: u16,
    major_version: u16,
}

impl ClassFile {
    pub fn open_from(path: &str) -> Option<Self> {
        match Prebuffer::load_file(path) {
            Ok(reader) => Self::new(box reader),
            Err(_) => None
        } 
    }
    pub fn new(mut reader: Box<dyn BufferReadable>) -> Option<Self> {
        let metadata = match ClassFileMetadata::new(&mut reader) {
            Some(metadata) => metadata,
            None => return None,
        };
        let class = match RawClass::load(&mut reader) {
            Ok(class) => class,
            Err(_) => return None,
        };
        Some(Self {
            reader,
            path: String::new(),
            classpath: String::new(),
            metadata,
            class,
        })
    }
}

impl ClassFileMetadata {
    pub fn new(reader: &mut Box<dyn BufferReadable>) -> Option<Self> {
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
