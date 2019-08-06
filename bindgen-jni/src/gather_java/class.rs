use super::*;

#[derive(Clone, Debug, Default)]
pub struct Class {
    access_flags:       ClassAccessFlags,
    constants:          ClassConstants,
    this_class:         u16,
    super_class:        u16,
    interfaces:         Vec<u16>,
    fields:             Vec<Field>,
    methods:            Vec<Method>,
    inner_classes:      BTreeMap<String, Class>,
}

impl Class {
    pub fn new() -> Self { Default::default() }

    pub fn try_read(read: &mut impl io::Read) -> io::Result<Self> {
        let mut c = Class::new();
        class_file_visitor::read(read, &mut c)?;
        Ok(c)
    }

    pub fn try_read_all(mut read: impl io::Read) -> io::Result<Self> {
        let class = Self::try_read(&mut read)?;
        let mut last = [0u8; 1];
        match read.read_exact(&mut last) {
            Ok(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Expected EOF, class file continues!")),
            Err(_) => Ok(class),
        }
    }

    pub fn access_flags(&self) -> ClassAccessFlags { self.access_flags }
    pub fn this_class(&self)  -> class_constants::ClassRef { self.constants.class(self.this_class) }
    pub fn super_class(&self) -> Option<class_constants::ClassRef> { if self.super_class == 0 { None } else { Some(self.constants.class(self.super_class)) } }
    pub fn interfaces(&self) -> impl Iterator<Item = class_constants::ClassRef> { self.interfaces.iter().map(move |i| self.constants.class(*i)) }
    pub fn methods(&self) -> impl Iterator<Item = MethodRef> { self.methods.iter().map(move |method| MethodRef { constants: &self.constants, method }) }
    pub fn fields(&self) -> impl Iterator<Item = FieldRef> { self.fields.iter().map(move |field| FieldRef { constants: &self.constants, field }) }

    pub fn clear(&mut self) {
        self.access_flags = ClassAccessFlags::default();
        self.constants.0.clear();
        self.this_class = 0;
        self.super_class = 0;
        self.interfaces.clear();
        self.fields.clear();
        self.methods.clear();
    }
}

impl constant::Visitor for Class {
    fn on_unused            (&mut self, index: u16, value: constant::UnusedPlaceholder      ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_class             (&mut self, index: u16, value: constant::Class                  ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_field             (&mut self, index: u16, value: constant::Fieldref               ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_method            (&mut self, index: u16, value: constant::Methodref              ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_interface_method  (&mut self, index: u16, value: constant::InterfaceMethodref     ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_string            (&mut self, index: u16, value: constant::String                 ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_integer           (&mut self, index: u16, value: constant::Integer                ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_float             (&mut self, index: u16, value: constant::Float                  ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_long              (&mut self, index: u16, value: constant::Long                   ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_double            (&mut self, index: u16, value: constant::Double                 ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_name_and_tag      (&mut self, index: u16, value: constant::NameAndType            ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_utf8              (&mut self, index: u16, value: constant::Utf8                   ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_method_handle     (&mut self, index: u16, value: constant::MethodHandle           ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_method_type       (&mut self, index: u16, value: constant::MethodType             ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
    fn on_invoke_dynamic    (&mut self, index: u16, value: constant::InvokeDynamic          ) { assert_eq!(index as usize, self.constants.0.len()); self.constants.0.push(value.into()); }
}

impl FieldVisitor for Class {
    fn on_field(&mut self, _index: u16, field: Field) { self.fields.push(field); }
    fn on_field_attribute(&mut self, _field_index: u16, _attribute_index: u16, _attribute: Attribute) {}
}

impl MethodVisitor for Class {
    fn on_method(&mut self, _index: u16, method: Method) { self.methods.push(method); }
    fn on_method_attribute(&mut self, _method_index: u16, _attribute_index: u16, _attribute: Attribute) {}
}

impl Visitor for Class {
    fn on_header(&mut self, _header: Header) {}

    // constant::Visitor

    fn on_class_access_flags(&mut self, class_access_flags: ClassAccessFlags) {
        self.access_flags = class_access_flags;
    }

    fn on_this_class(&mut self, this_class: u16) {
        self.this_class = this_class;
    }

    fn on_super_class(&mut self, super_class: u16) {
        self.super_class = super_class;

    }

    fn on_interface(&mut self, interface: u16) {
        self.interfaces.push(interface);
    }

    // field::Visitor
    // method::Visitor

    fn on_class_attribute(&mut self, _attribute_index: u16, _class_attribute: Attribute) {}
}
