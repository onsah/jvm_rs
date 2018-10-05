/* 
*This mod is same as from_bytes except this structs require a reference to constant_pool
*/
use std::rc::Rc;
use class_file::attribute_info::AttributeInfo;
use class_file::class_file::MemberInfo;
use class_file::constant_pool::ConstantPoolRep;
use class_file::pos_slice::PoSlice;
use result::Result;

pub trait Read<'a>: Sized {
    fn read(slice: &'a PoSlice<'a>, constant_pool: Rc<ConstantPoolRep>) -> Result<Self>;  
}

impl<'a> Read<'a> for AttributeInfo {
    #[inline]
    fn read(slice: &'a PoSlice<'a>, constant_pool: Rc<ConstantPoolRep>) -> Result<Self> {
        AttributeInfo::new(slice, constant_pool)
    }
}

impl<'a> Read<'a> for MemberInfo {
    fn read(slice: &'a PoSlice<'a>, constant_pool: Rc<ConstantPoolRep>) -> Result<Self> {
        let access_flags = slice.read_u2()?;
        let name_index = slice.read_u2()?;
        let descriptor_index = slice.read_u2()?;
        let attributes = AttributeInfo::read_attributes(slice, constant_pool.clone())?;
        Ok(MemberInfo {
            constant_pool,
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

impl<'a, T: Read<'a>> Read<'a> for Box<[T]> {
    fn read(slice: &'a PoSlice<'a>, constant_pool: Rc<ConstantPoolRep>) -> Result<Self> {
        let count = slice.read_u2()?;
        let mut vec = Vec::with_capacity(count as usize);
        for _ in 0..count {
            vec.push(Read::read(slice, constant_pool.clone())?);
        }
        Ok(vec.into_boxed_slice())
    }
}