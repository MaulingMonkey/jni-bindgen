use super::*;



#[derive(Clone, Copy)]
pub(crate) struct NameAndTypeRef<'a> {
    pub(crate) constants:      &'a ClassConstants,
    pub(crate) name_and_type:  &'a constant::NameAndType,
}

impl <'a> NameAndTypeRef<'a> {
    pub fn name(&self)       -> &'a String { self.constants.utf8(self.name_and_type.name_index) }
    pub fn descriptor(&self) -> &'a String { self.constants.utf8(self.name_and_type.descriptor_index) }
}
