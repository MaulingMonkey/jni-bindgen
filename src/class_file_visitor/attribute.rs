use super::*;

use std::ops::*;

#[derive(Clone, Debug)]
pub struct Attribute {
    pub attribute_name_index:   u16,
    pub attribute_length:       u32,
    pub info:                   Vec<u8>,
}

impl Attribute {
    pub(crate) fn read(read: &mut impl Read) -> io::Result<Self> {
        let attribute_name_index =  read_u2(read)?;
        let attribute_length =      read_u4(read)?;
        let mut info = Vec::new();
        info.resize(attribute_length as usize, 0u8);
        read.read_exact(&mut info[..])?;

        Ok(Self{ attribute_name_index, attribute_length, info })
    }

    pub(crate) fn read_list_callback(read: &mut impl Read, count: u16, mut callback: impl FnMut(u16, Attribute)) -> io::Result<()> {
        for index in 0..count {
            callback(index, Attribute::read(read)?);
        }
        Ok(())
    }
}
