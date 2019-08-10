//! [Java SE 7 &sect; 4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4):  Parsing APIs and structures for the constants pool.

use super::*;
use std::convert::*;

pub(crate) fn read_pool_visitor(read: &mut impl Read, visitor: &mut impl Visitor) -> io::Result<()> {
    let count = read_u2(read)?;
    visitor.on_unused(0, UnusedPlaceholder {});
    let mut index = 1; // "The constant_pool table is indexed from 1 to constant_pool_count-1."
    while index < count {
        let tag = Type::from(read_u1(read)?);
        match tag {
            Class::TAG              => visitor.on_class(index, Class::read_after_tag(read)?),
            Fieldref::TAG           => visitor.on_field(index, Fieldref::read_after_tag(read)?),
            Methodref::TAG          => visitor.on_method(index, Methodref::read_after_tag(read)?),
            InterfaceMethodref::TAG => visitor.on_interface_method(index, InterfaceMethodref::read_after_tag(read)?),
            String::TAG             => visitor.on_string(index, String::read_after_tag(read)?),
            Integer::TAG            => visitor.on_integer(index, Integer::read_after_tag(read)?),
            Float::TAG              => visitor.on_float(index, Float::read_after_tag(read)?),
            Long::TAG               => {
                visitor.on_long(index, Long::read_after_tag(read)?);
                index += 1;
                visitor.on_unused(index, UnusedPlaceholder {});
            },
            Double::TAG             => {
                visitor.on_double(index, Double::read_after_tag(read)?);
                index += 1;
                visitor.on_unused(index, UnusedPlaceholder {});
            },
            NameAndType::TAG        => visitor.on_name_and_tag(index, NameAndType::read_after_tag(read)?),
            Utf8::TAG               => visitor.on_utf8(index, Utf8::read_after_tag(read)?),
            MethodHandle::TAG       => visitor.on_method_handle(index, MethodHandle::read_after_tag(read)?),
            MethodType::TAG         => visitor.on_method_type(index, MethodType::read_after_tag(read)?),
            InvokeDynamic::TAG      => visitor.on_invoke_dynamic(index, InvokeDynamic::read_after_tag(read)?),
            _ => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expected CONSTANT_* value reading constant pool, got {:?}", tag)));
            },
        }
        index += 1;
    }
    Ok(())
}



/// [Java SE 7 &sect; 4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4):  Constant pool tag types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Type(u8);

impl From<u8> for Type {
    fn from(value: u8) -> Self {
        Self(value)
    }
}



/// [Java SE 7 &sect; 4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.1):  Visits possible CONSTANT_* values in the constants table.
pub trait Visitor {
    fn on_unused                    (&mut self, _index: u16, _unused: UnusedPlaceholder) {}
    fn on_class                     (&mut self, _index: u16, _class: Class) {}
    fn on_field                     (&mut self, _index: u16, _field: Fieldref) {}
    fn on_method                    (&mut self, _index: u16, _method: Methodref) {}
    fn on_interface_method          (&mut self, _index: u16, _interface_method: InterfaceMethodref) {}
    fn on_string                    (&mut self, _index: u16, _string: String) {}
    fn on_integer                   (&mut self, _index: u16, _integer: Integer) {}
    fn on_float                     (&mut self, _index: u16, _float: Float) {}
    fn on_long                      (&mut self, _index: u16, _long: Long) {}
    fn on_double                    (&mut self, _index: u16, _double: Double) {}
    fn on_name_and_tag              (&mut self, _index: u16, _name_and_tag: NameAndType) {}
    fn on_utf8                      (&mut self, _index: u16, _utf8: Utf8) {}
    fn on_method_handle             (&mut self, _index: u16, _method_handle: MethodHandle) {}
    fn on_method_type               (&mut self, _index: u16, _method_type: MethodType) {}
    fn on_invoke_dynamic            (&mut self, _index: u16, _invoke_dynamic: InvokeDynamic) {}
}



