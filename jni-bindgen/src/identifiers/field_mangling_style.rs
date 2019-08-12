use crate::java::*;
use serde_derive::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct FieldManglingStyle {
    pub const_finals:   bool,   // Default: true
    pub rustify_names:  bool,   // Default: true
    pub getter_pattern: String, // Default: "{NAME}", might consider "get_{NAME}"
    pub setter_pattern: String, // Default: "set_{NAME}"
}

impl Default for FieldManglingStyle {
    fn default() -> Self {
        Self {
            const_finals: true,
            rustify_names: true,
            getter_pattern: String::from("{NAME}"),
            setter_pattern: String::from("set_{NAME}"),
        }
    }
}

// TODO: tests

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FieldManglingError {
    NotYetImplemented, // XXX: Remove
    EmptyString,
    NotRustSafe,
    UnexpectedCharacter(char),
}

impl std::error::Error for FieldManglingError {}
impl std::fmt::Display for FieldManglingError { fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result { std::fmt::Debug::fmt(self, fmt) } }

impl FieldManglingStyle {
    pub fn mangle(&self, flags: field::Flags, name: &str, signature: &str) -> Result<String, FieldManglingError> {
        Err(FieldManglingError::NotYetImplemented)
    }
}
