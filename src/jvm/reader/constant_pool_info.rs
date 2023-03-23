use std::{error::Error, clone};
use std::fmt::Debug;
use log::{error, warn};


use super::FileReadUtility;
use super::method::MethodHandleKind;
#[derive(Debug, Clone)]
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
        Ok((ConstantPool(cp), offset))
    }

    pub fn from_fileish(file: &mut FileReadUtility) -> Result<Self, Box<dyn Error>> {
        let count_entries = file.read_u2().expect("failed to read constant pool count");
        
        let offset = file.pos();
        let mut cp: Vec<ConstantPoolEntry> = Vec::with_capacity(count_entries as usize);
        
        todo!("finish this")

    }

    pub fn get_zero_indexed() {
        unimplemented!("what are we doing with this @kwzuu")
    }

    pub fn get_java_aligned(&self, index: usize) -> Option<&ConstantPoolEntry> {
        self.0.get(index-1)
    }

    pub fn verify(&self) -> bool {
        let mut test_case = self.clone();
        for ele in test_case.0 {
            if ele.tag != 4 && ele.tag != 5 {
                return true;
            }
        }
        return false;
    }
}
#[derive(Debug, Clone)]
pub struct ConstantPoolEntry {
    tag: u8,
    info: ConstantPoolInfo,
}

impl ConstantPoolEntry {
    pub(super) fn from_buffer(buf: &[u8]) -> (Self, usize) {
        let tag = buf[0];
        if let Some(nfo) = ConstantPoolInfo::from_ordinal(tag, &buf[1..]) {
            let size = nfo.size_of();
            (Self { tag, info: nfo }, size )
        } else {
            error!("Something went wrong while reading constant pool entry. Previous logs should have more information.\n\tTag: {}\n\tBuffer: {:?}", tag, buf);
            (Self { tag, info: ConstantPoolInfo::__unused_ord_0 }, 0)
        }
        

    }
}

#[derive(Debug, Clone)]
pub enum ConstantPoolInfo {
    #[allow(dead_code, non_camel_case_types)]
    __unused_ord_0,
    Utf8(String), // 1
    #[allow(dead_code, non_camel_case_types)]
    __unused_ord_2,
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
    #[allow(dead_code, non_camel_case_types)]
    __unused_ord_13,
    #[allow(dead_code, non_camel_case_types)]
    __unused_ord_14,
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

impl ConstantPoolInfo {

    /// this is a bit of a hack, but it works :3
    pub fn to_ordinal(&self) -> u8 {
        return unsafe {*(self as *const Self as *const u8)};
    }
    