/// The constants table (and *only* the constants table) is 1-indexed.  That's just confusing.  Even worse, `Long` and `Double` take up two slots.  So I emit this as a placeholder for those slots.
#[derive(Clone, Debug)] pub struct UnusedPlaceholder {}
/// [Java SE 7 &sect; 4.4.1](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.1):  A CONSTANT_Class_info, minus the tag.
#[derive(Clone, Debug)] pub struct Class                { pub name_index: u16 }
/// [Java SE 7 &sect; 4.4.2](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.2):  A CONSTANT_Fieldref_info, minus the tag.
#[derive(Clone, Debug)] pub struct Fieldref             { pub class_index: u16, pub name_and_type_index: u16 }
/// [Java SE 7 &sect; 4.4.2](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.2):  A CONSTANT_Methodref_info, minus the tag.
#[derive(Clone, Debug)] pub struct Methodref            { pub class_index: u16, pub name_and_type_index: u16 }
/// [Java SE 7 &sect; 4.4.2](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.2):  A CONSTANT_InstanceMethodref_info, minus the tag.
#[derive(Clone, Debug)] pub struct InterfaceMethodref   { pub class_index: u16, pub name_and_type_index: u16 }
/// [Java SE 7 &sect; 4.4.3](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.3):  A CONSTANT_String_info, minus the tag.
#[derive(Clone, Debug)] pub struct String               { pub string_index: u16 }
/// [Java SE 7 &sect; 4.4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.4):  A CONSTANT_Integer_info, minus the tag.
#[derive(Clone, Debug)] pub struct Integer              ( pub i32 );
/// [Java SE 7 &sect; 4.4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.4):  A CONSTANT_Float_info, minus the tag.
#[derive(Clone, Debug)] pub struct Float                ( pub f32 );
/// [Java SE 7 &sect; 4.4.5](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.5):  A CONSTANT_Long_info, minus the tag.
#[derive(Clone, Debug)] pub struct Long                 ( pub i64 ); // Note: Requires two constant pool entries
/// [Java SE 7 &sect; 4.4.5](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.5):  A CONSTANT_Double_info, minus the tag.
#[derive(Clone, Debug)] pub struct Double               ( pub f64 ); // Note: Requires two constant pool entries
/// [Java SE 7 &sect; 4.4.6](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.6):  A CONSTANT_NameAndType_info, minus the tag.
#[derive(Clone, Debug)] pub struct NameAndType          { pub name_index: u16, pub descriptor_index: u16 }
/// [Java SE 7 &sect; 4.4.7](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.7):  A CONSTANT_Utf8_info, minus the tag.
#[derive(Clone, Debug)] pub struct Utf8                 ( pub std::string::String ); // NOTE:  Not really UTF8, actually some kind of hybrid monstrosity between UTF8 and UTF16.
/// [Java SE 7 &sect; 4.4.8](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.8):  A CONSTANT_MethodHandle_info, minus the tag.
#[derive(Clone, Debug)] pub struct MethodHandle         { pub reference_kind: u8, pub reference_index: u16 }
/// [Java SE 7 &sect; 4.4.9](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.9):  A CONSTANT_MethodType_info, minus the tag.
#[derive(Clone, Debug)] pub struct MethodType           { pub descriptor_index: u16 }
/// [Java SE 7 &sect; 4.4.10](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.10):  A CONSTANT_InvokeDynamic_info, minus the tag.
#[derive(Clone, Debug)] pub struct InvokeDynamic        { pub bootstrap_method_attr_index: u16, pub name_and_type_index: u16 }

impl Class                      { pub const TAG : Type = Type( 7); pub const ENTRIES : u8 = 1; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self{ name_index: read_u2(r)? })} }
impl Fieldref                   { pub const TAG : Type = Type( 9); pub const ENTRIES : u8 = 1; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self{ class_index: read_u2(r)?, name_and_type_index: read_u2(r)? })} }
impl Methodref                  { pub const TAG : Type = Type(10); pub const ENTRIES : u8 = 1; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self{ class_index: read_u2(r)?, name_and_type_index: read_u2(r)? })} }
impl InterfaceMethodref         { pub const TAG : Type = Type(11); pub const ENTRIES : u8 = 1; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self{ class_index: read_u2(r)?, name_and_type_index: read_u2(r)? })} }
impl String                     { pub const TAG : Type = Type( 8); pub const ENTRIES : u8 = 1; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self{ string_index: read_u2(r)? })} }
impl Integer                    { pub const TAG : Type = Type( 3); pub const ENTRIES : u8 = 1; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self( read_i4(r)? ))} }
impl Float                      { pub const TAG : Type = Type( 4); pub const ENTRIES : u8 = 1; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self( f32::from_bits(read_u4(r)?) ))} }
impl Long                       { pub const TAG : Type = Type( 5); pub const ENTRIES : u8 = 2; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self( read_i8(r)? ))} }
impl Double                     { pub const TAG : Type = Type( 6); pub const ENTRIES : u8 = 2; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self( f64::from_bits(read_u8(r)?) ))} }
impl NameAndType                { pub const TAG : Type = Type(12); pub const ENTRIES : u8 = 2; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self{ name_index: read_u2(r)?, descriptor_index: read_u2(r)? })} }
impl Utf8                       { pub const TAG : Type = Type( 1); pub const ENTRIES : u8 = 1; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> { Self::read_after_tag_impl(r) } }
impl MethodHandle               { pub const TAG : Type = Type(15); pub const ENTRIES : u8 = 1; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self{ reference_kind: read_u1(r)?, reference_index: read_u2(r)? })} }
impl MethodType                 { pub const TAG : Type = Type(16); pub const ENTRIES : u8 = 1; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self{ descriptor_index: read_u2(r)? })} }
impl InvokeDynamic              { pub const TAG : Type = Type(18); pub const ENTRIES : u8 = 1; pub fn read_after_tag(r: &mut impl Read) -> io::Result<Self> {Ok(Self{ bootstrap_method_attr_index: read_u2(r)?, name_and_type_index: read_u2(r)? })} }

