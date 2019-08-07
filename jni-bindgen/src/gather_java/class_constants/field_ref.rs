use super::*;



#[derive(Clone, Copy)]
pub struct FieldRef<'a> {
    pub(crate) constants:  &'a ClassConstants,
    pub(crate) field:      &'a constant::Fieldref,
}

impl<'a> FieldRef<'a> {
    pub fn class(&self) -> ClassRef<'a> { self.constants.class(self.field.class_index) }
    pub fn name(&self) -> &'a String { self.constants.name_and_type(self.field.name_and_type_index).name() }
    pub fn descriptor(&self) -> &'a String { self.constants.name_and_type(self.field.name_and_type_index).descriptor() }
}
