use crate::{io::BufferReadable, util::code_err::ClassParseError};

use super::{
    constant_pool_info::{ConstantPool}, interface::Interfaces, field::Fields, method::Methods, attribute::Attributes,
    //  Fileish, FileReadUtility
    };



pub struct RawClass {
    access_flags: u16,
    this_class: u16,
    super_class: u16,

    cp: ConstantPool,
    interfaces: Interfaces,
    fields: Fields,
    methods: Methods,
    attributes: Attributes,

}

impl RawClass {
    pub fn load(buf: &mut Box<dyn BufferReadable>) -> Result<Self, ClassParseError> {
        
        let cp = ConstantPool::load(buf)?;

        let access_flags = buf.read_u2()?;
        let this_class = buf.read_u2()?;
        let super_class = buf.read_u2()?;

        let interfaces = Interfaces::load(buf)?;
        let fields = Fields::load(buf)?;
        let methods = Methods::load(buf)?;
        let attributes = Attributes::load(buf)?;

        Ok(Self {
            access_flags,
            this_class,
            super_class,
            cp,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}
