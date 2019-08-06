use super::*;
use class_file_visitor::field::*;



#[derive(Clone, Copy)]
pub struct FieldRef<'a> {
    pub(crate) constants:  &'a ClassConstants,
    pub(crate) field:      &'a Field,
}

impl<'a> FieldRef<'a> {
    pub fn access_flags(&self) -> FieldAccessFlags { self.field.access_flags }
    pub fn name(&self) -> &'a String { self.constants.utf8(self.field.name_index) }
    pub fn descriptor(&self) -> &'a String { self.constants.utf8(self.field.descriptor_index) }
    // Attributes?
}
