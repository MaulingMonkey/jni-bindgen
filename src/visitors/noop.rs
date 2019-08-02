use crate::class_file_visitor::*;

struct Noop;

impl constant::Visitor for Noop {
    fn on_unused                    (&mut self, _index: u16, _value: constant::UnusedPlaceholder      ) {}
    fn on_class                     (&mut self, _index: u16, _value: constant::Class                  ) {}
    fn on_field                     (&mut self, _index: u16, _value: constant::Fieldref               ) {}
    fn on_method                    (&mut self, _index: u16, _value: constant::Methodref              ) {}
    fn on_interface_method          (&mut self, _index: u16, _value: constant::InterfaceMethodref     ) {}
    fn on_string                    (&mut self, _index: u16, _value: constant::String                 ) {}
    fn on_integer                   (&mut self, _index: u16, _value: constant::Integer                ) {}
    fn on_float                     (&mut self, _index: u16, _value: constant::Float                  ) {}
    fn on_long                      (&mut self, _index: u16, _value: constant::Long                   ) {}
    fn on_double                    (&mut self, _index: u16, _value: constant::Double                 ) {}
    fn on_name_and_tag              (&mut self, _index: u16, _value: constant::NameAndType            ) {}
    fn on_utf8                      (&mut self, _index: u16, _value: constant::Utf8                   ) {}
    fn on_method_handle             (&mut self, _index: u16, _value: constant::MethodHandle           ) {}
    fn on_method_type               (&mut self, _index: u16, _value: constant::MethodType             ) {}
    fn on_invoke_dynamic            (&mut self, _index: u16, _value: constant::InvokeDynamic          ) {}
}

impl FieldVisitor for Noop {
    fn on_field(&mut self, _index: u16, _field: Field) {}
    fn on_field_attribute(&mut self, _field_index: u16, _attribute_index: u16, _attribute: Attribute) {}
}

impl MethodVisitor for Noop {
    fn on_method(&mut self, _index: u16, _method: Method) {}
    fn on_method_attribute(&mut self, _method_index: u16, _attribute_index: u16, _attribute: Attribute) {}
}

impl Visitor for Noop {
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
