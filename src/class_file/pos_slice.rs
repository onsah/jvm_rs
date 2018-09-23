use std::cell::Cell;
use byteorder::{ByteOrder, BigEndian};

use result::{Result, Error};
use types::{u1, u2, u4};

#[derive(Clone)]
pub struct PoSlice<'a> {
    bytes: &'a [u1],
    pos: Cell<usize>,
}

impl<'a> PoSlice<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        PoSlice { bytes, pos: Cell::new(0) }
    }

    #[inline]
    fn ensure_index(&self, pos: usize) -> Result<()> {
        println!("pos: {}", pos);
        if pos >= self.bytes.len() {
            Err(Error::OutOfBounds(pos))
        } else {
            Ok(())
        }
    }

    pub fn read_u1(&self) -> Result<u1> {
        self.ensure_index(self.pos.get())?;
        let byte = self.bytes[self.pos.get()];
        self.pos.set(self.pos.get() + 1);
        Ok(byte)
    }

    pub fn read_u2(&self) -> Result<u2> {
        self.ensure_index(self.pos.get() + 1)?;
        let byte = BigEndian::read_u16(&self.bytes[self.pos.get()..]);
        self.pos.set(self.pos.get() + 2);
        Ok(byte)
    }

    pub fn read_u4(&self) -> Result<u4> {
        self.ensure_index(self.pos.get() + 3)?;
        let byte = BigEndian::read_u32(&self.bytes[self.pos.get()..]);
        self.pos.set(self.pos.get() + 4);
        Ok(byte)
    }

    pub fn read_slice(&self, len: usize) -> Result<&[u1]> {
        self.ensure_index(self.pos.get() + len - 1)?;
        let slice = &self.bytes[self.pos.get()..self.pos.get() + len];
        self.pos.set(self.pos.get() + len);
        Ok(slice)
    }

    pub fn read_slice_vec(&self, len: usize) -> Result<Vec<u1>> {
        let pos = self.pos.get();
        self.ensure_index(pos + len - 1)?;
        let slice = &self.bytes[pos..pos + len];
        self.pos.set(pos + len);
        Ok(slice.to_vec())
    }

    #[inline]
    pub fn peek_u1(&self) -> Result<u1> {
        self.ensure_index(self.pos.get())?;
        Ok(self.bytes[self.pos.get()])
    }

    #[inline]
    pub fn peek_u2(&self) -> Result<u2> {
        self.ensure_index(self.pos.get() + 1)?;
        Ok(BigEndian::read_u16(&self.bytes[self.pos.get()..]))
    }

    #[inline]
    pub fn peek_u4(&self) -> Result<u4> {
        self.ensure_index(self.pos.get() + 3)?;
        Ok(BigEndian::read_u32(&self.bytes[self.pos.get()..]))
    }

    pub fn peek_slice(&self, pos: usize) -> Result<&[u1]> {
        self.ensure_index(self.pos.get() + pos)?;
        Ok(&self.bytes[self.pos.get() + pos..])
    }

    pub fn skip(&self, len: usize) -> Result<usize> {
        self.ensure_index(self.pos.get() + len)?;
        self.pos.set(self.pos.get() + len);
        Ok(len)
    }

    pub fn pos(&self) -> usize { self.pos.get() }
}