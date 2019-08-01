use std::io::*;

pub fn read_u1(r: &mut impl Read) -> Result<u8> {
    let mut buffer = [0u8; 1];
    r.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

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

//pub fn read_i1(r: &mut impl Read) -> Result< i8> { read_u1(r).map(|u| u as  i8) }
//pub fn read_i2(r: &mut impl Read) -> Result<i16> { read_u2(r).map(|u| u as i16) }
pub fn read_i4(r: &mut impl Read) -> Result<i32> { read_u4(r).map(|u| u as i32) }
pub fn read_i8(r: &mut impl Read) -> Result<i64> { read_u8(r).map(|u| u as i64) }
