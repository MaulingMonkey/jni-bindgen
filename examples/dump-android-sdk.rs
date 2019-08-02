use bugsalot::*;

use bindgen_jni::class_file_visitor::{self, *};

use std::env;
use std::fs::{File};
use std::io::Read;
use std::path::{PathBuf};

struct DisplayConstants;

impl constant::Visitor for DisplayConstants {
    fn on_unused                    (&mut self, index: u16, value: constant::UnusedPlaceholder   ) { println!("  const {:3} = {:?}", index, &value); }
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

impl class_file_visitor::FieldVisitor for DisplayConstants {
    fn on_field(&mut self, index: u16, field: Field) { println!("  field {:3} = {:?}", index, field); }
    fn on_field_attribute(&mut self, _field_index: u16, attribute_index: u16, attribute: Attribute) { println!("    attribute {:3} = {:?}", attribute_index, &attribute); }
}

impl class_file_visitor::MethodVisitor for DisplayConstants {
    fn on_method(&mut self, index: u16, method: Method) { println!("  method {:3} = {:?}", index, method); }
    fn on_method_attribute(&mut self, _method_index: u16, attribute_index: u16, attribute: Attribute) { println!("    attribute {:3} = {:?}", attribute_index, &attribute); }
}

impl class_file_visitor::Visitor for DisplayConstants {
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



struct Noop;

impl constant::Visitor for Noop {
    fn on_unused                    (&mut self, _index: u16, _value: constant::UnusedPlaceholder   ) {}
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

impl class_file_visitor::FieldVisitor for Noop {
    fn on_field(&mut self, _index: u16, _field: Field) {}
    fn on_field_attribute(&mut self, _field_index: u16, _attribute_index: u16, _attribute: Attribute) {}
}

impl class_file_visitor::MethodVisitor for Noop {
    fn on_method(&mut self, _index: u16, _method: Method) {}
    fn on_method_attribute(&mut self, _method_index: u16, _attribute_index: u16, _attribute: Attribute) {}
}

impl class_file_visitor::Visitor for Noop {
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



fn main() {
    std::panic::set_hook(Box::new(|panic|{ bug!("{:?}", panic); }));

    let local_app_data = env::var("LOCALAPPDATA").unwrap();
    let android_jar : PathBuf = [local_app_data.as_str(), "Android/Sdk/platforms/android-28/android.jar"].iter().collect();
    let mut android_jar = File::open(&android_jar).unwrap();
    let mut android_jar = zip::ZipArchive::new(&mut android_jar).unwrap();

    for i in 0..android_jar.len() {
        let mut file = android_jar.by_index(i).unwrap();
        if !file.name().ends_with(".class") { continue; }
        println!("{}", file.name());

        if i < 5 || file.name().ends_with("/Patterns.class") || file.name().ends_with("/JsonWriter.class") {
            class_file_visitor::read(&mut file, &mut DisplayConstants).unwrap();
            println!();
            println!();
            println!();
        } else {
            class_file_visitor::read(&mut file, &mut Noop).unwrap();
        }
        let mut more = [0u8; 1];
        let _ = file.read_exact(&mut more[..]).unwrap_err();
    }
}
