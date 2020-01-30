//! [Java SE 7 &sect; 4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4):  Parsing APIs and structures for the constants pool.

use crate::io::be::*;

use bugsalot::*;

use std::convert::*;
use std::io::{self, Read};



/// [Java SE 7 &sect; 4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4):  A CONSTANT_* values.
#[derive(Clone, Debug, Default)]
pub struct Constants(pub(crate) Vec<Constant>);

/// [Java SE 7 &sect; 4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4):  A CONSTANT_* value.  Not ABI compatible with the raw C ABIs but that's fine.
#[derive(Clone, Debug)]
pub enum Constant {
    /// The constants table (and *only* the constants table) is 1-indexed.  That's just confusing.  Even worse, `Long` and `Double` take up two slots.  So I emit this as a placeholder for those slots.
    UnusedPlaceholder,
    /// [Java SE 7 &sect; 4.4.1](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.1):  A CONSTANT_Class_info, minus the tag.
    Class { name_index: u16 },
    /// [Java SE 7 &sect; 4.4.2](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.2):  A CONSTANT_Fieldref_info, minus the tag.
    Fieldref { class_index: u16, name_and_type_index: u16 },
    /// [Java SE 7 &sect; 4.4.2](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.2):  A CONSTANT_Methodref_info, minus the tag.
    Methodref { class_index: u16, name_and_type_index: u16 },
    /// [Java SE 7 &sect; 4.4.2](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.2):  A CONSTANT_InstanceMethodref_info, minus the tag.
    InterfaceMethodref { class_index: u16, name_and_type_index: u16 },
    /// [Java SE 7 &sect; 4.4.3](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.3):  A CONSTANT_String_info, minus the tag.
    String { string_index: u16 },
    /// [Java SE 7 &sect; 4.4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.4):  A CONSTANT_Integer_info, minus the tag.
    Integer(i32),
    /// [Java SE 7 &sect; 4.4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.4):  A CONSTANT_Float_info, minus the tag.
    Float(f32),
    /// [Java SE 7 &sect; 4.4.5](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.5):  A CONSTANT_Long_info, minus the tag.
    Long(i64),
    /// [Java SE 7 &sect; 4.4.5](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.5):  A CONSTANT_Double_info, minus the tag.
    Double(f64),
    /// [Java SE 7 &sect; 4.4.6](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.6):  A CONSTANT_NameAndType_info, minus the tag.
    NameAndType { name_index: u16, descriptor_index: u16 },
    /// [Java SE 7 &sect; 4.4.7](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.7):  A CONSTANT_Utf8_info, minus the tag.
    Utf8(String),
    /// [Java SE 7 &sect; 4.4.7](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.7):  A CONSTANT_Utf8_info, minus the tag, but containing mispaired surrogate sequences, such as U+DBEC U+0
    MispairedUtf16(Vec<u16>),
    /// [Java SE 7 &sect; 4.4.7](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.7):  A CONSTANT_Utf8_info, minus the tag, but containing completely invalid "Modified UTF8"
    InvalidModifiedUtf8(Vec<u8>),
    /// [Java SE 7 &sect; 4.4.8](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.8):  A CONSTANT_MethodHandle_info, minus the tag.
    MethodHandle { reference_kind: u8, reference_index: u16 },
    /// [Java SE 7 &sect; 4.4.9](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.9):  A CONSTANT_MethodType_info, minus the tag.
    MethodType { descriptor_index: u16 },
    /// [Java SE 7 &sect; 4.4.10](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.10):  A CONSTANT_InvokeDynamic_info, minus the tag.
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 },

    #[doc(hidden)] _NonExhaustive,
}

impl Constants {
    pub fn get(&self, index: u16) -> io::Result<&Constant> {
        let index = index as usize;
        self.0.get(index).ok_or_else(|| io_data_error!("No such constant #{}", index))
    }

    pub fn get_utf8_possibly_invalid(&self, index: u16) -> io::Result<Option<&str>> {
        match self.get(index)? {
            Constant::Utf8(ref s)               => Ok(Some(s.as_str())),
            Constant::MispairedUtf16(_)         => Ok(None),
            Constant::InvalidModifiedUtf8(_)    => Ok(None),
            other                               => io_data_err!("Expected a CONSTANT_Utf8_info at constant #{}, found a {:?} instead", index, other),
        }
    }

