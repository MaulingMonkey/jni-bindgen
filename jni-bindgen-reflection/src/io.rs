#[allow(dead_code)]
pub mod be {
    use std::io::{Read, Result};
    pub use super::common::*;

    pub fn read_u2(r: &mut impl Read) -> Result<u16> {
        let mut buffer = [0u8; 2];
        r.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }
    
    pub fn read_u4(r: &mut impl Read) -> Result<u32> {
        let mut buffer = [0u8; 4];
        r.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    pub fn read_u8(r: &mut impl Read) -> Result<u64> {
        let mut buffer = [0u8; 8];
        r.read_exact(&mut buffer)?;
        Ok(u64::from_be_bytes(buffer))
    }

    pub fn read_i2(r: &mut impl Read) -> Result<i16> { read_u2(r).map(|u| u as i16) }
    pub fn read_i4(r: &mut impl Read) -> Result<i32> { read_u4(r).map(|u| u as i32) }
    pub fn read_i8(r: &mut impl Read) -> Result<i64> { read_u8(r).map(|u| u as i64) }
}

#[allow(dead_code)]
pub mod le {
    use std::io::{Read, Result};
    pub use super::common::*;

    pub fn read_u2(r: &mut impl Read) -> Result<u16> {
        let mut buffer = [0u8; 2];
        r.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    pub fn read_u4(r: &mut impl Read) -> Result<u32> {
        let mut buffer = [0u8; 4];
        r.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    pub fn read_u8(r: &mut impl Read) -> Result<u64> {
        let mut buffer = [0u8; 8];
        r.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }

    pub fn read_i2(r: &mut impl Read) -> Result<i16> { read_u2(r).map(|u| u as i16) }
    pub fn read_i4(r: &mut impl Read) -> Result<i32> { read_u4(r).map(|u| u as i32) }
    pub fn read_i8(r: &mut impl Read) -> Result<i64> { read_u8(r).map(|u| u as i64) }
}

#[allow(dead_code)]
pub mod common {
    use std::io::{Read, Result};

    pub fn read_u1(r: &mut impl Read) -> Result<u8> {
        let mut buffer = [0u8; 1];
        r.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    pub fn read_i1(r: &mut impl Read) -> Result< i8> { read_u1(r).map(|u| u as  i8) }

    pub fn read_ignore(read: &mut impl Read, bytes: usize) -> Result<()> {
        let mut info = Vec::new();
        info.resize(bytes, 0u8);
        read.read_exact(&mut info[..])?;
        Ok(())
    }
}



// I/O errors here "probably" indicate bugs in class parsing - break at the callsite for ease of debugging.
// The other alternative is you're parsing bad/corrupt classes, so good luck with that.

#[macro_export]
macro_rules! io_data_error {
    ($($arg:tt)*) => {{
        use bugsalot::*;
        let message = format!($($arg)*);
        bug!("{}", &message);
        std::io::Error::new(std::io::ErrorKind::InvalidData, message)
    }};
}

#[macro_export]
macro_rules! io_data_err {
    ($($arg:tt)*) => { Err($crate::io_data_error!($($arg)*)) };
}

macro_rules! io_assert {
    ($condition:expr) => {
        if !$condition {
            return $crate::io_data_err!("Assertion failed: {}", stringify!($condition));
        }
    };
    ($condition:expr, $($arg:tt)*) => {
        if !$condition {
            return $crate::io_data_err!($($arg)*);
        }
    };
}
