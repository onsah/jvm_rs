use num_traits::FromPrimitive;

use class_file::pos_slice::PoSlice;
use types::{u1, u2, u4};
use result::{Result, Error};

/* Working on progress */

pub struct ConstantPoolRep(Vec<Option<CPInfoRep>>);

impl ConstantPoolRep {
    pub fn read<'a>(slice: &'a PoSlice) -> Result<Self> {
        let cp_count = slice.read_u2()? as usize;
        let mut constant_pool = vec![None; cp_count];
        {
            let mut jump = true;
            for cp_info in constant_pool.iter_mut() {
                *cp_info = if !jump {
                    let cp_info = CPInfoRep::new(slice)?;
                    jump = match cp_info {
                        CPInfoRep::Double(_) | CPInfoRep::Long(_) => true,
                        _ => false, 
                    };
                    Some(cp_info)
                } else {
                    jump = false;
                    None
                }
            }
        }
        Ok(ConstantPoolRep(constant_pool))
    }

    pub fn get(&self, index: usize) -> Result<&CPInfoRep> {
        match self.0.get(index) {
            Some(cp_info) => match cp_info {
                Some(cp_info) => Ok(cp_info),
                None => Err(Error::EmptyCPInfo(index as u2)),  
            },
            None => Err(Error::OutOfBounds(index))
        }
    }
}

// replacement of CPInfo
#[derive(Clone)]
pub enum CPInfoRep {
    Class(ConstantClass),
    Fieldref(ConstantFieldref),
    Methodref(ConstantMethodref),
    InterfaceMethodref(ConstantInterfaceMethodref),
    String(ConstantString),
    Integer(ConstantInteger),
    Float(ConstantFloat),
    Long(ConstantLong),
    Double(ConstantDouble),
    NameAndType(ConstantNameAndType),
    Utf8(ConstantUtf8),
    MethodHandle(ConstantMethodHandle),
    MethodType(ConstantMethodType),
    InvokeDynamic(ConstantInvokeDynamic),
}

impl CPInfoRep {
    pub fn new(slice: &PoSlice) -> Result<Self> {
        let tag = slice.read_u1()?;
        Ok(match Tag::new(tag)? {
            Tag::CLASS                  => CPInfoRep::Class(ConstantClass::read(slice)?),
            Tag::FIELD_REF              => CPInfoRep::Fieldref(ConstantFieldref::read(slice)?),
            Tag::METHOD_REF             => CPInfoRep::Methodref(ConstantMethodref::read(slice)?),
            Tag::INTERFACE_METHOD_REF   => CPInfoRep::InterfaceMethodref(ConstantInterfaceMethodref::read(slice)?),
            Tag::STRING                 => CPInfoRep::String(ConstantString::read(slice)?),
            Tag::INTEGER                => CPInfoRep::Integer(ConstantInteger::read(slice)?),
            Tag::FLOAT                  => CPInfoRep::Float(ConstantFloat::read(slice)?),
            Tag::LONG                   => CPInfoRep::Long(ConstantLong::read(slice)?),
            Tag::DOUBLE                 => CPInfoRep::Double(ConstantDouble::read(slice)?),
            Tag::NAME_AND_TYPE          => CPInfoRep::NameAndType(ConstantNameAndType::read(slice)?),
            Tag::UTF8                   => CPInfoRep::Utf8(ConstantUtf8::read(slice)?),
            Tag::METHOD_HANDLE          => CPInfoRep::MethodHandle(ConstantMethodHandle::read(slice)?),
            Tag::METHOD_TYPE            => CPInfoRep::MethodType(ConstantMethodType::read(slice)?),
            Tag::INVOKE_DYNAMIC         => CPInfoRep::InvokeDynamic(ConstantInvokeDynamic::read(slice)?),
        })
    }

    pub fn tag(&self) -> Tag {
        match self {
            CPInfoRep::Class(_) => Tag::CLASS,
            CPInfoRep::Fieldref(_) => Tag::FIELD_REF,
            CPInfoRep::Methodref(_) => Tag::METHOD_REF,
            CPInfoRep::InterfaceMethodref(_) => Tag::INTERFACE_METHOD_REF,
            CPInfoRep::String(_) => Tag::STRING,
            CPInfoRep::Integer(_) => Tag::INTEGER,
            CPInfoRep::Float(_) => Tag::FLOAT,
            CPInfoRep::Long(_) => Tag::LONG,
            CPInfoRep::Double(_) => Tag::DOUBLE,
            CPInfoRep::NameAndType(_) => Tag::NAME_AND_TYPE,
            CPInfoRep::Utf8(_) => Tag::UTF8,
            CPInfoRep::MethodHandle(_) => Tag::METHOD_HANDLE,
            CPInfoRep::MethodType(_) => Tag::METHOD_TYPE,
            CPInfoRep::InvokeDynamic(_) => Tag::INVOKE_DYNAMIC,
        }
    }

    pub fn as_str(&self) -> Result<&str> {
        match self {
            CPInfoRep::Utf8(utf8) => Ok(&utf8.0),
            _ => Err(Error::NotUtf8),
        }
    }
}

trait CPElem {
    fn tag() -> Tag; 
}

#[derive(Clone, Copy)]
pub struct ConstantClass {
    pub(super) name_index: u2,
}

impl CPElem for ConstantClass {
    fn tag() -> Tag { Tag::CLASS }
}

impl ConstantClass {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        let name_index = slice.read_u2()?;
        Ok(ConstantClass {
            name_index,
        })
    }
}

#[derive(Clone)]
pub struct ConstantFieldref {
    class_index: u2,
    name_and_type_index: u2,
}

impl CPElem for ConstantFieldref {
    fn tag() -> Tag { Tag::FIELD_REF }
}

