use super::*;



#[derive(Clone, Copy)]
pub struct ClassRef<'a> {
    pub(crate) constants:  &'a ClassConstants,
    pub(crate) class:      &'a constant::Class,
}

impl<'a> ClassRef<'a> {
    pub fn name(&self) -> &'a String { self.constants.utf8(self.class.name_index) }
}
