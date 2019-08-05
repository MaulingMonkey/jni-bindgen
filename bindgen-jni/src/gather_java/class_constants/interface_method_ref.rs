use super::*;



#[derive(Clone, Copy)]
pub struct InterfaceMethodRef<'a> {
    pub(crate) constants:  &'a ClassConstants,
    pub(crate) method:     &'a constant::InterfaceMethodref,
}

impl<'a> InterfaceMethodRef<'a> {
    pub fn class(&self) -> ClassRef<'a> { self.constants.class(self.method.class_index) }
    pub fn name(&self) -> &'a String { self.constants.name_and_type(self.method.name_and_type_index).name() }
    pub fn descriptor(&self) -> &'a String { self.constants.name_and_type(self.method.name_and_type_index).descriptor() }
}
