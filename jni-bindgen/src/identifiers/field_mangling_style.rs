use super::*;
use crate::java::*;

use serde_derive::*;



pub enum FieldMangling<'a> {
    ConstValue(String, &'a field::Constant),
    GetSet(String, String),
}

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

impl FieldManglingStyle {
    pub fn mangle<'a>(&self, field: &'a Field) -> Result<FieldMangling<'a>, IdentifierManglingError> {
        if let (Some(value), true) = (field.constant.as_ref(), self.const_finals) {
            let name = if self.rustify_names {
                constify_identifier(field.name.as_str())
            } else {
                javaify_identifier(field.name.as_str())
            }?;

            Ok(FieldMangling::ConstValue(name, value))
        } else {
            Ok(FieldMangling::GetSet(
                self.mangle_identifier(self.getter_pattern.replace("{NAME}", field.name.as_str()).as_str())?,
                self.mangle_identifier(self.setter_pattern.replace("{NAME}", field.name.as_str()).as_str())?
            ))
        }
    }

    fn mangle_identifier(&self, ident: &str) -> Result<String, IdentifierManglingError> {
        if self.rustify_names {
            rustify_identifier(ident)
        } else {
            javaify_identifier(ident)
        }
    }
}
