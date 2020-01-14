//! [Java SE 7 &sect; 4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html):  Lower level I/O for parsing .class files

// https://en.wikipedia.org/wiki/Java_class_file
// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html

use crate::*;
use crate::io::be::*;

use bitflags::bitflags;

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
    pub magic:          u32,
    pub minor_version:  u16,
    pub major_version:  version::Major,
}

impl Header {
    pub(crate) fn read(reader: &mut impl Read) -> io::Result<Header> {
        let mut h = Header::default();
        h.magic         = read_u4(reader)?;
        if h.magic != 0xCAFEBABE { return io_data_err!("Invalid header magic, not a class file"); }
        h.minor_version = read_u2(reader)?;
        h.major_version = version::Major(read_u2(reader)?);
        Ok(h)
    }
}



#[derive(Clone, Debug, Default)]
pub struct Class {
    pub flags:      Flags,
    pub path:       IdBuf,
    pub super_path: Option<IdBuf>,
    pub interfaces: Vec<IdBuf>,
    pub fields:     Vec<Field>,
    pub methods:    Vec<Method>,
    pub deprecated: bool,
}

#[allow(dead_code)]
impl Class {
    /// [Java SE 7 &sect; 4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html):  Read a class File.
    pub fn read(read: &mut impl Read) -> io::Result<Self> {
        let _header     = Header::read(read)?;
        let constants   = Constants::read(read)?;
        let flags       = Flags::read(read)?;
        let path        = IdBuf::new(constants.get_class(read_u2(read)?)?.to_owned());
        let super_path  = constants.get_optional_class(read_u2(read)?)?.map(|s| IdBuf::new(s.to_owned()));

        let interfaces_count = read_u2(read)? as usize;
        let mut interfaces = Vec::with_capacity(interfaces_count);
        for _ in 0..interfaces_count {
            interfaces.push(IdBuf::new(constants.get_class(read_u2(read)?)?.to_owned()));
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



#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdBuf(String);

impl IdBuf {
    pub fn new(s: String) -> Self { Self(s) }
    pub fn as_str(&self) -> &str { self.0.as_str() }
    pub fn as_id(&self) -> Id { Id(self.0.as_str()) }
    #[allow(dead_code)] pub fn iter(&self) -> IdIter { IdIter::new(self.0.as_str()) }
}

// XXX: This should really be `#[repr(transparent)] pub struct Id(str);`, but I've banned unsafe for this lib...
// Also, patterns apparently can't handle Id::new(...) even when it's a const fn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<'a>(pub &'a str);

impl<'a> Id<'a> {
    pub fn as_str(&self) -> &'a str { self.0 }
    pub fn iter(&self) -> IdIter<'a> { IdIter::new(self.0) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IdPart<'a> {
    Namespace(&'a str),
    ContainingClass(&'a str),
    LeafClass(&'a str),
}

pub struct IdIter<'a> {
    rest: &'a str,
}

impl<'a> IdIter<'a> {
    pub fn new(path: &'a str) -> Self { IdIter { rest: path } }
}

impl<'a> Iterator for IdIter<'a> {
    type Item = IdPart<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(slash) = self.rest.find('/') {
            let (namespace, rest) = self.rest.split_at(slash);
            self.rest = &rest[1..];
            return Some(IdPart::Namespace(namespace));
        }

        if let Some(dollar) = self.rest.find('$') {
            let (class, rest) = self.rest.split_at(dollar);
            self.rest = &rest[1..];
            return Some(IdPart::ContainingClass(class));
        }

        if !self.rest.is_empty() {
            let class = self.rest;
            self.rest = "";
            return Some(IdPart::LeafClass(class));
        }

        None
    }
}

#[test] fn id_iter_test() {
    assert_eq!(Id("").iter().collect::<Vec<IdPart>>(), &[]);

    assert_eq!(Id("Bar").iter().collect::<Vec<IdPart>>(), &[
        IdPart::LeafClass("Bar"),
    ]);

    assert_eq!(Id("java/foo/Bar").iter().collect::<Vec<IdPart>>(), &[
        IdPart::Namespace("java"),
        IdPart::Namespace("foo"),
        IdPart::LeafClass("Bar"),
    ]);

    assert_eq!(Id("java/foo/Bar$Inner").iter().collect::<Vec<IdPart>>(), &[
        IdPart::Namespace("java"),
        IdPart::Namespace("foo"),
        IdPart::ContainingClass("Bar"),
        IdPart::LeafClass("Inner"),
    ]);

    assert_eq!(Id("java/foo/Bar$Inner$MoreInner").iter().collect::<Vec<IdPart>>(), &[
        IdPart::Namespace("java"),
        IdPart::Namespace("foo"),
        IdPart::ContainingClass("Bar"),
        IdPart::ContainingClass("Inner"),
        IdPart::LeafClass("MoreInner"),
    ]);
}
