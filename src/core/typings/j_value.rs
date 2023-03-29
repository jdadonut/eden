
pub enum JValue {
    Null, // Null value, used for void return types as well as null references

    // Int types
    Byte(i8), // fully stack allocated
    UByte(u8), // fully stack allocated
    Short(i16), // fully stack allocated
    UShort(u16), // fully stack allocated
    Int(i32), // fully stack allocated 
    UInt(u32), // fully stack allocated
    Long(i64), // fully stack allocated
    ULong(u64), // fully stack allocated

    // Float types
    Float(f32), // fully stack allocated
    Double(f64), // fully stack allocated

    // big int types
    BigInt(i128), // fully stack allocated
    UBigInt(u128), // fully stack allocated

    // Reference types
    String(String), // stack allocated pointer to heap allocated string of dynamic size
    Object

}