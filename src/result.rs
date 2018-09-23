use std::result;

use class_file::constant_pool::Tag;
use types::{u1, u2};

pub type Result<T> = result::Result<T, Error>; 

#[derive(Debug)]
pub enum Error {
    CPTag(u1),
    Index(usize),
    WrongTag(Tag),
    EmptyCPInfo(u2),
    OutOfBounds(usize),
    InvalidUtf8,
    MainNotFound,
}