    pub fn get_utf8(&self, index: u16) -> io::Result<&str> {
        match self.get(index)? {
            Constant::Utf8(ref s)               => Ok(s.as_str()),
            Constant::MispairedUtf16(_)         => io_data_err!("CONSTANT_Utf8_info at constant #{} has mispaired UTF16 surrogates", index),
            Constant::InvalidModifiedUtf8(_)    => io_data_err!("CONSTANT_Utf8_info at constant #{} has invalid 'Modified UTF8'", index),
            other                               => io_data_err!("Expected a CONSTANT_Utf8_info at constant #{}, found a {:?} instead", index, other),
        }
    }

    pub fn get_class(&self, index: u16) -> io::Result<&str> {
        match self.get(index)? {
            Constant::Class { name_index } => Ok(self.get_utf8(*name_index)?),
            other => io_data_err!("Expected a CONSTANT_Class_info at constant #{}, found a {:?} instead", index, other),
        }
    }

    pub fn get_optional_class(&self, index: u16) -> io::Result<Option<&str>> {
        if index == 0 { return Ok(None); }
        match self.get(index)? {
            Constant::Class { name_index } => Ok(Some(self.get_utf8(*name_index)?)),
            other => io_data_err!("Expected a CONSTANT_Class_info at constant #{}, found a {:?} instead", index, other),
        }
    }

    pub fn read(read: &mut impl Read) -> io::Result<Self> {
        let count = read_u2(read)?;
        let mut constants = Vec::with_capacity(count as usize);
        constants.push(Constant::UnusedPlaceholder);

        let mut index = 1; // "The constant_pool table is indexed from 1 to constant_pool_count-1."
        while index < count {
            debug_assert_eq!(index as usize, constants.len());

            // https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4
            let tag = read_u1(read)?;
            let constant = match tag {
                7  => Constant::Class { name_index: read_u2(read)? },
                9  => Constant::Fieldref { class_index: read_u2(read)?, name_and_type_index: read_u2(read)? },
                10 => Constant::Methodref { class_index: read_u2(read)?, name_and_type_index: read_u2(read)? },
                11 => Constant::InterfaceMethodref { class_index: read_u2(read)?, name_and_type_index: read_u2(read)? },
                8  => Constant::String { string_index: read_u2(read)? },
                3  => Constant::Integer(read_i4(read)?),
                4  => Constant::Float(f32::from_bits(read_u4(read)?)),
                5  => {
                    constants.push(Constant::Long(read_i8(read)?));
                    constants.push(Constant::UnusedPlaceholder);
                    index += 2;
                    continue;
                },
                6  => {
                    constants.push(Constant::Double(f64::from_bits(read_u8(read)?)));
                    constants.push(Constant::UnusedPlaceholder);
                    index += 2;
                    continue;
                },
                12 => Constant::NameAndType { name_index: read_u2(read)?, descriptor_index: read_u2(read)? },
                1  => read_modified_utf8(read)?,
                15 => Constant::MethodHandle { reference_kind: read_u1(read)?, reference_index: read_u2(read)? },
                16 => Constant::MethodType { descriptor_index: read_u2(read)? },
                18 => Constant::InvokeDynamic { bootstrap_method_attr_index: read_u2(read)?, name_and_type_index: read_u2(read)? },
                _ => { return io_data_err!("Expected CONSTANT_* value reading constant pool, got {:?}", tag); },
            };
            constants.push(constant);
            index += 1;
        }

        debug_assert_eq!(count as usize, constants.len());
        Ok(Constants(constants))
    }
}



/// Reads a Java "UTF8" string.  Which is not actually UTF8.  Weirdness:
/// "\u{0}" is encoded as *two bytes*, neither of which is 0.
/// "\u{10000}" and above is encoded as *six bytes* - each first encoded as UTF16 surrogate pairs, then encoded as UTF8.
/// Of course, parts of some versions of the JDK also contain mispaired surrogates and other fun stuff.
fn read_modified_utf8(r: &mut impl Read) -> io::Result<Constant> {
    let bytes = read_u2(r)? as usize;
    let mut buffer = Vec::new();
    buffer.resize(bytes, 0u8);
    r.read_exact(&mut buffer[..])?;

    if let Some(c) = read_modified_utf8_as_utf8             (&buffer[..]) { return Ok(Constant::Utf8(c)); }
    if let Some(c) = read_modified_utf8_as_mispaired_utf16  (&buffer[..]) { return Ok(Constant::MispairedUtf16(c)); }
    Ok(Constant::InvalidModifiedUtf8(buffer))
}