// NOTE:  Not really UTF8, actually some kind of hybrid monstrosity between UTF8 and UTF16.
impl Utf8 {
    fn expect_byte(remaining: &[u8], index: usize, mask: u8, equal: u8) -> io::Result<u8> {
        if let Some(&value) = remaining.get(index) {
            if value & mask == equal {
                return Ok(value & !mask);
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid 'UTF8' string - expected index {} byte {:b} & mask {:b} == {:b}", index, value, mask, equal)))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Incomplete 'UTF8' string - expected more bytes"))
        }
    }

    fn read_char(remaining: &mut &[u8]) -> io::Result<char> {
        if let Some(&b0) = remaining.get(0) {
            if b0 & 0b10000000 == 0b00000000 {
                // Standard 1-byte UTF8: 0b0xxxxxxx = 1 byte 0x00..=7F
                *remaining = &remaining[1..];
                Ok(b0 as char)

            } else if b0 & 0b11100000 == 0b11000000 {
                // Standard 2-byte UTF8:0b110xxxxx 0b10yyyyyy = 2 bytes 0x80..=7FF
                let b0 = (b0 & 0b00011111) as u32;
                let b1 = Self::expect_byte(*remaining, 1, 0b11000000, 0b10000000)? as u32;
                *remaining = &remaining[2..];
                Ok(char::try_from(b0 << 6 | b1 << 0).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Expected 'UTF8' bytes"))?)

            } else if b0 == 0b11011101 {
                // STRANGE 6-byte UTF8/UTF16 nonsense: 0b11101101 0b1010aaaa 0b10bbbbbb 0b11101101 0b1011cccc 0b10dddddd
                let a = Self::expect_byte(*remaining, 1, 0b11110000, 0b10100000)? as u32; // ....aaaa
                let b = Self::expect_byte(*remaining, 2, 0b11000000, 0b10000000)? as u32; // ..bbbbbb
                let _ = Self::expect_byte(*remaining, 3, 0b11111111, 0b11101101)? as u32; // ........
                let c = Self::expect_byte(*remaining, 4, 0b11110000, 0b10110000)? as u32; // ....cccc
                let d = Self::expect_byte(*remaining, 5, 0b11000000, 0b10000000)? as u32; // ..dddddd
                *remaining = &remaining[6..];
                Ok(char::try_from(a << 16 | b << 10 | c << 6 | d << 0).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Expected 'UTF8' bytes"))?)

            } else if b0 & 0b11110000 == 0b11100000 {
                // Standard 3-byte UTF8: 0b1110xxxx 0b10yyyyyy 0b10zzzzzz = 3 bytes 0x800..=FFFF
                let b0 = (b0 & 0b00001111) as u32;
                let b1 = Self::expect_byte(*remaining, 1, 0b11000000, 0b10000000)? as u32;
                let b2 = Self::expect_byte(*remaining, 2, 0b11000000, 0b10000000)? as u32;
                *remaining = &remaining[3..];
                Ok(char::try_from(b0 << 12 | b1 << 6 | b2 << 0).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Expected 'UTF8' bytes"))?)

            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "Expected 'UTF8' bytes, invalid starting byte"))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Expected 'UTF8' bytes"))
        }
    }

    fn read_after_tag_impl(r: &mut impl Read) -> io::Result<Self> {
        let bytes = read_u2(r)? as usize;
        let mut buffer = Vec::new();
        buffer.resize(bytes, 0u8);
        r.read_exact(&mut buffer[..])?;
        let mut remaining = &buffer[..];
        let mut output = std::string::String::new();
        while !remaining.is_empty() {
            output.push(Self::read_char(&mut remaining)?);
        }
        Ok(Self(output))
    }
}



/// [Java SE 7 &sect; 4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.1):  A CONSTANT_* value.  Not ABI compatible with the raw C ABIs but that's fine.
#[derive(Clone, Debug)]
pub enum Constant {
    /// The constants table (and *only* the constants table) is 1-indexed.  That's just confusing.  Even worse, `Long` and `Double` take up two slots.  So I emit this as a placeholder for those slots.
    UnusedPlaceholder(UnusedPlaceholder),
    /// [Java SE 7 &sect; 4.4.1](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.1):  A CONSTANT_Class_info, minus the tag.
    Class(Class),
    /// [Java SE 7 &sect; 4.4.2](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.2):  A CONSTANT_Fieldref_info, minus the tag.
    Fieldref(Fieldref),
    /// [Java SE 7 &sect; 4.4.2](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.2):  A CONSTANT_Methodref_info, minus the tag.
    Methodref(Methodref),
    /// [Java SE 7 &sect; 4.4.2](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.2):  A CONSTANT_InstanceMethodref_info, minus the tag.
    InterfaceMethodref(InterfaceMethodref),
    /// [Java SE 7 &sect; 4.4.3](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.3):  A CONSTANT_String_info, minus the tag.
    String(String),
    /// [Java SE 7 &sect; 4.4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.4):  A CONSTANT_Integer_info, minus the tag.
    Integer(Integer),
    /// [Java SE 7 &sect; 4.4.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.4):  A CONSTANT_Float_info, minus the tag.
    Float(Float),
    /// [Java SE 7 &sect; 4.4.5](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.5):  A CONSTANT_Long_info, minus the tag.
    Long(Long),
    /// [Java SE 7 &sect; 4.4.5](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.5):  A CONSTANT_Double_info, minus the tag.
    Double(Double),
    /// [Java SE 7 &sect; 4.4.6](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.6):  A CONSTANT_NameAndType_info, minus the tag.
    NameAndType(NameAndType),
    /// [Java SE 7 &sect; 4.4.7](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.7):  A CONSTANT_Utf8_info, minus the tag.
    Utf8(Utf8),
    /// [Java SE 7 &sect; 4.4.8](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.8):  A CONSTANT_MethodHandle_info, minus the tag.
    MethodHandle(MethodHandle),
    /// [Java SE 7 &sect; 4.4.9](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.9):  A CONSTANT_MethodType_info, minus the tag.
    MethodType(MethodType),
    /// [Java SE 7 &sect; 4.4.10](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.10):  A CONSTANT_InvokeDynamic_info, minus the tag.
    InvokeDynamic(InvokeDynamic),
}

impl From<UnusedPlaceholder>    for Constant { fn from(value: UnusedPlaceholder     ) -> Self { Constant::UnusedPlaceholder(value) } }
impl From<Class>                for Constant { fn from(value: Class                 ) -> Self { Constant::Class(value) } }
impl From<Fieldref>             for Constant { fn from(value: Fieldref              ) -> Self { Constant::Fieldref(value) } }
impl From<Methodref>            for Constant { fn from(value: Methodref             ) -> Self { Constant::Methodref(value) } }
impl From<InterfaceMethodref>   for Constant { fn from(value: InterfaceMethodref    ) -> Self { Constant::InterfaceMethodref(value) } }
impl From<String>               for Constant { fn from(value: String                ) -> Self { Constant::String(value) } }
impl From<Integer>              for Constant { fn from(value: Integer               ) -> Self { Constant::Integer(value) } }
impl From<Float>                for Constant { fn from(value: Float                 ) -> Self { Constant::Float(value) } }
impl From<Long>                 for Constant { fn from(value: Long                  ) -> Self { Constant::Long(value) } }
impl From<Double>               for Constant { fn from(value: Double                ) -> Self { Constant::Double(value) } }
impl From<NameAndType>          for Constant { fn from(value: NameAndType           ) -> Self { Constant::NameAndType(value) } }
impl From<Utf8>                 for Constant { fn from(value: Utf8                  ) -> Self { Constant::Utf8(value) } }
impl From<MethodHandle>         for Constant { fn from(value: MethodHandle          ) -> Self { Constant::MethodHandle(value) } }
impl From<MethodType>           for Constant { fn from(value: MethodType            ) -> Self { Constant::MethodType(value) } }
impl From<InvokeDynamic>        for Constant { fn from(value: InvokeDynamic         ) -> Self { Constant::InvokeDynamic(value) } }
