use std::error::Error;
use std::fmt::Debug;
use crate::jvm::reader::constant_pool::ConstantPoolEntryTag::Package;

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
        Ok((cp, offset))
    }

    pub fn get_zero_indexed() {

    }
}

pub struct ConstantPoolEntry {
    tag: u8,
    info: ConstantPoolInfo,
}

impl ConstantPoolEntry {
    pub(super) fn from_buffer(buf: &[u8]) -> (Self, usize) {
        let info = ConstantPoolInfo::from_buffer(buf);
    }
}

#[repr(u8)]
pub enum ConstantPoolEntryTag {
    Class = 7,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    String = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

union ConstantPoolInfo {
    class: ConstantClassInfo,
    fieldref: ConstantFieldrefInfo,
    methodref: ConstantMethodrefInfo,
    interface_methodref: ConstantInterfaceMethodrefInfo,
    string: ConstantStringInfo,
    integer: ConstantIntegerInfo,
    float: ConstantFloatInfo,
    long: ConstantLongInfo,
    double: ConstantDoubleInfo,
    name_and_type: ConstantNameAndTypeInfo,
    utf8: ConstantUtf8Info,
    method_handle: ConstantMethodHandleInfo,    
    method_type: ConstantMethodTypeInfo,
    dynamic: ConstantDynamicInfo,
    invoke_dynamic: ConstantInvokeDynamicInfo,
    module: ConstantModuleInfo,
    package: ConstantPackageInfo,
}
#[derive(Debug)]
struct ConstantClassInfo {
    name_index: u16,
}
#[derive(Debug)]
struct ConstantFieldrefInfo {
    class_index: u16,
    name_and_type_index: u16,
}
#[derive(Debug)]
struct ConstantMethodrefInfo {
    class_index: u16,
    name_and_type_index: u16,
}
#[derive(Debug)]
struct ConstantInterfaceMethodrefInfo {
    class_index: u16,
    name_and_type_index: u16,
}
#[derive(Debug)]
struct ConstantStringInfo {
    string_index: u16,
}
#[derive(Debug)]
struct ConstantIntegerInfo {
    bytes: u32,
}
#[derive(Debug)]
struct ConstantFloatInfo {
    bytes: u32,
}
#[derive(Debug)]
struct ConstantLongInfo {
    high_bytes: u32,
    low_bytes: u32,
}
#[derive(Debug)]
struct ConstantDoubleInfo {
    high_bytes: u32,
    low_bytes: u32,
}
#[derive(Debug)]
struct ConstantNameAndTypeInfo {
    name_index: u16,
    descriptor_index: u16,
}
#[derive(Debug)]
struct ConstantUtf8Info {
    string: Box<String>,
}

impl ConstantUtf8Info {
    pub fn read_from(buf: &[u8]) -> Result<Self, Box<dyn Error>> {
        let len = u16::from_be_bytes(buf.try_into()?);
        let bytes: &[u8] = buf[2..len+2];
        Ok(Self {
            string: box String::from_utf8(Vec::from(bytes))?,
        })
    }

    pub fn take(self) -> String {
        *self.string
    }
    pub fn as_str(&self) -> &str {
        &self.string
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }
}

#[derive(Clone, Copy, Debug)]
struct ConstantMethodHandleInfo {
    reference_kind: u8,
    reference_index: u16,
}
#[derive(Clone, Copy, Debug)]
struct ConstantMethodTypeInfo {
    descriptor_index: u16,
}
#[derive(Clone, Copy, Debug)]
struct ConstantDynamicInfo {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16,
}
#[derive(Clone, Copy, Debug)]
struct ConstantInvokeDynamicInfo {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16,
}
#[derive(Clone, Copy, Debug)]
struct ConstantModuleInfo {
    name_index: u16,
}
#[derive(Clone, Copy, Debug)]
struct ConstantPackageInfo {
    name_index: u16,
}


impl Debug for ConstantPoolEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("ConstantPoolEntry");
        ds.field("tag", &self.tag);
        match self.tag {
            7 => ds.field("info", unsafe { &self.info.class }),
            9 => ds.field("info", unsafe { &self.info.fieldref }),
            10 => ds.field("info", unsafe { &self.info.methodref }),
            11 => ds.field("info", unsafe { &self.info.interface_methodref }),
            8 => ds.field("info", unsafe { &self.info.string }),
            3 => ds.field("info", unsafe { &self.info.integer }),
            4 => ds.field("info", unsafe { &self.info.float }),
            5 => ds.field("info", unsafe { &self.info.long }),
            6 => ds.field("info", unsafe { &self.info.double }),
            12 => ds.field("info", unsafe { &self.info.name_and_type }),
            1 => ds.field("info", unsafe { &self.info.utf8 }),
            15 => ds.field("info", unsafe { &self.info.method_handle }),
            16 => ds.field("info", unsafe { &self.info.method_type }),
            17 => ds.field("info", unsafe { &self.info.dynamic }),
            18 => ds.field("info", unsafe { &self.info.invoke_dynamic }),
            19 => ds.field("info", unsafe { &self.info.module }),
            20 => ds.field("info", unsafe { &self.info.package }),
            _ => panic!("Invalid tag: {}", self.tag),

        };
        ds.finish()
    }
}