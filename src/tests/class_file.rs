use class_file::pos_slice::PoSlice;
use class_file::constant_pool::{ConstantClass, ConstantDouble, ConstantMethodref, ConstantInvokeDynamic};

#[test]
fn pos_slice_works() {
    let bytes = [4, 6, 2, 7, 1, 2, 9, 6, 8, 0];
    let poslice = PoSlice::new(&bytes);
    assert_eq!(poslice.read_u1().unwrap(), 4);
    assert_eq!(poslice.peek_u2().unwrap(), 1538);
    assert_eq!(poslice.read_u2().unwrap(), 1538);
    assert_eq!(poslice.peek_u2().unwrap(), 1793);
    assert_eq!(poslice.read_u4().unwrap(), 117_506_569);
    assert_eq!(poslice.read_slice(2).unwrap(), &[6, 8]);
    assert_eq!(poslice.read_u1().unwrap(), 0);
}

use std::io::Read;
use std::fs::File;

use class_file::class_file::ClassFile;
use class_file::from_bytes::FromBytes;

#[test]
fn class_file_works() {
    let mut file = File::open("/home/zer0/Projects/jvm_rs/src/tests/source_files/Test.class")
        .unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    let len = bytes.len();
    println!("len: {}", len);
    let slice = PoSlice::new(&bytes);
    let class_file = ClassFile::from_bytes(&slice).unwrap();
    assert_eq!(len, slice.pos());
    let _main = class_file.get_main_method().unwrap();
}

#[test]
fn constant_pool_works() {
    let bytes = [0u8; 30];
    let pos_slice = PoSlice::new(&bytes);
    let _ = ConstantClass::read(&pos_slice).unwrap();
    assert_eq!(pos_slice.pos(), 2);
    let _ = ConstantDouble::read(&pos_slice).unwrap();
    assert_eq!(pos_slice.pos(), 10);
    let _ = ConstantMethodref::read(&pos_slice).unwrap();
    assert_eq!(pos_slice.pos(), 14);
    let _ = ConstantInvokeDynamic::read(&pos_slice).unwrap();
    assert_eq!(pos_slice.pos(), 18);
}