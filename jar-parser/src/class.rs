//! [Java SE 7 &sect; 4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html):  Lower level I/O for parsing .class files

// https://en.wikipedia.org/wiki/Java_class_file
// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html

use super::*;
use crate::io::*;

use bitflags::bitflags;

use std::collections::*;
use std::io::{self, Read};



bitflags! {
    #[derive(Default)]
    /// [Java SE 7 &sect; 4.1](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1):  ClassFile::access_flags values.
    pub struct Flags : u16 {
        /// Declared `public`; may be accessed from outside its package.
        const PUBLIC        = 0x0001;
        /// Declared `static`.
        const STATIC        = 0x0008;
        /// Declared `final`; no subclasses allowed.
        const FINAL         = 0x0010;
        /// Treat superclass methods specifically when invoked by the *invokespecial* instruction.
        const SUPER         = 0x0020;
        /// Is an interface, not a class.
        const INTERFACE     = 0x0200;
        /// Declared `abstract`; must not be instantiated.
        const ABSTRACT      = 0x0400;
        /// Declared synthetic; not present in the source code.
        const SYNTHETIC     = 0x1000;
        /// Declared as an annotation type.
        const ANNOTATION    = 0x2000;
        /// Declared as an enum type.
        const ENUM          = 0x4000;
    }
}

impl Flags {
    pub(crate) fn read(r: &mut impl Read) -> io::Result<Self> {
        Ok(Self::from_bits_truncate(read_u2(r)?))
    }
}



/// [Java SE 7 &sect; 4.1](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1):  The first few fields of a given ClassFile.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
struct Header {
    pub magic:          [u8; 4],
    pub minor_version:  u16,
    pub major_version:  version::Major,
}

impl Header {
    pub(crate) fn read(reader: &mut impl Read) -> io::Result<Header> {
        let mut h = Header::default();
        reader.read_exact(&mut h.magic)?;
        if h.magic != [0xCA, 0xFE, 0xBA, 0xBE] { return io_data_err!("Invalid header magic, not a class file"); }
        h.minor_version = read_u2(reader)?;
        h.major_version = version::Major(read_u2(reader)?);
        Ok(h)
    }
}



#[derive(Clone, Debug, Default)]
pub struct Class {
    pub flags:      Flags,
    pub path:       String,
    pub super_path: Option<String>,
    pub interfaces: Vec<String>,
    pub fields:     Vec<Field>,
    pub methods:    Vec<Method>,
    pub deprecated: bool,
    inner_classes:  BTreeMap<String, Class>,
}

impl Class {
    /// [Java SE 7 &sect; 4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html):  Read a class File.
    pub fn read(read: &mut impl Read) -> io::Result<Self> {
        let _header     = Header::read(read)?;
        let constants   = Constants::read(read)?;
        let flags       = Flags::read(read)?;
        let path        = constants.get_class(read_u2(read)?)?.to_owned();
        let super_path  = constants.get_optional_class(read_u2(read)?)?.map(|s| s.to_owned());

        let interfaces_count = read_u2(read)? as usize;
        let mut interfaces = Vec::with_capacity(interfaces_count);
        for _ in 0..interfaces_count {
            interfaces.push(constants.get_class(read_u2(read)?)?.to_owned());
        }

        let fields  = Field::read_list(read, &constants)?;
        let methods = Method::read_list(read, &constants)?;

        let attributes_count = read_u2(read)?;
        let mut deprecated = false;
        for _ in 0..attributes_count {
            match Attribute::read(read, &constants)? {
                Attribute::Deprecated { .. } => { deprecated = true; },
                _ => {},
            }
        }

        Ok(Self {
            flags,
            path,
            super_path,
            interfaces,
            fields,
            methods,
            deprecated,
            inner_classes: BTreeMap::new(),
        })
    }

    pub fn is_public(&self)         -> bool { self.flags.contains(Flags::PUBLIC) }
    pub fn is_static(&self)         -> bool { self.flags.contains(Flags::STATIC) }
    pub fn is_final(&self)          -> bool { self.flags.contains(Flags::FINAL) }
    pub fn is_super(&self)          -> bool { self.flags.contains(Flags::SUPER) }
    pub fn is_interface(&self)      -> bool { self.flags.contains(Flags::INTERFACE) }
    pub fn is_abstract(&self)       -> bool { self.flags.contains(Flags::ABSTRACT) }
    pub fn is_synthetic(&self)      -> bool { self.flags.contains(Flags::SYNTHETIC) }
    pub fn is_annotation(&self)     -> bool { self.flags.contains(Flags::ANNOTATION) }
    pub fn is_enum(&self)           -> bool { self.flags.contains(Flags::ENUM) }
}
