use super::constant_pool::ConstantPoolEntry;



pub struct RawClass<'a> {
    constant_pool_count: u16,
    constant_pool: Vec<ConstantPoolEntry<'a>>,
    
    
}