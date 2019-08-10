//! [Java SE 7 &sect; 4.5](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.5):  Parsing APIs and structures for class fields.

use super::*;



bitflags! {
    #[derive(Default)]
    /// [Java SE 7 &sect; 4.5](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.5):  field_info::access_flags
    pub struct FieldAccessFlags : u16 {
        /// Declared `public`; may be accessed from outside its package.
        const PUBLIC        = 0x0001;
        /// Declared `private`; usable only with the defining class.
        const PRIVATE       = 0x0002;
        /// Declared `protectdd`; may be accessed within subclasses.
        const PROTECTED     = 0x0004;
        /// Declared `static`.
        const STATIC        = 0x0008;
        /// Declared `final`; no subclasses allowed.
        const FINAL         = 0x0010;
        /// Declared `volatile`; cannot be cached.
        const VOLATILE      = 0x0040;
        /// Declared `transient`; not written or read by a persistent object manager.
        const TRANSIENT     = 0x0080;
        /// Declared synthetic; not present in the source code.
        const SYNTHETIC     = 0x1000;
        /// Declared as an enum type.
        const ENUM          = 0x4000;
    }
}

impl FieldAccessFlags {
    pub(crate) fn read(r: &mut impl Read) -> io::Result<Self> {
        Ok(Self::from_bits_truncate(read_u2(r)?))
    }
}



/// [Java SE 7 &sect; 4.5](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.5):  field_info, minus the attributes array
#[derive(Clone, Debug)] // XXX: Re-enable Copy if inferred fields removed
pub struct Field {
    pub access_flags:       FieldAccessFlags,
    pub name_index:         u16,
    pub descriptor_index:   u16,
    pub attributes_count:   u16,

    // XXX: Values inferred from attributes
    pub(crate) deprecated:         bool,
    pub(crate) rust_const_value:   Option<String>,
}

impl Field {
    pub(crate) fn read_except_attributes(read: &mut impl Read) -> io::Result<Self> {
        Ok(Self{
            access_flags:       FieldAccessFlags::read(read)?,
            name_index:         read_u2(read)?,
            descriptor_index:   read_u2(read)?,
            attributes_count:   read_u2(read)?,
            deprecated:         false,
            rust_const_value:   None,
        })
    }

    pub(crate) fn read_list_visitor(read: &mut impl Read, visitor: &mut impl Visitor) -> io::Result<()> {
        let field_count = read_u2(read)?;
        for field_index in 0..field_count {
            let field = Field::read_except_attributes(read)?;
            visitor.on_field(field_index, field.clone());
            Attribute::read_list_callback(read, field.attributes_count, |attribute_index, attribute| visitor.on_field_attribute(field_index, attribute_index, attribute))?;
        }
        Ok(())
    }
}



/// [Java SE 7 &sect; 4.5](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.5):  Visit a field_info
pub trait Visitor {
    fn on_field(&mut self, _index: u16, _field: Field) {}
    fn on_field_attribute(&mut self, _field_index: u16, _attribute_index: u16, _attribute: Attribute) {}
}
