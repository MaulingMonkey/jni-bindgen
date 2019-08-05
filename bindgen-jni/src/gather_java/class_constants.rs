use super::*;

mod class_ref;
mod field_ref;
mod interface_method_ref;
mod name_and_type_ref;

pub use class_ref::*;
pub use field_ref::*;
pub use interface_method_ref::*;
pub(crate) use name_and_type_ref::*;

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

    // MethodRef -> MethodRefRef?
    //pub fn method(&self, index: u16) -> MethodRef {
    //    let index = index as usize;
    //    if index == 0 || index >= self.0.len() {
    //        panic!("Constant {} is not a MethodRef: Out of bounds", index);
    //    }
    //
    //    let instance = &self.0[index];
    //    if let constant::Constant::Methodref(ref method) = instance {
    //        MethodRef { constants: &self, method }
    //    } else {
    //        panic!("Constant {} is not a MethodRef: {:?}", index, instance);
    //    }
    //}

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
}
