use std::fs::File;
use std::io::Write;

use crate::jvm::reader::classfile::ClassFile;
use crate::jvm::reader::raw_class::RawClass;


#[test]
pub fn load_HelloWorld_class() {
    
    let class = ClassFile::new(&mut File::open("java_tests/HelloWorld.java").unwrap());

    let mut debug_file = File::create("java_tests/HelloWorld.class.deserialized").unwrap();
    write!(debug_file, "{:#?}", class).unwrap();
}