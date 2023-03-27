use std::io::{self, Read, Seek, SeekFrom};

use crate::util::code_err::ClassParseError;

use super::{BufferReadable, BlanketBufferReadableImpl};


/// Prebuffer is a wrapper around a [u8] that
/// imlements Read and Seek without copying any data.
/// Any data that this struct offers up is a slice
/// of the original data.
/// 
/// # Memory Safety & Data Integrity
/// To assure that the backing data is not modified,
/// it is requested that before modifying any data 
/// that this struct returns, the data is copied.
/// If this is not done, the data may be modified
/// and can be corrupted.
/// This can not be fixed due to the fact that
/// doing anything else would require copying the
/// data.
pub struct Prebuffer {
    data: Box<[u8]>,
    position: usize,
}
impl Prebuffer {
    pub fn new(data: Box<[u8]>) -> Self {
        Self {
            data,
            position: 0,
        }
    }
    pub fn load_file(path: &str) -> Result<Self, ClassParseError> {
        let mut file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(
                ClassParseError::IOError {
                    internal: err,
                    classpath: None,
                    path: Some(path.to_string()),
                }
            ),
        };
        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(_) => Ok(Self::new(data.into_boxed_slice())),
            Err(err) => Err(ClassParseError::IOError { internal: err, classpath: None, path: Some(path.to_string())}),
        }
    }
    pub fn read_n_bytes(&mut self, n: usize) -> Result<&[u8], ClassParseError> {
        if self.position + n > self.data.len() {
            return Err(ClassParseError::EarlyEOF(format!("EOF in Prebuffer, reading {} bytes when we only have {} left.", n, self.data.len() - self.position).to_string()));
        }
        let start = self.position;
        self.position += n;
        Ok(&self.data[start..self.position])
    }
}

impl Read for Prebuffer {
    /// Reads up to `buf.len()` bytes from the underlying data. THIS COPIES!
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let n = buf.len();
        if self.position + n > self.data.len() {
            return Err(io::Error::new(io::ErrorKind::Other, "Read position is out of bounds."));
        }
        let start = self.position;
        self.position += n;
        buf.copy_from_slice(&self.data[start..self.position]);
        Ok(n)
    }
}
impl Seek for Prebuffer {
    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64, std::io::Error> {
        match pos {
            SeekFrom::Start(n) => {
                if n > self.data.len() as u64 {
                    return Err(io::Error::new(io::ErrorKind::Other, "Seek position is out of bounds."));
                }
                self.position = n as usize;
            },
            SeekFrom::Current(n) => {
                if self.position as i64 + n > self.data.len() as i64 {
                    return Err(io::Error::new(io::ErrorKind::Other, "Seek position is out of bounds."));
                }
                self.position = (self.position as i64 + n) as usize;
            },
            SeekFrom::End(n) => {
                if self.data.len() as i64 + n > self.data.len() as i64 {
                    return Err(io::Error::new(io::ErrorKind::Other, "Seek position is out of bounds."));
                }
                self.position = (self.data.len() as i64 + n) as usize;
            },
        }
        Ok(self.position as u64)
    }
}
impl !BlanketBufferReadableImpl for Prebuffer {}
impl BufferReadable for Prebuffer {
    // Manual implementation that uses read_n_bytes instead of Read::read
    // read_n_bytes does not copy any data, as opposed to Read::read

    fn read_byte(&mut self) -> Result<u8, ClassParseError> {
        match self.read_n_bytes(1) {
            Ok(data) => {
                Ok(data[0])
            },
            Err(e) => Err(e),
        }
    }
    fn read_u2(&mut self) -> Result<u16, ClassParseError> {
        match self.read_n_bytes(2) {
            Ok(data) => {
                Ok(u16::from_be_bytes([data[0], data[1]]))
            },
            Err(e) => Err(e),
        }
    }
    fn read_u4(&mut self) -> Result<u32, ClassParseError> {
        match self.read_n_bytes(4) {
            Ok(data) => {
                Ok(u32::from_be_bytes([data[0], data[1], data[2], data[3]]))
            },
            Err(e) => Err(e),
        }
    }
    fn read_u8(&mut self) -> Result<u64, ClassParseError> {
        match self.read_n_bytes(8) {
            Ok(data) => {
                Ok(u64::from_be_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]))
            },
            Err(e) => Err(e),
        }
    }
    fn read_string(&mut self) -> Result<String, ClassParseError> {
        let len = self.read_u2()?;
        let data = self.read_n_bytes(len as usize)?;
        match String::from_utf8(data.to_vec()) {
            Ok(s) => Ok(s),
            Err(e) => Err(
                ClassParseError::StringDecodeError { 
                    internal: e,
                    buffer: data.to_vec(),
                 }
            ),
        }
    }

    fn peek_next(&mut self) -> Result<u8, ClassParseError> {
        let pos = self.position;
        let byte = self.read_byte()?;
        self.position = pos;
        Ok(byte)
    }
    fn peek_u1(&mut self, offset: u64) -> Result<u8, ClassParseError> {
        let pos = self.position;
        match self.seek(SeekFrom::Current(offset as i64)) {
            Ok(_) => {
                let byte = self.read_byte()?;
                self.position = pos;
                Ok(byte)
            },
            Err(e) => return Err(ClassParseError::IOError { internal: e, classpath: None, path: None }),
        }
    }
    fn peek_u2(&mut self, offset: u64) -> Result<u16, ClassParseError> {
        let pos = self.position;
        match self.seek(SeekFrom::Current(offset as i64)) {
            Ok(_) => {
                let byte = self.read_u2()?;
                self.position = pos;
                Ok(byte)
            },
            Err(e) => return Err(ClassParseError::IOError { internal: e, classpath: None, path: None }),
        }
    }
    fn peek_u4(&mut self, offset: u64) -> Result<u32, ClassParseError> {
        let pos = self.position;
        match self.seek(SeekFrom::Current(offset as i64)) {
            Ok(_) => {
                let byte = self.read_u4()?;
                self.position = pos;
                Ok(byte)
            },
            Err(e) => return Err(ClassParseError::IOError { internal: e, classpath: None, path: None }),
        }
    }
    fn peek_u8(&mut self, offset: u64) -> Result<u64, ClassParseError> {
        let pos = self.position;
        match self.seek(SeekFrom::Current(offset as i64)) {
            Ok(_) => {
                let byte = self.read_u8()?;
                self.position = pos;
                Ok(byte)
            },
            Err(e) => return Err(ClassParseError::IOError { internal: e, classpath: None, path: None }),
        }  
    }
    fn peek_string(&mut self, offset: u64) -> Result<String, ClassParseError> {
        let pos = self.position;
        match self.seek(SeekFrom::Current(offset as i64)) {
            Ok(_) => {
                let byte = self.read_string()?;
                self.position = pos;
                Ok(byte)
            },
            Err(e) => return Err(ClassParseError::IOError { internal: e, classpath: None, path: None }),
        }  
    }

    fn skip(&mut self, offset: u64) -> Result<(), ClassParseError> {
        match self.seek(SeekFrom::Current(offset as i64)) {
            Ok(_) => Ok(()),
            Err(e) => Err(ClassParseError::IOError { internal: e, classpath: None, path: None }),
        }
    }
    fn seek_to(&mut self, offset: u64) -> Result<(), ClassParseError> {
        match self.seek(SeekFrom::Start(offset)) {
            Ok(_) => Ok(()),
            Err(e) => Err(ClassParseError::IOError { internal: e, classpath: None, path: None }),
        }
    }
}
