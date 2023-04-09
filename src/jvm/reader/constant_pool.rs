use std::io::Read;
use std::{error::Error};
use std::fmt::Debug;
use log::{error, warn};


use crate::io::BufferReadable;
use crate::util::code_err::ClassParseError;

use super::method_handle_kind::MethodHandleKind;

#[derive(Debug, Clone)]
pub struct ConstantPool(Vec<ConstantPoolEntry>);

impl ConstantPool {
    pub fn load<R: BufferReadable>(buf: &mut R) -> Result<Self, ClassParseError> {
        let count_entries = buf.read_u2().expect("failed to read constant pool count")-1;
        let mut cp: Vec<ConstantPoolEntry> = Vec::with_capacity(count_entries as usize);
        for _ in 1..count_entries {
            cp.push(ConstantPoolEntry::load(buf)?);
        }
        Ok(ConstantPool(cp))

    }
    pub fn get_java_aligned(&self, index: usize) -> Option<&ConstantPoolEntry> {
        self.0.get(index-1)
    }

    pub fn verify(&self) -> bool {
        todo!();
    }
}
#[derive(Debug, Clone)]
pub struct ConstantPoolEntry {
    pub tag: u8,
    pub info: ConstantPoolInfo,
} 

impl ConstantPoolEntry {
    pub fn load<R: BufferReadable>(buf: &mut R) -> Result<Self, ClassParseError> {
        let tag = buf.read_byte()?;
        let info = match tag {
            1 => {
                let length = buf.read_u2()?;
                let mut bytes = vec![0; length as usize];
                buf.read_exact(&mut bytes).map_err(|e| ClassParseError::IOError(e))?;
                ConstantPoolInfo::Utf8(String::from_utf8(bytes).expect("failed to convert bytes to string"))
            },
            3 => {
                let bytes = buf.read_u4()?;
                ConstantPoolInfo::Integer(bytes as i32)
            },
            4 => {
                let bytes = buf.read_u4()?;
                ConstantPoolInfo::Float(f32::from_bits(bytes))
            },
            5 => {
                let bytes = buf.read_u8()?;
                ConstantPoolInfo::Long(bytes as i64)
            },
            6 => {
                let bytes = buf.read_u8()?;
                ConstantPoolInfo::Double(f64::from_bits(bytes))
            },
            7 => {
                let index = buf.read_u2()?;
                ConstantPoolInfo::ClassRef(index)
            },
            8 => {
                let index = buf.read_u2()?;
                ConstantPoolInfo::StringRef(index)
            },
            9 => {
                let class = buf.read_u2()?;
                let name_and_type = buf.read_u2()?;
                ConstantPoolInfo::FieldRef {
                    class,
                    name_and_type,
                }
            },
            10 => {
                let class = buf.read_u2()?;
                let name_and_type = buf.read_u2()?;
                ConstantPoolInfo::MethodRef {
                    class,
                    name_and_type,
                }
            },
            11 => {
                let class = buf.read_u2()?;
                let name_and_type = buf.read_u2()?;
                ConstantPoolInfo::InterfaceMethodRef {
                    class,
                    name_and_type,
                }
            },
            12 => {
                let name_index = buf.read_u2()?;
                let descriptor_index = buf.read_u2()?;
                ConstantPoolInfo::NameAndType(name_index, descriptor_index)
            },
            15 => {
                let kind = MethodHandleKind::from_ordinal(buf.read_byte()?);
                let index = buf.read_u2()?;
                ConstantPoolInfo::MethodHandle {
                    kind: kind.unwrap(),
                    index,
                }
            },
            16 => {
                let index = buf.read_u2()?;
                ConstantPoolInfo::MethodType(index)
            },
            17 => {
                let bootstrap_method_attr_index = buf.read_u2()?;
                let name_and_type_index = buf.read_u2()?;
                ConstantPoolInfo::Dynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                }
            },
            18 => {
                let bootstrap_method_attr_index = buf.read_u2()?;
                let name_and_type_index = buf.read_u2()?;
                ConstantPoolInfo::InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                }
            },
            19 => {
                let index = buf.read_u2()?;
                ConstantPoolInfo::Module(index)
            },
            20 => {
                let index = buf.read_u2()?;
                ConstantPoolInfo::Package(index)
            },
            _ => {
                error!("Unknown constant pool tag: {}", tag);
                return Err(ClassParseError::UnknownConstantPoolTag(tag));
            }
        };
        Ok(ConstantPoolEntry {
            tag,
            info,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ConstantPoolInfo {
    Utf8(String), // 1
    Integer(i32), // 3
    Float(f32), // 4
    Long(i64), // 5
    Double(f64), // 6
    /// <Self>.0 is the index of the class in the constant pool
    ClassRef(u16), // 7
    StringRef(u16), // 8
    FieldRef {
        class: u16,
        name_and_type: u16,
    }, // 9
    MethodRef {
        class: u16,
        name_and_type: u16,
    }, // 10
    InterfaceMethodRef {
        class: u16,
        name_and_type: u16,
    }, // 11
    NameAndType(u16, u16), // 12

    MethodHandle {
        kind: MethodHandleKind,
        index: u16,
    }, // 15
    MethodType(u16), // 16
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    }, // 17
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    }, // 18
    Module(u16), // 19
    Package(u16), // 20
}