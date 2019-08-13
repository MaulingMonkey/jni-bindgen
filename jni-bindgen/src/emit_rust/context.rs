use super::*;

use java::class;

use std::error::Error;
use std::fmt::Write;
use std::io;
use std::path::PathBuf;

pub struct Context<'a> {
    pub(crate) config: &'a config::runtime::Config,
    pub(crate) module: Module,
}

impl<'a> Context<'a> {
    pub fn new(config: &'a config::runtime::Config) -> Self {
        Self {
            config,
            module: Default::default(),
        }
    }

    pub fn java_to_rust_path(&self, java_class: class::Id) -> Result<String, Box<dyn Error>> {
        let mut rust_name = String::from("crate::");

        for component in java_class.iter() {
            match component {
                class::IdPart::Namespace(id) => {
                    let id = match RustIdentifier::from_str(id) {
                        RustIdentifier::Identifier(id) => id,
                        RustIdentifier::KeywordRawSafe(id) => id,
                        RustIdentifier::KeywordUnderscorePostfix(id) => id,
                        RustIdentifier::NonIdentifier(id) => Err(format!("Unable to add_struct(): parent java namespace name {:?} has no rust equivalent", id))?,
                    };

                    write!(&mut rust_name, "{}::", id)?;
                },
                class::IdPart::ContainingClass(id) => {
                    let id = match RustIdentifier::from_str(id) {
                        RustIdentifier::Identifier(id) => id,
                        RustIdentifier::KeywordRawSafe(id) => id,
                        RustIdentifier::KeywordUnderscorePostfix(id) => id,
                        RustIdentifier::NonIdentifier(id) => Err(format!("Unable to add_struct(): parent java class name {:?} has no rust equivalent", id))?,
                    };

                    write!(&mut rust_name, "{}_", id)?;
                },
                class::IdPart::LeafClass(id) => {
                    let id = match RustIdentifier::from_str(id) {
                        RustIdentifier::Identifier(id) => id,
                        RustIdentifier::KeywordRawSafe(id) => id,
                        RustIdentifier::KeywordUnderscorePostfix(id) => id,
                        RustIdentifier::NonIdentifier(id) => Err(format!("Unable to add_struct(): java class name {:?} has no rust equivalent", id))?,
                    };

                    write!(&mut rust_name, "{}", id)?;

                    return Ok(rust_name);
                },
            }
        }

        Err(format!("Failed to find LeafClass in {:?}", java_class))?
    }

    pub fn add_struct(&mut self, class: java::Class) -> Result<(), Box<dyn Error>> {
        let mut rust_mod            = &mut self.module;
        let mut rust_mod_prefix     = String::from("crate::");
        let mut rust_struct_name    = String::new();
        let mut sharded_class_path  = String::new();
        if let Some(name) = self.config.output_path.file_stem() {
            sharded_class_path = name.to_string_lossy().to_string() + "/";
        }

        for component in class.path.iter() {
            match component {
                class::IdPart::Namespace(id) => {
                    let id = match RustIdentifier::from_str(id) {
                        RustIdentifier::Identifier(id) => id,
                        RustIdentifier::KeywordRawSafe(id) => id,
                        RustIdentifier::KeywordUnderscorePostfix(id) => id,
                        RustIdentifier::NonIdentifier(id) => Err(format!("Unable to add_struct(): parent java namespace name {:?} has no rust equivalent", id))?,
                    };

                    write!(&mut rust_mod_prefix, "{}::", id)?;
                    write!(&mut sharded_class_path, "{}/", id)?;
                    rust_mod = rust_mod.modules.entry(id.to_owned()).or_insert(Default::default());
                },
                class::IdPart::ContainingClass(id) => {
                    let id = match RustIdentifier::from_str(id) {
                        RustIdentifier::Identifier(id) => id,
                        RustIdentifier::KeywordRawSafe(id) => id,
                        RustIdentifier::KeywordUnderscorePostfix(id) => id,
                        RustIdentifier::NonIdentifier(id) => Err(format!("Unable to add_struct(): parent java class name {:?} has no rust equivalent", id))?,
                    };

                    write!(&mut rust_struct_name, "{}_", id)?;
                    write!(&mut sharded_class_path, "{}_", id)?;
                },
                class::IdPart::LeafClass(id) => {
                    let id = match RustIdentifier::from_str(id) {
                        RustIdentifier::Identifier(id) => id,
                        RustIdentifier::KeywordRawSafe(id) => id,
                        RustIdentifier::KeywordUnderscorePostfix(id) => id,
                        RustIdentifier::NonIdentifier(id) => Err(format!("Unable to add_struct(): java class name {:?} has no rust equivalent", id))?,
                    };

                    write!(&mut rust_struct_name, "{}", id)?;
                    write!(&mut sharded_class_path, "{}.rs", id)?;
                    let id = &rust_struct_name[..];

                    if rust_mod.structs.contains_key(id) {
                        Err(format!("Unable to add_struct(): java class name {:?} was already added", id))?
                    }

                    rust_mod.structs.insert(rust_struct_name.clone(), Struct {
                        rust_mod_prefix,
                        rust_struct_name,
                        sharded_class_path: PathBuf::from(sharded_class_path),
                        java: class,
                    });

                    return Ok(());
                },
            }
        }

        Err(format!("Failed to find LeafClass in {:?}", &class.path))?
    }

    pub fn write(&self, out: &mut impl io::Write) -> io::Result<()> {
        write_preamble(out)?;
        self.module.write(self, "", out)
    }
}