    /// read constant pool info from a buffer given the tag (treated as ordinal into the enum)
    pub fn from_ordinal(ordinal: u8, data: &[u8]) -> Option<Self> {

        match ordinal {
            1 => {
                let len = u16::from_be_bytes(data[0..2].try_into().unwrap());
                let string = String::from_utf8(data[2..2 + len as usize].to_vec()).unwrap();
                Some(Self::Utf8(string))
            }
            3 => {
                let int = i32::from_be_bytes(data[0..4].try_into().unwrap());
                Some(Self::Integer(int))
            }
            4 => {
                let float = f32::from_be_bytes(data[0..4].try_into().unwrap());
                Some(Self::Float(float))
            }
            5 => {
                let long = i64::from_be_bytes(data[0..8].try_into().unwrap());
                Some(Self::Long(long))
            }
            6 => {
                let double = f64::from_be_bytes(data[0..8].try_into().unwrap());
                Some(Self::Double(double))
            }
            7 => {
                let class_ref = u16::from_be_bytes(data[0..2].try_into().unwrap());
                Some(Self::ClassRef(class_ref))
            }
            8 => {
                let string_ref = u16::from_be_bytes(data[0..2].try_into().unwrap());
                Some(Self::StringRef(string_ref))
            }
            9 => {
                let class_ref = u16::from_be_bytes(data[0..2].try_into().unwrap());
                let name_and_type = u16::from_be_bytes(data[2..4].try_into().unwrap());
                Some(Self::FieldRef {
                    class: class_ref,
                    name_and_type,
                })
            }
            10 => {
                let class_ref = u16::from_be_bytes(data[0..2].try_into().unwrap());
                let name_and_type = u16::from_be_bytes(data[2..4].try_into().unwrap());
                Some(Self::MethodRef {
                    class: class_ref,
                    name_and_type,
                })
            }
            11 => {
                let class_ref = u16::from_be_bytes(data[0..2].try_into().unwrap());
                let name_and_type = u16::from_be_bytes(data[2..4].try_into().unwrap());
                Some(Self::InterfaceMethodRef {
                    class: class_ref,
                    name_and_type,
                })
            }
            12 => {
                let name = u16::from_be_bytes(data[0..2].try_into().unwrap());
                let descriptor = u16::from_be_bytes(data[2..4].try_into().unwrap());
                Some(Self::NameAndType(name, descriptor))
            }
            15 => {
                let reference_kind = data[0];
                let reference_index = u16::from_be_bytes(data[1..3].try_into().unwrap());
                Some(Self::MethodHandle {
                    kind: MethodHandleKind::from_ordinal(reference_kind).expect(&format!("Invalid MethodHandleKind ordinal in constant pool {}", reference_kind)),
                    index: reference_index,
                })
            }
            16 => {
                let descriptor_index = u16::from_be_bytes(data[0..2].try_into().unwrap());
                Some(Self::MethodType(descriptor_index))
            }
            17 => {
                let bootstrap_method_attr_index = u16::from_be_bytes(data[0..2].try_into().unwrap());
                let name_and_type_index = u16::from_be_bytes(data[2..4].try_into().unwrap());
                Some(Self::Dynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                })
            }
            18 => {
                let bootstrap_method_attr_index = u16::from_be_bytes(data[0..2].try_into().unwrap());
                let name_and_type_index = u16::from_be_bytes(data[2..4].try_into().unwrap());
                Some(Self::InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                })
            }
            19 => {
                let module_index = u16::from_be_bytes(data[0..2].try_into().unwrap());
                Some(Self::Module(module_index))
            }
            20 => {
                let package_index = u16::from_be_bytes(data[0..2].try_into().unwrap());
                Some(Self::Package(package_index))
            }
            _ => None
        }

    }
    
    /// Serializes the constant pool info into a vector of bytes that *should* be the same as the original data
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = vec![self.to_ordinal()];
        match self {
            Self::Utf8(string) => {
                let len = string.len() as u16;
                data.extend_from_slice(&len.to_be_bytes());
                data.extend_from_slice(string.as_bytes());
            }
            Self::Integer(int) => {
                data.extend_from_slice(&int.to_be_bytes());
            }
            Self::Float(float) => {
                data.extend_from_slice(&float.to_be_bytes());
            }
            Self::Long(long) => {
                data.extend_from_slice(&long.to_be_bytes());
            }
            Self::Double(double) => {
                data.extend_from_slice(&double.to_be_bytes());
            }
            Self::ClassRef(class_ref) => {
                data.extend_from_slice(&class_ref.to_be_bytes());
            }
            Self::StringRef(string_ref) => {
                data.extend_from_slice(&string_ref.to_be_bytes());
            }
            Self::FieldRef {
                class,
                name_and_type,
            } => {
                data.extend_from_slice(&class.to_be_bytes());
                data.extend_from_slice(&name_and_type.to_be_bytes());
            }
            Self::MethodRef {
                class,
                name_and_type,
            } => {
                data.extend_from_slice(&class.to_be_bytes());
                data.extend_from_slice(&name_and_type.to_be_bytes());
            }
            Self::InterfaceMethodRef {
                class,
                name_and_type,
            } => {
                data.extend_from_slice(&class.to_be_bytes());
                data.extend_from_slice(&name_and_type.to_be_bytes());
            }
            Self::NameAndType(name, descriptor) => {
                data.extend_from_slice(&name.to_be_bytes());
                data.extend_from_slice(&descriptor.to_be_bytes());
            }
            Self::MethodHandle{kind, index} => {
                data.push(kind.to_ordinal());
                data.extend_from_slice(&index.to_be_bytes());
            }
            Self::MethodType(descriptor_index) => {
                data.extend_from_slice(&descriptor_index.to_be_bytes());
            }
            Self::Dynamic{bootstrap_method_attr_index, name_and_type_index} => {
                data.extend_from_slice(&bootstrap_method_attr_index.to_be_bytes());
                data.extend_from_slice(&name_and_type_index.to_be_bytes());
            }
            Self::InvokeDynamic{bootstrap_method_attr_index, name_and_type_index} => {
                data.extend_from_slice(&bootstrap_method_attr_index.to_be_bytes());
                data.extend_from_slice(&name_and_type_index.to_be_bytes());
            }
            Self::Module(module_index) => {
                data.extend_from_slice(&module_index.to_be_bytes());
            }
            Self::Package(package_index) => {
                data.extend_from_slice(&package_index.to_be_bytes());
            }
            _ => {
                warn!("Attempted serialization of unsupported constant pool entry type {:?}.", self);
                return vec![];
            }
        }
        return data;
    }

    /// Returns the size of the constant pool info in bytes. Includes the tag byte.
    pub fn size_of(&self) -> usize {
        match self {
            Self::Utf8(string) => 3 + string.len(),
            Self::Integer(_) => 5,
            Self::Float(_) => 5,
            Self::Long(_) => 9,
            Self::Double(_) => 9,
            Self::ClassRef(_) => 3,
            Self::StringRef(_) => 3,
            Self::FieldRef {
                class: _,
                name_and_type: _,
            } => 5,
            Self::MethodRef {
                class: _,
                name_and_type: _,
            } => 5,
            Self::InterfaceMethodRef {
                class: _,
                name_and_type: _,
            } => 5,
            Self::NameAndType(_, _) => 5,
            Self::MethodHandle{kind: _, index: _} => 4,
            Self::MethodType(_) => 3,
            Self::Dynamic{bootstrap_method_attr_index: _, name_and_type_index: _} => 5,
            Self::InvokeDynamic{bootstrap_method_attr_index: _, name_and_type_index: _} => 5,
            Self::Module(_) => 3,
            Self::Package(_) => 3,
            _ => {
                warn!("Attempted to get size of unsupported constant pool entry type {:?}.", self);
                return 0;
            }
        }

    }

}