use super::{constant_pool_info::{ConstantPool}, Fileish, FileReadUtility};



pub struct RawClass {
    cp: ConstantPool
}

impl RawClass {
    pub fn read_from(input: &mut FileReadUtility) -> Option<Self> {
        let cp = ConstantPool::from_fileish(input);
        None
    }
}
