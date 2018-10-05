use std::rc::Rc;
use class_file::attribute_info::{AttributeInfo, CodeAttribute};
use class_file::constant_pool::{ConstantPoolRep, CPInfoRep};
use class_file::pos_slice::PoSlice;
use result::{Result, Error};
use types::{u2, u4};

#[allow(dead_code)]
pub struct ClassFile {
    pub(super) magic:          u4,
    pub(super) minor_version:  u2,
    pub(super) major_version:  u2,
    pub(super) constant_pool:  Rc<ConstantPoolRep>,
    pub(super) access_flags:   u2,
    pub(super) this_class:     u2,
    pub(super) super_class:    u2,
    pub(super) interfaces:     Box<[u2]>,
    pub(super) fields:         Box<[MemberInfo]>,
    pub(super) methods:        Box<[MemberInfo]>,
    pub(super) attributes:     Box<[AttributeInfo]>,  
}

impl<'a> ClassFile {
    pub fn minor_version(&self) -> u2 { self.minor_version }

    pub fn major_version(&self) -> u2 { self.major_version }
    
    pub fn constant_pool(&self) -> Rc<ConstantPoolRep> {
        self.constant_pool.clone()
    }

    pub fn interfaces(&self) -> &[u2] {
        self.interfaces.as_ref()
    }

    pub fn fields(&self) -> &[MemberInfo] {
        self.fields.as_ref()
    } 

    pub fn methods(&self) -> &[MemberInfo] {
        self.methods.as_ref()
    }

    pub fn name(&self) -> Result<&str> {
        let class_info = match self.constant_pool.get(self.this_class as usize)? {
            CPInfoRep::Class(class_info) => Ok(class_info),
            cp_info => Err(Error::WrongTag(cp_info.tag())),
        };
        match self.constant_pool.get(class_info?.name_index as usize)? {
            CPInfoRep::Utf8(utf8_info) => Ok(&utf8_info.0),
            cp_info => Err(Error::WrongTag(cp_info.tag())),
        }
    }

    pub fn super_name(&self) -> Result<&str> {
        let class_info = match self.constant_pool.get(self.super_class as usize)? {
            CPInfoRep::Class(class_info) => Ok(class_info),
            cp_info => Err(Error::WrongTag(cp_info.tag())),
        };
        match self.constant_pool.get(class_info?.name_index as usize)? {
            CPInfoRep::Utf8(utf8_info) => Ok(&utf8_info.0),
            cp_info => Err(Error::WrongTag(cp_info.tag())),
        }
    } 

    pub fn get_main_method(&self) -> Result<&MemberInfo> {
        for member_info in self.methods.iter() {
            if member_info.get_name()? == "main" && member_info.get_descriptor()? == "([Ljava/lang/String;)V" {
                return Ok(member_info);
            }
        }
        Err(Error::MainNotFound)
    }
}

pub struct MemberInfo {
    pub(super) constant_pool: Rc<ConstantPoolRep>,
    pub(super) access_flags: u2,
    pub(super) name_index: u2,
    pub(super) descriptor_index: u2,
    pub(super) attributes: Box<[AttributeInfo]>,
}

impl<'a> MemberInfo {
    pub fn new(slice: &'a PoSlice<'a>, constant_pool: Rc<ConstantPoolRep>) -> Result<Self> {
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

    pub fn read_members(slice: &'a PoSlice<'a>, constant_pool: Rc<ConstantPoolRep>) -> Result<Box<[Self]>> {
        let member_count = slice.read_u2()?;
        let mut vec = Vec::with_capacity(member_count as usize);
        for _ in 0..member_count {
            let member_info = MemberInfo::new(slice, constant_pool.clone())?;
            vec.push(member_info);
        }
        Ok(vec.into_boxed_slice())
    }

    pub fn access_flags(&self) -> u2 { 
        self.access_flags 
    }

    pub fn get_name(&self) -> Result<&str> {
        self.get_str(self.name_index as usize)
    }

    pub fn get_descriptor(&self) -> Result<&str> {
        self.get_str(self.descriptor_index as usize)
    }

    // use find_map when it gets stable -> https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find_map
    pub fn get_code_attribute(&self) -> Option<&CodeAttribute> {
        self.attributes.iter()
            .filter_map(|attr_info| {
                match attr_info {
                    AttributeInfo::Code(ref code_info) => Some(code_info),
                    _ => None,
                }
            })
            .next()
    }

    #[inline]
    fn get_str(&self, index: usize) -> Result<&str> {
        self.constant_pool.get(index)?.as_str()
    }
}