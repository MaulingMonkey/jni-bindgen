use super::*;
use std::io;

/// [Java SE 7 &sect; 4.1](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1):  The first few fields of a given ClassFile.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Header {
    pub magic:          [u8; 4],
    pub minor_version:  u16,
    pub major_version:  MajorVersion,
}

impl Header {
    pub(crate) fn read(reader: &mut impl Read) -> io::Result<Header> {
        let mut h = Header::default();
        reader.read_exact(&mut h.magic)?;
        if h.magic != [0xCA, 0xFE, 0xBA, 0xBE] { return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid header magic, not a class file")); }
        h.minor_version = read_u2(reader)?;
        h.major_version = MajorVersion::from(read_u2(reader)?);
        Ok(h)
    }
}
