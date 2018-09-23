use class_file::constant_pool::{ConstantPool, Utf8Info};
use class_file::from_bytes::FromBytes;
use class_file::pos_slice::PoSlice;
use result::Result;
use types::{u1, u2, u4};

const CONSTANT_VALUE: &str = "ConstantValue";
const CODE: &str = "Code";
const SOURCE_FILE: &str = "SourceFile";

#[derive(Clone)]
pub enum AttributeInfo<'a> {
    Raw(RawAttribute<'a>),
    Constant(ConstantValueAttribute),
    Code(CodeAttribute<'a>),
    Source(SourceFileAttribute<'a>),
}

impl<'a> AttributeInfo<'a> {
    pub fn new(slice: &'a PoSlice<'a>, constant_pool: ConstantPool<'a>) -> Result<Self> {
        let attribute_name_index = slice.read_u2()?;
        let info: Utf8Info<'a> = constant_pool.get(attribute_name_index as usize)?.into()?;
        let attribute_length = slice.read_u4()?;
        let name = info.get_string()?;
        Ok(match name.as_ref() {
            CONSTANT_VALUE => AttributeInfo::Constant(ConstantValueAttribute::new(slice, 
                name, 
                attribute_length, 
                constant_pool.clone())?),
            CODE => AttributeInfo::Code(CodeAttribute::new(slice, 
                name, 
                attribute_length, 
                constant_pool.clone())?),
            SOURCE_FILE => AttributeInfo::Source(SourceFileAttribute::new(slice, 
                name, 
                attribute_length, 
                constant_pool)?),
            _ => AttributeInfo::Raw(RawAttribute::new(slice, 
                name, 
                attribute_length, 
                constant_pool.clone())?),
        })
    }

    pub fn read_attributes(slice: &'a PoSlice<'a>, constant_pool: ConstantPool<'a>) -> Result<Box<[Self]>> {
        let attributes_length = slice.read_u2()?;
        println!("count: {}", attributes_length);
        let mut attribute_infos = Vec::with_capacity(attributes_length as usize);
        for _ in 0..attributes_length {
            let info = AttributeInfo::new(slice, constant_pool.clone())?;
            attribute_infos.push(info);
        }
        Ok(attribute_infos.into_boxed_slice())
    }

    pub fn is_code(&self) -> bool {
        match *self {
            AttributeInfo::Code(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct RawAttribute<'a> {
    pub(super) name: String,
    pub(super) info: &'a [u1],
}

impl<'a> RawAttribute<'a> {
    pub(super) fn new(slice: &'a PoSlice<'a>, name: String, length: u4, _constant_pool: ConstantPool<'a>) -> Result<Self> {
        Ok(RawAttribute {
            name,
            info: slice.read_slice(length as usize)?,
        })
    }
}

#[derive(Clone)]
pub struct ConstantValueAttribute {
    pub(super) name: String, 
    pub(super) constant_value_index: u2,
}

impl<'a> ConstantValueAttribute {
    pub(super) fn new(slice: &'a PoSlice<'a>, name: String, _length: u4, _constant_pool: ConstantPool<'a>) -> Result<Self> {
        Ok(ConstantValueAttribute {
            name,
            constant_value_index: slice.read_u2()?,
        })
    }
}

#[derive(Clone)]
pub struct CodeAttribute<'a> {
    pub(super) name: String,
    pub(super) constant_pool: ConstantPool<'a>,
    pub(super) max_stack: u2,
    pub(super) max_locals: u2,
    pub(super) code:  &'a [u1],
    pub(super) exception_table: Box<[Exception]>,
    pub(super) attributes: Box<[AttributeInfo<'a>]>,
}

impl<'a> CodeAttribute<'a> {
    pub(super) fn new(slice: &'a PoSlice<'a>, name: String, _length: u4, constant_pool: ConstantPool<'a>) -> Result<Self> {
        let max_stack = slice.read_u2()?;
        let max_locals = slice.read_u2()?;
        let code_length = slice.read_u4()?;
        let code = slice.read_slice(code_length as usize)?;
        let exception_table = <Box<[Exception]>>::from_bytes(slice)?;
        let attributes = AttributeInfo::read_attributes(slice, constant_pool.clone())?;
        Ok(CodeAttribute {
            name,
            constant_pool,
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        })
    }
}

#[derive(Clone)]
pub struct SourceFileAttribute<'a> {
    constant_pool: ConstantPool<'a>,
    name: String,
    sourcefile_index: u2,
}

impl<'a> SourceFileAttribute<'a> {
    pub(super) fn new(slice: &'a PoSlice<'a>, name: String, _length: u4, constant_pool: ConstantPool<'a>) -> Result<Self> {
        Ok(SourceFileAttribute {
            constant_pool,
            name,
            sourcefile_index: slice.read_u2()?,
        })
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Exception {
    pub(super) start_pc: u2,
    pub(super) end_pc: u2,
    pub(super) handler_pc: u2,
    pub(super) catch_type: u2,
}