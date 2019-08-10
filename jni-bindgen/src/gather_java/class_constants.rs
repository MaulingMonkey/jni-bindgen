use super::*;
use bugsalot::*;
use class_file_visitor::constant::Constant;

mod class_ref;
mod field_ref;
mod method_ref;
mod interface_method_ref;
mod name_and_type_ref;

pub use class_ref::ClassRef;
pub use field_ref::FieldRef;
pub use method_ref::MethodRef;
pub use interface_method_ref::InterfaceMethodRef;
pub(crate) use name_and_type_ref::NameAndTypeRef;

#[derive(Clone, Debug, Default)]
pub(crate) struct ClassConstants(pub(crate) Vec<constant::Constant>);

impl ClassConstants {
    pub fn class(&self, index: u16) -> ClassRef {
        let index = index as usize;
        if index == 0 || index >= self.0.len() {
            panic!("Constant {} is not a Class: Out of bounds", index);
        }

        let instance = &self.0[index];
        if let constant::Constant::Class(ref class) = instance {
            ClassRef { constants: &self, class }
        } else {
            panic!("Constant {} is not a Class: {:?}", index, instance);
        }
    }

    pub fn field(&self, index: u16) -> FieldRef {
        let index = index as usize;
        if index == 0 || index >= self.0.len() {
            panic!("Constant {} is not a FieldRef: Out of bounds", index);
        }

        let instance = &self.0[index];
        if let constant::Constant::Fieldref(ref field) = instance {
            FieldRef { constants: &self, field }
        } else {
            panic!("Constant {} is not a FieldRef: {:?}", index, instance);
        }
    }

    pub fn method(&self, index: u16) -> MethodRef {
        let index = index as usize;
        if index == 0 || index >= self.0.len() {
            panic!("Constant {} is not a MethodRef: Out of bounds", index);
        }
    
        let instance = &self.0[index];
        if let constant::Constant::Methodref(ref method) = instance {
            MethodRef { constants: &self, method }
        } else {
            panic!("Constant {} is not a MethodRef: {:?}", index, instance);
        }
    }

    pub fn interface_method(&self, index: u16) -> InterfaceMethodRef {
        let index = index as usize;
        if index == 0 || index >= self.0.len() {
            panic!("Constant {} is not a InterfaceMethodRef: Out of bounds", index);
        }

        let instance = &self.0[index];
        if let constant::Constant::InterfaceMethodref(ref method) = instance {
            InterfaceMethodRef { constants: &self, method }
        } else {
            panic!("Constant {} is not a InterfaceMethodRef: {:?}", index, instance);
        }
    }

    pub(crate) fn name_and_type(&self, index: u16) -> NameAndTypeRef {
        let index = index as usize;
        if index == 0 || index >= self.0.len() {
            panic!("Constant {} is not a NameAndType: Out of bounds", index);
        }

        let instance = &self.0[index];
        if let constant::Constant::NameAndType(ref name_and_type) = instance {
            NameAndTypeRef { constants: &self, name_and_type }
        } else {
            panic!("Constant {} is not a NameAndType: {:?}", index, instance);
        }
    }

    pub(crate) fn utf8(&self, index: u16) -> &String {
        let index = index as usize;
        if index == 0 || index >= self.0.len() {
            panic!("Constant {} is not a Utf8: Out of bounds", index);
        }

        let instance = &self.0[index];
        if let constant::Constant::Utf8(ref utf8) = instance {
            &utf8.0
        } else {
            panic!("Constant {} is not a Utf8: {:?}", index, instance);
        }
    }

