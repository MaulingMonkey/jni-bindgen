use super::*;

use std::collections::*;
use std::error::Error;
use std::fmt::Write;
use std::io;

#[derive(Debug, Default)]
pub struct Context {
    pub(crate) module:                 Module,
    pub(crate) jni_type_to_rust_type:  HashMap<String, String>, // XXX
}

impl Context {
    pub fn new() -> Self { Default::default() }

    pub fn add_struct(&mut self, java_class: Class) -> Result<(), Box<dyn Error>> {
        let mut rust_mod : &mut Module = &mut self.module;
        let mut rust_mod_prefix     = String::from("crate::");
        let mut rust_struct_name    = String::new();

        for component in JniPathIter::new(java_class.this_class().name()) {
            match component {
                JniIdentifier::Namespace(id) => {
                    let id = match RustIdentifier::from_str(id) {
                        RustIdentifier::Identifier(id) => id,
                        RustIdentifier::KeywordRawSafe(id) => id,
                        RustIdentifier::KeywordUnderscorePostfix(id) => id,
                        RustIdentifier::NonIdentifier(id) => Err(format!("Unable to add_struct(): parent java namespace name {:?} has no rust equivalent", id))?,
                    };

                    write!(&mut rust_mod_prefix, "{}::", id)?;
                    rust_mod = rust_mod.modules.entry(id.to_owned()).or_insert(Default::default());
                },
                JniIdentifier::ContainingClass(id) => {
                    let id = match RustIdentifier::from_str(id) {
                        RustIdentifier::Identifier(id) => id,
                        RustIdentifier::KeywordRawSafe(id) => id,
                        RustIdentifier::KeywordUnderscorePostfix(id) => id,
                        RustIdentifier::NonIdentifier(id) => Err(format!("Unable to add_struct(): parent java class name {:?} has no rust equivalent", id))?,
                    };

                    write!(&mut rust_struct_name, "{}_", id)?;
                },
                JniIdentifier::LeafClass(id) => {
                    let id = match RustIdentifier::from_str(id) {
                        RustIdentifier::Identifier(id) => id,
                        RustIdentifier::KeywordRawSafe(id) => id,
                        RustIdentifier::KeywordUnderscorePostfix(id) => id,
                        RustIdentifier::NonIdentifier(id) => Err(format!("Unable to add_struct(): java class name {:?} has no rust equivalent", id))?,
                    };

                    write!(&mut rust_struct_name, "{}", id)?;
                    let id = &rust_struct_name[..];

                    if rust_mod.structs.contains_key(id) {
                        Err(format!("Unable to add_struct(): java class name {:?} was already added", id))?
                    }

                    self.jni_type_to_rust_type.insert(java_class.this_class().name().clone(), format!("{}{}", rust_mod_prefix, &rust_struct_name));

                    rust_mod.structs.insert(rust_struct_name.clone(), Struct {
                        rust_mod_prefix,
                        rust_struct_name,
                        java_class,
                    });

                    return Ok(());
                },
            }
        }

        Err(format!("Failed to find LeafClass in {:?}", java_class.this_class().name()))?
    }

    pub fn write(&self, out: &mut impl io::Write) -> io::Result<()> {
        write_preamble(out)?;
        self.module.write(self, "", out)
    }
}
