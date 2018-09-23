use std::rc::Rc;

use class_file::try_from::TryFrom;
use types::{u1, u2, u4};
use result::{Result, Error};

#[derive(Clone)]
pub struct ConstantPool<'a>(pub(super) Rc<Vec<Option<CPInfo<'a>>>>);

impl<'a> ConstantPool<'a> {
    pub fn get(&self, index: usize) -> Result<CPInfo<'a>> { 
        if let Some(cp_info) = self.0.get(index) {
            cp_info.ok_or(Error::EmptyCPInfo(index as u2))
        } else {
            Err(Error::OutOfBounds(index))
        }
    }
}

pub enum CPItem {
    Fieldref(FieldrefInfo),
    Methodref(MethodrefInfo),
    InterfaceMethodref(InterfaceMethodrefInfo),
    String(StringInfo),
}

#[derive(Debug, Copy, Clone)]
pub struct CPInfo<'a> {
    pub(super) tag: Tag,
    pub(super) info: &'a [u1],
}

impl<'a> CPInfo<'a> {
    #[inline]
    pub fn into<T>(self) -> Result<T> 
    where T: TryFrom<CPInfo<'a>> {
        T::try_from(self)
    }
}

#[allow(non_camel_case_types)]
#[derive(Primitive, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Tag {
    INTEGER =  3,
    FLOAT =  4,
    LONG =  5,
    DOUBLE =  6,
    UTF8 =  1,
    STRING =  8,
    CLASS =  7,
    FIELD_REF =  9,
    METHOD_REF =  10,
    INTERFACE_METHOD_REF =  11,
    NAME_AND_TYPE =  12,
    METHOD_HANDLE =  15,
    METHOD_TYPE =  16,
    INVOKE_DYNAMIC =  18,
}

pub trait FromCpInfo<'a>
where Self: Sized {
    fn from_cp_info(cp_info: CPInfo<'a>) -> Result<Self>;
}

#[derive(Copy, Clone)] 
pub struct IntegerInfo {
    bytes: u4,
}

#[derive(Copy, Clone)] 
pub struct FloatInfo {
    bytes: u4,
}   

#[derive(Copy, Clone)] 
pub struct LongInfo {
    high_bytes: u4,
    low_bytes: u4,
}

#[derive(Copy, Clone)] 
pub struct DoubleInfo {
    high_bytes: u4,
    low_bytes: u4,
}

#[derive(Copy, Clone)] 
pub struct Utf8Info<'a> {
    pub(super) length: u2,
    pub(super) bytes: &'a [u1],
}

impl<'a> Utf8Info<'a> {
    pub fn get_string(&self) -> Result<String> {
        String::from_utf8(self.bytes.to_vec())
            .map_err(|_| Error::InvalidUtf8)
    }
}

#[derive(Copy, Clone)] 
pub struct StringInfo {
    string_index: u2,
}

#[derive(Copy, Clone)] 
pub struct ClassInfo {
    pub(super) name_index: u2,
}

#[derive(Copy, Clone)] 
pub struct FieldrefInfo {
    class_index: u2,
    name_and_type_index: u2,
}

#[derive(Copy, Clone)] 
pub struct MethodrefInfo {
    class_index: u2,
    name_and_type_index: u2,
}

#[derive(Copy, Clone)] 
pub struct InterfaceMethodrefInfo {
    class_index: u2,
    name_and_type_index: u2,
}

#[derive(Copy, Clone)] 
pub struct NameAndTypeInfo {
    tag: Tag,
    class_index: u2,
    descriptor_index: u2,
}

#[derive(Copy, Clone)] 
pub struct MethodHandleInfo {
    tag: Tag,
    reference_kind: u1,
    reference_index: u2,
}

#[derive(Copy, Clone)] 
pub struct MethodTypeInfo {
    tag: Tag,
    descriptor_index: u2,
}

#[derive(Copy, Clone)] 
pub struct InvokeDynamicInfo {
    tag: Tag,
    bootstrap_method_attr_index: u2,
    name_and_type_index: u2,
}