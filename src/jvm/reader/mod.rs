use std::{fs::File, io::{Read, Seek}};

pub mod classfile;
pub mod raw_class;
pub mod constant_pool;
pub mod access_flags;



trait Fileish: Read + Seek {}
pub struct FileReadUtility {
    file: Box<dyn Fileish>,
}
impl Fileish for File {}

impl FileReadUtility {
    pub fn new(path: &str) -> Result<Self, ()> {
        
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(()),
        };
        return Ok(Self { file: box (file) });
    }

    pub fn read_byte(&mut self) -> Result<u8, ()> {
        let mut buffer = [0; 1];
        match self.file.read(&mut buffer) {
            Ok(_) => Ok(buffer[0]),
            Err(_) => Err(()),
        }
    }
    pub fn read_u2(&mut self) -> Result<u16, ()> {
        let mut buffer = [0; 2];
        match self.file.read(&mut buffer) {
            Ok(_) => Ok(u16::from_be_bytes(buffer)),
            Err(_) => Err(()),
        }
    }
    pub fn read_u4(&mut self) -> Result<u32, ()> {
        let mut buffer = [0; 4];
        match self.file.read(&mut buffer) {
            Ok(_) => Ok(u32::from_be_bytes(buffer)),
            Err(_) => Err(()),
        }
    }
    pub fn read_u8(&mut self) -> Result<u64, ()> {
        let mut buffer = [0; 8];
        match self.file.read(&mut buffer) {
            Ok(_) => Ok(u64::from_be_bytes(buffer)),
            Err(_) => Err(()),
        }
    }
    pub fn read_string(&mut self, length: usize) -> Result<String, ()> {
        let mut buffer = vec![0; length];
        match self.file.read(&mut buffer) {
            Ok(_) => Ok(String::from_utf8(buffer).unwrap()),
            Err(_) => Err(()),
        }
    }
    pub fn read_string_tonull(&mut self) -> Result<String, ()> {
        let mut buffer = Vec::new();
        while let Ok(byte) = self.read_byte() {
            if byte == 0 {
                break;
            }
            buffer.push(byte);
        }
        Ok(String::from_utf8(buffer).unwrap())
    }
    pub fn read_jstring(&mut self) -> Result<String, ()> {
        todo!("check classfile format for jstring");
        let length = self.read_u2().unwrap() as usize;
        self.read_string(length)
    }

    pub fn read_u1_at(&mut self, offset: u64) -> Result<u8, ()> {
        let pos = self.file.seek(std::io::SeekFrom::Current(0)).unwrap();
        self.file.seek(std::io::SeekFrom::Start(offset)).unwrap();
        let ret = self.read_byte();
        self.file.seek(std::io::SeekFrom::Start(pos)).unwrap();
        ret
    }
    pub fn read_u2_at(&mut self, offset: u64) -> Result<u16, ()> {
        let pos = self.file.seek(std::io::SeekFrom::Current(0)).unwrap();
        self.file.seek(std::io::SeekFrom::Start(offset)).unwrap();
        let ret = self.read_u2();
        self.file.seek(std::io::SeekFrom::Start(pos)).unwrap();
        ret
    }
    pub fn read_u4_at(&mut self, offset: u64) -> Result<u32, ()> {
        let pos = self.file.seek(std::io::SeekFrom::Current(0)).unwrap();
        self.file.seek(std::io::SeekFrom::Start(offset)).unwrap();
        let ret = self.read_u4();
        self.file.seek(std::io::SeekFrom::Start(pos)).unwrap();
        ret
    }
    pub fn read_u8_at(&mut self, offset: u64) -> Result<u64, ()> {
        let pos = self.file.seek(std::io::SeekFrom::Current(0)).unwrap();
        self.file.seek(std::io::SeekFrom::Start(offset)).unwrap();
        let ret = self.read_u8();
        self.file.seek(std::io::SeekFrom::Start(pos)).unwrap();
        ret
    }
    pub fn read_string_at(&mut self, offset: u64, length: usize) -> Result<String, ()> {
        let pos = self.file.seek(std::io::SeekFrom::Current(0)).unwrap();
        self.file.seek(std::io::SeekFrom::Start(offset)).unwrap();
        let ret = self.read_string(length);
        self.file.seek(std::io::SeekFrom::Start(pos)).unwrap();
        ret
    }
    pub fn read_string_tonull_at(&mut self, offset: u64) -> Result<String, ()> {
        let pos = self.file.seek(std::io::SeekFrom::Current(0)).unwrap();
        self.file.seek(std::io::SeekFrom::Start(offset)).unwrap();
        let ret = self.read_string_tonull();
        self.file.seek(std::io::SeekFrom::Start(pos)).unwrap();
        ret
    }
    pub fn read_jstring_at(&mut self, offset: u64) -> Result<String, ()> {
        let pos = self.file.seek(std::io::SeekFrom::Current(0)).unwrap();
        self.file.seek(std::io::SeekFrom::Start(offset)).unwrap();
        let ret = self.read_jstring();
        self.file.seek(std::io::SeekFrom::Start(pos)).unwrap();
        ret
    }
    
    
}