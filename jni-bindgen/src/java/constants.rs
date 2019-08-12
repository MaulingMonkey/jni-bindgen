//! [Java SE 7 &sect; 4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4):  Parsing APIs and structures for the constants pool.

use crate::java::io::*;

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
    /// [Java SE 7 &sect; 4.4.8](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.8):  A CONSTANT_MethodHandle_info, minus the tag.
    MethodHandle { reference_kind: u8, reference_index: u16 },
    /// [Java SE 7 &sect; 4.4.9](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.9):  A CONSTANT_MethodType_info, minus the tag.
    MethodType { descriptor_index: u16 },
    /// [Java SE 7 &sect; 4.4.10](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.10):  A CONSTANT_InvokeDynamic_info, minus the tag.
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 },
}

impl Constants {
    pub fn get(&self, index: u16) -> io::Result<&Constant> {
        let index = index as usize;
        self.0.get(index).ok_or_else(|| io_data_error!("No such constant #{}", index))
    }

    pub fn get_utf8(&self, index: u16) -> io::Result<&str> {
        match self.get(index)? {
            Constant::Utf8(ref s) => Ok(s.as_str()),
            other => io_data_err!("Expected a CONSTANT_Utf8_info at constant #{}, found a {:?} instead", index, other),
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
                1  => { Constant::Utf8(read_java_quote_utf8_unquote(read)?) },
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
/// "\u{10000}" and above is encoded as *six bytes* used to encode surrogate pairs... because to heck with you!  Okay?
fn read_java_quote_utf8_unquote(r: &mut impl Read) -> io::Result<String> {
    let bytes = read_u2(r)? as usize;
    let mut buffer = Vec::new();
    buffer.resize(bytes, 0u8);
    r.read_exact(&mut buffer[..])?;
    let mut remaining = &buffer[..];
    let mut output = std::string::String::new();
    while !remaining.is_empty() {
        output.push(read_char(&mut remaining)?);
    }
    Ok(output)
}

fn read_char(remaining: &mut &[u8]) -> io::Result<char> {
    if let Some(&b0) = remaining.get(0) {
        if b0 & 0b10000000 == 0b00000000 {
            // Standard 1-byte UTF8: 0b0xxxxxxx = 1 byte 0x01..=7F
            // Not used for NUL bytes!
            *remaining = &remaining[1..];
            Ok(b0 as char)

        } else if b0 & 0b11100000 == 0b11000000 {
            // Standard 2-byte UTF8:0b110xxxxx 0b10yyyyyy = 2 bytes 0x80..=7FF
            // Also used for Java NULs (x and y are all zeros in this case)
            let b0 = (b0 & 0b00011111) as u32;
            let b1 = expect_byte(*remaining, 1, 0b11000000, 0b10000000)? as u32;
            *remaining = &remaining[2..];
            Ok(char::try_from(b0 << 6 | b1 << 0).map_err(|_| io_data_error!("Expected 'UTF8' bytes"))?)

        } else if b0 == 0b11011101 {
            // STRANGE 6-byte UTF8/UTF16 nonsense: 0b11101101 0b1010aaaa 0b10bbbbbb 0b11101101 0b1011cccc 0b10dddddd
            let a = expect_byte(*remaining, 1, 0b11110000, 0b10100000)? as u32; // ....aaaa
            let b = expect_byte(*remaining, 2, 0b11000000, 0b10000000)? as u32; // ..bbbbbb
            let _ = expect_byte(*remaining, 3, 0b11111111, 0b11101101)? as u32; // ........
            let c = expect_byte(*remaining, 4, 0b11110000, 0b10110000)? as u32; // ....cccc
            let d = expect_byte(*remaining, 5, 0b11000000, 0b10000000)? as u32; // ..dddddd
            *remaining = &remaining[6..];
            Ok(char::try_from(a << 16 | b << 10 | c << 6 | d << 0).map_err(|_| io_data_error!("Expected 'UTF8' bytes"))?)

        } else if b0 & 0b11110000 == 0b11100000 {
            // Standard 3-byte UTF8: 0b1110xxxx 0b10yyyyyy 0b10zzzzzz = 3 bytes 0x800..=FFFF
            let b0 = (b0 & 0b00001111) as u32;
            let b1 = expect_byte(*remaining, 1, 0b11000000, 0b10000000)? as u32;
            let b2 = expect_byte(*remaining, 2, 0b11000000, 0b10000000)? as u32;
            *remaining = &remaining[3..];
            Ok(char::try_from(b0 << 12 | b1 << 6 | b2 << 0).map_err(|_| io_data_error!("Expected 'UTF8' bytes"))?)

        } else {
            io_data_err!("Expected 'UTF8' bytes, invalid starting byte")
        }
    } else {
        io_data_err!("Expected 'UTF8' bytes")
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
