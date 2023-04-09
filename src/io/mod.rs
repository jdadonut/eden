mod prebuffer;

pub use prebuffer::Prebuffer;

use std::io::{Read, Seek, SeekFrom};

use crate::util::code_err::ClassParseError;



pub trait BufferReadable : Read + Seek {
    
    fn read_byte(&mut self) -> Result<u8, ClassParseError> {
        let mut buffer = [0; 1];
        if let Err(io) = self.read(&mut buffer) {
            return Err(ClassParseError::IOError(io));
        }
        Ok(buffer[0])
    }
    fn read_u2(&mut self) -> Result<u16, ClassParseError> {
        Ok((self.read_byte()? as u16) << 8 | self.read_byte()? as u16)
    }
    fn read_u4(&mut self) -> Result<u32, ClassParseError> {
        Ok((self.read_u2()? as u32) << 16 | self.read_u2()? as u32)
    }
    fn read_u8(&mut self) -> Result<u64, ClassParseError> {
        Ok((self.read_u4()? as u64) << 32 | self.read_u4()? as u64)
    }
    fn read_string(&mut self) -> Result<String, ClassParseError> {
        let size = self.read_u2()?;
        let mut buffer = vec![0; size as usize];
        match self.read(&mut buffer) {
            Ok(_) => Ok(String::from_utf8(buffer).unwrap()),
            Err(_) => Err(ClassParseError::EarlyEOF("EOF@".to_string())),
        }
    }

    fn peek_next(&mut self) -> Result<u8, ClassParseError> {
        let mut buffer = [0; 1];
        match self.read(&mut buffer) {
            Ok(_) => {
                self.seek(std::io::SeekFrom::Current(-1)).unwrap();
                Ok(buffer[0])
            },
            Err(_) => Err(ClassParseError::EarlyEOF("EOF@".to_string())),
        }
    }
    fn peek_u1(&mut self, offset: u64) -> Result<u8, ClassParseError> {
        let pos = self.seek(std::io::SeekFrom::Current(0)).unwrap();
        self.seek(SeekFrom::Start(offset)).unwrap();
        let ret = self.read_byte();
        self.seek(SeekFrom::Start(pos)).unwrap();
        ret
    }
    fn peek_u2(&mut self, offset: u64) -> Result<u16, ClassParseError> {
        let pos = self.seek(std::io::SeekFrom::Current(0)).unwrap();
        self.seek(SeekFrom::Start(offset)).unwrap();
        let ret = self.read_u2();
        self.seek(SeekFrom::Start(pos)).unwrap();
        ret
    }
    fn peek_u4(&mut self, offset: u64) -> Result<u32, ClassParseError> {
        let pos = self.seek(std::io::SeekFrom::Current(0)).unwrap();
        self.seek(SeekFrom::Start(offset)).unwrap();
        let ret = self.read_u4();
        self.seek(SeekFrom::Start(pos)).unwrap();
        ret
    }
    fn peek_u8(&mut self, offset: u64) -> Result<u64, ClassParseError> {
        let pos = self.seek(std::io::SeekFrom::Current(0)).unwrap();
        self.seek(SeekFrom::Start(offset)).unwrap();
        let ret = self.read_u8();
        self.seek(SeekFrom::Start(pos)).unwrap();
        ret
    }
    fn peek_string(&mut self, offset: u64) -> Result<String, ClassParseError> {
        let pos = self.seek(std::io::SeekFrom::Current(0)).unwrap();
        self.seek(SeekFrom::Start(offset)).unwrap();
        let ret = self.read_string();
        self.seek(SeekFrom::Start(pos)).unwrap();
        ret
    }
    
    fn skip(&mut self, offset: u64) -> Result<(), ClassParseError> {
        match self.seek(SeekFrom::Current(offset as i64)) {
            Ok(_) => Ok(()),
            Err(_) => Err(ClassParseError::EarlyEOF("EOF@skip".to_string())),
        }
    }
    fn seek_to(&mut self, offset: u64) -> Result<(), ClassParseError> {
        match self.seek(SeekFrom::Start(offset)) {
            Ok(_) => Ok(()),
            Err(_) => Err(ClassParseError::EarlyEOF("EOF@seek_to".to_string())),
        }
    }

}

auto trait BlanketBufferReadableImpl {}

impl<T: Read + Seek + BlanketBufferReadableImpl> BufferReadable for T {}
