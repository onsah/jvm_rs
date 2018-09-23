use std::rc::Rc;
use num_traits::FromPrimitive;

use class_file::class_file::{ClassFile, MemberInfo};
use class_file::attribute_info::{AttributeInfo, Exception};
use class_file::constant_pool::{ConstantPool, CPInfo, Tag};
use class_file::pos_slice::PoSlice;
use class_file::read::Read;
use types::{u1, u2, u4};
use result::{Result, Error};

pub trait FromBytes<'a>
where Self: Sized {
    fn from_bytes(slice: &'a PoSlice) -> Result<Self>;
}

/* class_file */

// Yes I can half the length of this function but Rust doesn't guarantee that fields are executed in order although it works as exepcted.
impl<'a> FromBytes<'a> for ClassFile<'a> {
    fn from_bytes(slice: &'a PoSlice) -> Result<Self> {
        let magic = slice.read_u4()?;
        let minor_version = slice.read_u2()?;
        let major_version = slice.read_u2()?;
        let constant_pool = ConstantPool::from_bytes(slice)?;
        let access_flags = slice.read_u2()?;
        let this_class = slice.read_u2()?;
        let super_class = slice.read_u2()?;
        let interfaces = <Box<[u2]>>::from_bytes(slice)?;
        let fields = <Box<[MemberInfo<'a>]>>::read(slice, constant_pool.clone())?;
        let methods = <Box<[MemberInfo<'a>]>>::read(slice, constant_pool.clone())?;
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

type Members<'a> = Vec<MemberInfo<'a>>;

impl<'a, T: 'a> FromBytes<'a> for Vec<T> 
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

/* constant_pool */

impl<'a> FromBytes<'a> for ConstantPool<'a> {
    fn from_bytes(slice: &'a PoSlice) -> Result<Self> {
        let constant_pool_count = slice.read_u2()?;
        let mut constant_pool = Vec::with_capacity(constant_pool_count as usize);
        {
            let mut jump = true;
            for i in 0..constant_pool.capacity() {
                let cp_info = if !jump {
                    let cp_info = CPInfo::from_bytes(slice)?;
                    jump = match cp_info.tag {
                        Tag::LONG | Tag::DOUBLE => true,
                        _ => false,
                    };
                    println!("{}: {:?}", i, cp_info.tag);
                    Some(cp_info)
                } else {
                    println!("{}: jumped", i);
                    jump = false;
                    None
                };
                constant_pool.push(cp_info);
            }
        }
        Ok(ConstantPool(Rc::new(constant_pool)))
    }
}

impl<'a> FromBytes<'a> for CPInfo<'a> {
    fn from_bytes(slice: &'a PoSlice) -> Result<Self> {
        let tag = slice.read_u1()?;
        let tag = Tag::from_u8(tag).ok_or(Error::CPTag(tag))?;
        let get_size = || -> Result<usize> { 
            Ok(match tag {
                Tag::INTEGER | Tag::FLOAT | Tag::FIELD_REF | Tag::METHOD_REF | 
                Tag::INTERFACE_METHOD_REF | Tag::NAME_AND_TYPE | Tag::INVOKE_DYNAMIC => 4,
                Tag::METHOD_HANDLE => 3,
                Tag::LONG | Tag::DOUBLE => 8,
                Tag::CLASS | Tag::STRING | Tag::METHOD_TYPE => 2,
                Tag::UTF8 => 2 + slice.peek_u2()? as usize,
            })
        };
        let size = get_size()?;
        Ok(CPInfo {
            tag,
            info: slice.read_slice(size as usize)?,
        })
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