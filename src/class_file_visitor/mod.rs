//! [Java SE 7 &sect; 4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html):  Lower level I/O for parsing .class files

use bitflags::bitflags;

    mod attribute;
pub mod constant;
pub mod field;
    mod header;
    mod io_ext;
pub mod method;
    mod version;

//use attribute::*;
//use constant::*;
//use field::*;
//use header::*;
use io_ext::*;
//use method::*;
//use version::*;

pub use attribute::{Attribute};
pub use constant::{Visitor as ConstantVisitor};
pub use field::{Field, Visitor as FieldVisitor};
pub use header::{Header};
//pub use io_ext::{};
pub use method::{Method, Visitor as MethodVisitor};
pub use version::{MajorVersion};

use std::io::{self, *};

// https://en.wikipedia.org/wiki/Java_class_file
// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html

/// [Java SE 7 &sect; 4.1](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1):  Visitor for ClassFile fields as read straight from I/O.
pub trait Visitor : constant::Visitor + field::Visitor + method::Visitor {
    // Methods + comments are organized by invoke order

    fn on_header(&mut self, _header: Header) {}
    // constant::Visitor
    fn on_class_access_flags(&mut self, _class_access_flags: ClassAccessFlags) {}
    fn on_this_class(&mut self, _this_class: u16) {}
    fn on_super_class(&mut self, _super_class: u16) {}
    fn on_interface(&mut self, _interface: u16) {}
    // field::Visitor
    // method::Visitor
    fn on_class_attribute(&mut self, _attribute_index: u16, _class_attribute: Attribute) {}
}

bitflags! {
    #[derive(Default)]
    /// [Java SE 7 &sect; 4.1](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1):  ClassFile::access_flags values.
    pub struct ClassAccessFlags : u16 {
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

impl ClassAccessFlags {
    pub(crate) fn read(r: &mut impl Read) -> io::Result<Self> {
        Ok(Self::from_bits_truncate(read_u2(r)?))
    }
}

/// [Java SE 7 &sect; 4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html):  Read a class File.
pub fn read(read: &mut impl Read, visitor: &mut impl Visitor) -> io::Result<()> {
    visitor.on_header(Header::read(read)?);
    constant::read_pool_visitor(read, visitor)?;
    visitor.on_class_access_flags(ClassAccessFlags::read(read)?);
    visitor.on_this_class(read_u2(read)?);
    visitor.on_super_class(read_u2(read)?);
    let interfaces = read_u2(read)?;
    for _ in 0..interfaces {
        visitor.on_interface(read_u2(read)?);
    }
    Field::read_list_visitor(read, visitor)?;
    Method::read_list_visitor(read, visitor)?;
    let class_attribute_count = read_u2(read)?;
    Attribute::read_list_callback(read, class_attribute_count, |index, attribute| visitor.on_class_attribute(index, attribute))?;
    Ok(())
}
