use super::*;
use class_file_visitor::method::*;



#[derive(Clone, Copy)]
pub struct MethodRef<'a> {
    pub(crate) constants:  &'a ClassConstants,
    pub(crate) method:     &'a Method,
}

impl<'a> MethodRef<'a> {
    pub fn access_flags(&self) -> MethodAccessFlags { self.method.access_flags }
    pub fn name(&self) -> &'a String { self.constants.utf8(self.method.name_index) }
    pub fn descriptor(&self) -> &'a String { self.constants.utf8(self.method.descriptor_index) }
    // Attributes?
}
