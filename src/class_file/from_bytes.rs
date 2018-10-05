use std::rc::Rc;

use class_file::class_file::{ClassFile, MemberInfo};
use class_file::attribute_info::{AttributeInfo, Exception};
use class_file::constant_pool::ConstantPoolRep;
use class_file::pos_slice::PoSlice;
use class_file::read::Read;
use types::u2;
use result::Result;

pub trait FromBytes<'a>
where Self: Sized {
    fn from_bytes(slice: &'a PoSlice) -> Result<Self>;
}

/* class_file */

// Yes I can half the length of this function but Rust doesn't guarantee that fields are executed in order although it works as exepcted.
impl<'a> FromBytes<'a> for ClassFile {
    fn from_bytes(slice: &'a PoSlice) -> Result<Self> {
        let magic = slice.read_u4()?;
        let minor_version = slice.read_u2()?;
        let major_version = slice.read_u2()?;
        let constant_pool = ConstantPoolRep::read(slice)?;
        let constant_pool = Rc::new(constant_pool);
        let access_flags = slice.read_u2()?;
        let this_class = slice.read_u2()?;
        let super_class = slice.read_u2()?;
        let interfaces = <Box<[u2]>>::from_bytes(slice)?;
        let fields = <Box<[MemberInfo]>>::read(slice, constant_pool.clone())?;
        let methods = <Box<[MemberInfo]>>::read(slice, constant_pool.clone())?;
        let attributes = AttributeInfo::read_attributes(slice, constant_pool.clone())?;
        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool: constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}

impl<'a> FromBytes<'a> for u2 {
    fn from_bytes(slice: &'a PoSlice) -> Result<Self> {
        slice.read_u2()
    }
}

type Members<'a> = Vec<MemberInfo>;

impl<'a, T> FromBytes<'a> for Vec<T> 
where T: FromBytes<'a> {
    fn from_bytes(slice: &'a PoSlice) -> Result<Self> {
        let count = slice.read_u2()?;
        println!("count: {}", count);
        let mut cp_infos = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let cp_info = FromBytes::from_bytes(slice)?;
            cp_infos.push(cp_info);
        }
        Ok(cp_infos)
    }
}

impl<'a, T: 'a> FromBytes<'a> for Box<[T]> 
where T: FromBytes<'a> {
    fn from_bytes(slice: &'a PoSlice) -> Result<Self> {
        <Vec<T>>::from_bytes(slice).map(Vec::into_boxed_slice)
    }
}

/* attribute_info */

impl<'a> FromBytes<'a> for Exception {
    fn from_bytes(slice: &'a PoSlice) -> Result<Self> {
        Ok(Exception {
            start_pc: slice.read_u2()?,
            end_pc: slice.read_u2()?,
            handler_pc: slice.read_u2()?,
            catch_type: slice.read_u2()?,
        })
    }
}