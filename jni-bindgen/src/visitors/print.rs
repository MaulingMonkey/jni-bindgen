use crate::class_file_visitor::*;

struct Print;

impl constant::Visitor for Print {
    fn on_unused                    (&mut self, index: u16, value: constant::UnusedPlaceholder      ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_class                     (&mut self, index: u16, value: constant::Class                  ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_field                     (&mut self, index: u16, value: constant::Fieldref               ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_method                    (&mut self, index: u16, value: constant::Methodref              ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_interface_method          (&mut self, index: u16, value: constant::InterfaceMethodref     ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_string                    (&mut self, index: u16, value: constant::String                 ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_integer                   (&mut self, index: u16, value: constant::Integer                ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_float                     (&mut self, index: u16, value: constant::Float                  ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_long                      (&mut self, index: u16, value: constant::Long                   ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_double                    (&mut self, index: u16, value: constant::Double                 ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_name_and_tag              (&mut self, index: u16, value: constant::NameAndType            ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_utf8                      (&mut self, index: u16, value: constant::Utf8                   ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_method_handle             (&mut self, index: u16, value: constant::MethodHandle           ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_method_type               (&mut self, index: u16, value: constant::MethodType             ) { println!("  const {:3} = {:?}", index, &value); }
    fn on_invoke_dynamic            (&mut self, index: u16, value: constant::InvokeDynamic          ) { println!("  const {:3} = {:?}", index, &value); }
}

impl FieldVisitor for Print {
    fn on_field(&mut self, index: u16, field: Field) { println!("  field {:3} = {:?}", index, field); }
    fn on_field_attribute(&mut self, _field_index: u16, attribute_index: u16, attribute: Attribute) { println!("    attribute {:3} = {:?}", attribute_index, &attribute); }
}

impl MethodVisitor for Print {
    fn on_method(&mut self, index: u16, method: Method) { println!("  method {:3} = {:?}", index, method); }
    fn on_method_attribute(&mut self, _method_index: u16, attribute_index: u16, attribute: Attribute) { println!("    attribute {:3} = {:?}", attribute_index, &attribute); }
}

impl Visitor for Print {
    fn on_header(&mut self, header: Header) {
        println!("  version: {}.{}", header.major_version, header.minor_version);
    }

    // constant::Visitor

    fn on_class_access_flags(&mut self, class_access_flags: ClassAccessFlags) {
        println!("  access_flags: {:?}", class_access_flags);
    }

    fn on_this_class(&mut self, this_class: u16) {
        println!("  this_class:   {}", this_class);
    }

    fn on_super_class(&mut self, super_class: u16) {
        println!("  super_class:  {}", super_class);

    }

    fn on_interface(&mut self, interface: u16) {
        println!("  interface: {}", interface);
    }

    // field::Visitor
    // method::Visitor

    fn on_class_attribute(&mut self, attribute_index: u16, class_attribute: Attribute) {
        println!("  class attribute {:3} = {:?}", attribute_index, &class_attribute);
    }
}