fn read_modified_utf8_as_utf8(buffer: &[u8]) -> Option<String> {
    let mut output = String::new();
    let mut remaining = buffer;
    while !remaining.is_empty() {
        let point = read_modified_utf8_point(&mut remaining).ok()?;
        match point {
            0xDC00..=0xDFFF => return None, // invalid leading low surrogate
            0xD800..=0xDBFF => {            // valid leading high surrogate
                let hi = point;
                let lo = read_modified_utf8_point(&mut remaining).ok()?;
                if !(0xDC00..=0xDFFF).contains(&lo) { return None; } // expected a low surrogate to trail a high surrogate
                output.push(char::try_from(0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00)).ok()?);
            },
            _ => output.push(char::try_from(point).ok()?),
        }
    }
    Some(output)
}

fn read_modified_utf8_as_mispaired_utf16(buffer: &[u8]) -> Option<Vec<u16>> {
    let mut output = Vec::new();
    let mut remaining = buffer;
    while !remaining.is_empty() {
        let p = read_modified_utf8_point(&mut remaining).ok()?;
        let p = u16::try_from(p).ok()?;
        output.push(p);
    }
    Some(output)
}

fn read_modified_utf8_point(remaining: &mut &[u8]) -> io::Result<u32> {
    if let Some(&b0) = remaining.get(0) {
        if b0 & 0b10000000 == 0b00000000 {
            // Standard 1-byte UTF8: 0b0xxxxxxx = 1 byte 0x01..=7F
            // Not used for NUL bytes!
            *remaining = &remaining[1..];
            Ok(b0 as u32)

        } else if b0 & 0b11100000 == 0b11000000 {
            // Standard 2-byte UTF8: 0b110xxxxx 0b10yyyyyy = 2 bytes 0x80..=7FF
            // Also used for Java NULs (x and y are all zeros in this case)
            let b0 = (b0 & 0b00011111) as u32;
            let b1 = expect_byte(*remaining, 1, 0b11000000, 0b10000000)? as u32;
            let ch = b0 << 6 | b1 << 0;
            *remaining = &remaining[2..];
            Ok(ch as u32)

        } else if b0 & 0b11110000 == 0b11100000 {
            // Standardish 3-byte UTF8: 0b1110xxxx  0b10yyyyyy  0b10zzzzzz = 3 bytes 0x800..=FFFF
            // One big caveat here - this can include surrogate pairs!
            let b0 = (b0 & 0b00001111) as u32;
            let b1 = expect_byte(*remaining, 1, 0b11000000, 0b10000000)? as u32;
            let b2 = expect_byte(*remaining, 2, 0b11000000, 0b10000000)? as u32;
            let ch = b0 << 12 | b1 << 6 | b2 << 0;
            *remaining = &remaining[3..];
            Ok(ch as u32)

        } else if b0 & 0b11111000 == 0b11110000 {
            // Standard 4-byte UTF8?: 0b11110xxx 0b10yyyyyy 0b10zzzzzz 0b10wwwwww ?
            // While the Java docs all seem to imply this isn't encoded, and I haven't seen it in the entire JDK, lets handle it anyways.
            let b0 = (b0 & 0b00000111) as u32;
            let b1 = expect_byte(*remaining, 1, 0b11000000, 0b10000000)? as u32;
            let b2 = expect_byte(*remaining, 2, 0b11000000, 0b10000000)? as u32;
            let b3 = expect_byte(*remaining, 3, 0b11000000, 0b10000000)? as u32;
            let ch = b0 << 18 | b1 << 12 | b2 << 6 | b3 << 0;
            *remaining = &remaining[2..];
            Ok(ch as u32)

        } else {
            io_data_err!("expected 'Modified UTF8', encountered invalid starting byte for character: {:02x}", b0)
        }
    } else {
        io_data_err!("expected 'Modified UTF8', encountered unexpected end of string")
    }
}

fn expect_byte(remaining: &[u8], index: usize, mask: u8, equal: u8) -> io::Result<u8> {
    if let Some(&value) = remaining.get(index) {
        if value & mask == equal {
            return Ok(value & !mask);
        } else {
            io_data_err!("Invalid 'UTF8' string - expected index {} byte {:b} & mask {:b} == {:b}", index, value, mask, equal)
        }
    } else {
        io_data_err!("Incomplete 'UTF8' string - expected more bytes")
    }
}