impl ConstantFieldref {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        let class_index = slice.read_u2()?;
        let name_and_type_index = slice.read_u2()?;
        Ok(ConstantFieldref {
            class_index,
            name_and_type_index,
        })
    }
}

#[derive(Clone)]
pub struct ConstantMethodref {
    class_index: u2,
    name_and_type_index: u2,
}

impl CPElem for ConstantMethodref {
    fn tag() -> Tag { Tag::METHOD_REF }
}

impl ConstantMethodref {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        let class_index = slice.read_u2()?;
        let name_and_type_index = slice.read_u2()?;
        Ok(ConstantMethodref {
            class_index,
            name_and_type_index,
        })
    }
}

#[derive(Clone)]
pub struct ConstantInterfaceMethodref {
    class_index: u2,
    name_and_type_index: u2,
}

impl CPElem for ConstantInterfaceMethodref {
    fn tag() -> Tag { Tag::INTERFACE_METHOD_REF }
}

impl ConstantInterfaceMethodref {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        let class_index = slice.read_u2()?;
        let name_and_type_index = slice.read_u2()?;
        Ok(ConstantInterfaceMethodref {
            class_index,
            name_and_type_index,
        })
    }
}

#[derive(Clone)]
pub struct ConstantString {
    string_index: u2,
}

impl CPElem for ConstantString {
    fn tag() -> Tag { Tag::STRING }
}

impl ConstantString {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        Ok(ConstantString {
            string_index: slice.read_u2()?,
        })
    }
}

#[derive(Clone, Copy)]
pub struct ConstantInteger(u4);

impl CPElem for ConstantInteger {
    fn tag() -> Tag { Tag::INTEGER }
}

impl ConstantInteger {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        Ok(ConstantInteger(slice.read_u4()?))
    }
}

#[derive(Clone, Copy)]
pub struct ConstantFloat(u4);

impl CPElem for ConstantFloat {
    fn tag() -> Tag { Tag::FLOAT }
}

impl ConstantFloat {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        Ok(ConstantFloat(slice.read_u4()?))
    }
}

#[derive(Clone, Copy)]
pub struct ConstantLong(u4, u4);

impl CPElem for ConstantLong {
    fn tag() -> Tag { Tag::LONG }
}

impl ConstantLong {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        let low_bytes = slice.read_u4()?;
        let hig_bytes = slice.read_u4()?;
        Ok(ConstantLong(low_bytes, hig_bytes))
    }
}

#[derive(Clone, Copy)]
pub struct ConstantDouble(u4, u4);

impl CPElem for ConstantDouble {
    fn tag() -> Tag { Tag::DOUBLE }
}

impl ConstantDouble {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        let low_bytes = slice.read_u4()?;
        let hig_bytes = slice.read_u4()?;
        Ok(ConstantDouble(low_bytes, hig_bytes))
    }
}

#[derive(Clone)]
pub struct ConstantNameAndType {
    name_index: u2,
    descriptor_index: u2,
}

impl CPElem for ConstantNameAndType {
    fn tag() -> Tag { Tag::NAME_AND_TYPE }
}

impl ConstantNameAndType {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        let name_index = slice.read_u2()?;
        let descriptor_index = slice.read_u2()?;
        Ok(ConstantNameAndType {
            name_index,
            descriptor_index,
        })
    }
}

#[derive(Clone)]
pub struct ConstantUtf8(pub(super) String);

impl CPElem for ConstantUtf8 {
    fn tag() -> Tag { Tag::UTF8 }
}

impl ConstantUtf8 {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        let length = slice.read_u2()? as usize;
        let text = String::from_utf8(slice.read_slice_vec(length)?)
            .map_err(|_| Error::InvalidUtf8);
        Ok(ConstantUtf8(text?))
    }
}

#[repr(u8)]
#[derive(Primitive, Clone, Copy)]
pub enum RefKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

impl RefKind {
    pub fn read(val: u1) -> Result<Self> {
        RefKind::from_u8(val).ok_or(Error::WrongKind(val))
    }
}

#[derive(Clone)]
pub struct ConstantMethodHandle {
    reference_kind: RefKind,
    reference_index: u2,
}

impl CPElem for ConstantMethodHandle {
    fn tag() -> Tag { Tag::METHOD_HANDLE }
}

impl ConstantMethodHandle {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        let ref_kind = slice.read_u1()?;
        let reference_index = slice.read_u2()?;
        Ok(ConstantMethodHandle {
            reference_kind: RefKind::read(ref_kind)?,
            reference_index,
        })
    }
}

#[derive(Clone)]
pub struct ConstantMethodType {
    descriptor_index: u2,
}

impl CPElem for ConstantMethodType {
    fn tag() -> Tag { Tag::METHOD_TYPE }
}

impl ConstantMethodType {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        Ok(ConstantMethodType {
            descriptor_index: slice.read_u2()?,
        })
    }
}

#[derive(Clone)]
pub struct ConstantInvokeDynamic {
    bootstrap_method_attr_index: u2,
    name_and_type_index: u2,
}

impl CPElem for ConstantInvokeDynamic {
    fn tag() -> Tag { Tag::INVOKE_DYNAMIC }
}

impl ConstantInvokeDynamic {
    pub fn read(slice: &PoSlice) -> Result<Self> {
        let bootstrap_method_attr_index = slice.read_u2()?;
        let name_and_type_index = slice.read_u2()?;
        Ok(ConstantInvokeDynamic {
            bootstrap_method_attr_index,
            name_and_type_index,
        })
    }
}

/* End of working on progress */

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

impl Tag {
    pub fn new(val: u8) -> Result<Self> {
        Self::from_u8(val).ok_or(Error::CPTag(val))
    }
}