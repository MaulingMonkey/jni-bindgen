use super::*;



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
}



#[derive(Clone, Copy)]
pub struct ClassRef<'a> {
    constants:  &'a ClassConstants,
    class:      &'a constant::Class,
}

impl<'a> ClassRef<'a> {
    pub fn name(&self) -> &'a String { self.constants.utf8(self.class.name_index) }
}



#[derive(Clone, Copy)]
pub struct FieldRef<'a> {
    constants:  &'a ClassConstants,
    field:      &'a constant::Fieldref,
}

impl<'a> FieldRef<'a> {
    pub fn class(&self) -> ClassRef<'a> { self.constants.class(self.field.class_index) }
    pub fn name(&self) -> &'a String { self.constants.name_and_type(self.field.name_and_type_index).name() }
    pub fn descriptor(&self) -> &'a String { self.constants.name_and_type(self.field.name_and_type_index).descriptor() }
}



#[derive(Clone, Copy)]
pub struct MethodRef<'a> {
    constants:  &'a ClassConstants,
    method:     &'a constant::Methodref,
}

impl<'a> MethodRef<'a> {
    pub fn class(&self) -> ClassRef<'a> { self.constants.class(self.method.class_index) }
    pub fn name(&self) -> &'a String { self.constants.name_and_type(self.method.name_and_type_index).name() }
    pub fn descriptor(&self) -> &'a String { self.constants.name_and_type(self.method.name_and_type_index).descriptor() }
}



#[derive(Clone, Copy)]
pub struct InterfaceMethodRef<'a> {
    constants:  &'a ClassConstants,
    method:     &'a constant::InterfaceMethodref,
}

impl<'a> InterfaceMethodRef<'a> {
    pub fn class(&self) -> ClassRef<'a> { self.constants.class(self.method.class_index) }
    pub fn name(&self) -> &'a String { self.constants.name_and_type(self.method.name_and_type_index).name() }
    pub fn descriptor(&self) -> &'a String { self.constants.name_and_type(self.method.name_and_type_index).descriptor() }
}


#[derive(Clone, Copy)]
pub(crate) struct NameAndTypeRef<'a> {
    constants:      &'a ClassConstants,
    name_and_type:  &'a constant::NameAndType,
}

impl <'a> NameAndTypeRef<'a> {
    pub fn name(&self)       -> &'a String { self.constants.utf8(self.name_and_type.name_index) }
    pub fn descriptor(&self) -> &'a String { self.constants.utf8(self.name_and_type.descriptor_index) }
}