    pub(crate) fn try_utf8(&self, index: u16) -> Option<&String> {
        let index = index as usize;
        if index == 0 || index >= self.0.len() {
            return None;
        }

        let instance = &self.0[index];
        if let constant::Constant::Utf8(ref utf8) = instance {
            Some(&utf8.0)
        } else {
            None
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub(crate) enum KnownAttribute<'a> {
    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
    ConstantValue_Long(i64),
    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
    ConstantValue_Float(f32),
    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
    ConstantValue_Double(f64),
    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
    ConstantValue_Integer(i32), // int, short, char, byte, boolean
    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
    ConstantValue_String(&'a String),

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.3
    Code {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.4
    StackMapTable {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.5
    Exceptions {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.6
    InnerClasses {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.7
    EnclosingMethod {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.8
    Synthetic {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.9
    Signature {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.10
    SourceFile {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.11
    SourceDebugExtension {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.12
    LineNumberTable {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.13
    LocalVariableTable {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.14
    LocalVariableTypeTable {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.15
    Deprecated {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.16
    RuntimeVisibleAnnotations {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.17
    RuntimeInvisibleAnnotations {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.18
    RuntimeVisibleParameterAnnotations {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.19
    RuntimeInvisibleParameterAnnotations {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.20
    AnnotationDefault {},

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.21
    BootstrapMethods {},

    #[doc(hidden)] __NonExhaustive,
}

impl<'a> KnownAttribute<'a> {
    pub(crate) fn from(constants: &'a ClassConstants, attribute: &Attribute) -> std::result::Result<Self, ()> {
        let name = constants.try_utf8(attribute.attribute_name_index).ok_or(())?.as_str();

        match name {
            "ConstantValue" => {
                // https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
                unwrap!(attribute.attribute_length == 2, return Err(()));
                unwrap!(attribute.info.len() == 2, return Err(()));
                let constantvalue_index = u16::from_be_bytes([attribute.info[0], attribute.info[1]]) as usize;
                unwrap!(constantvalue_index < constants.0.len(), return Err(()));
                match &constants.0[constantvalue_index] {
                    Constant::Long(ref c)       => { Ok(KnownAttribute::ConstantValue_Long(c.0)) },
                    Constant::Float(ref c)      => { Ok(KnownAttribute::ConstantValue_Float(c.0)) },
                    Constant::Double(ref c)     => { Ok(KnownAttribute::ConstantValue_Double(c.0)) },
                    Constant::Integer(ref c)    => { Ok(KnownAttribute::ConstantValue_Integer(c.0)) },
                    Constant::String(ref c)     => {
                        let s = constants.try_utf8(c.string_index).ok_or(())?;
                        Ok(KnownAttribute::ConstantValue_String(s))
                    },
                    c => {
                        bug!("Expected Constant::{{Long, Float, Double, Integer, String}}, got {:?}", c);
                        Err(())
                    },
                }
            },
            "Code"                                  => { Ok(KnownAttribute::Code {}) },
            "StackMapTable"                         => { Ok(KnownAttribute::StackMapTable {}) },
            "Exceptions"                            => { Ok(KnownAttribute::Exceptions {}) },
            "InnerClasses"                          => { Ok(KnownAttribute::InnerClasses {}) },
            "EnclosingMethod"                       => { Ok(KnownAttribute::EnclosingMethod {}) },
            "Synthetic"                             => { Ok(KnownAttribute::Synthetic {}) },
            "Signature"                             => { Ok(KnownAttribute::Signature {}) },
            "SourceFile"                            => { Ok(KnownAttribute::SourceFile {}) },
            "SourceDebugExtension"                  => { Ok(KnownAttribute::SourceDebugExtension {}) },
            "LineNumberTable"                       => { Ok(KnownAttribute::LineNumberTable {}) },
            "LocalVariableTable"                    => { Ok(KnownAttribute::LocalVariableTable {}) },
            "LocalVariableTypeTable"                => { Ok(KnownAttribute::LocalVariableTypeTable {}) },
            "Deprecated"                            => { Ok(KnownAttribute::Deprecated {}) },
            "RuntimeVisibleAnnotations"             => { Ok(KnownAttribute::RuntimeVisibleAnnotations {}) },
            "RuntimeInvisibleAnnotations"           => { Ok(KnownAttribute::RuntimeInvisibleAnnotations {}) },
            "RuntimeVisibleParameterAnnotations"    => { Ok(KnownAttribute::RuntimeVisibleParameterAnnotations {}) },
            "RuntimeInvisibleParameterAnnotations"  => { Ok(KnownAttribute::RuntimeInvisibleParameterAnnotations {}) },
            "AnnotationDefault"                     => { Ok(KnownAttribute::AnnotationDefault {}) },
            "BootstrapMethods"                      => { Ok(KnownAttribute::BootstrapMethods {}) },
            _ => Err(()),
        }
    }
}
