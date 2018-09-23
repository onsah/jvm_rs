use byteorder::{ByteOrder, BigEndian};

use class_file::constant_pool::{CPInfo, Tag, ClassInfo, Utf8Info};
use result::{Result, Error};

pub trait TryFrom<T>: Sized {
    fn tag() -> Tag;

    fn try_from(value: T) -> Result<Self>;

    fn tag_to_result(tag: Tag) -> Result<()> {
        if tag == Self::tag() {
            Ok(())
        } else {
            Err(Error::WrongTag(tag))
        }
    }
}

impl<'a> TryFrom<CPInfo<'a>> for ClassInfo {
    fn tag() -> Tag { Tag::CLASS }

    fn try_from(cp_info: CPInfo<'a>) -> Result<Self> {
        let tag = cp_info.tag;
        Self::tag_to_result(tag)
        .map(|_| ClassInfo {
            name_index: BigEndian::read_u16(cp_info.info),
        })
    }
}

impl<'a> TryFrom<CPInfo<'a>> for Utf8Info<'a> {
    fn tag() -> Tag { Tag::UTF8 }

    fn try_from(cp_info: CPInfo<'a>) -> Result<Self> {
        let tag = cp_info.tag;
        if tag == Self::tag() {
            Ok(Utf8Info {
                length: BigEndian::read_u16(&cp_info.info[..2]),
                bytes: &cp_info.info[2..],
            })
        } else {
            Err(Error::WrongTag(tag))
        }
    }
}