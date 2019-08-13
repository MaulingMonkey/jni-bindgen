use super::*;

use std::collections::*;
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Debug, Default)]
pub(crate) struct Module {
    // For consistent diffs / printing order, these should *not* be HashMaps
    pub(crate) structs: BTreeMap<String, Struct>,
    pub(crate) modules: BTreeMap<String, Module>,
}

impl Module {
    pub(crate) fn write(&self, context: &Context, indent: &str, out: &mut impl Write) -> io::Result<()> {
        let next_indent = format!("{}    ", indent);

        for (name, module) in self.modules.iter() {
            writeln!(out, "")?;
            if indent.is_empty() {
                writeln!(out, "#[allow(non_camel_case_types)] // We map Java inner classes to Outer_Inner")?;
                writeln!(out, "#[allow(dead_code)] // We generate structs for private Java types too, just in case.")?;
                writeln!(out, "#[allow(deprecated)] // We're generating deprecated types/methods")?;
            }
            writeln!(out, "{}pub mod {} {{", indent, name)?;
            writeln!(out, "{}    #[allow(unused_imports)] use super::__jni_bindgen;", indent)?;
            module.write(context, &next_indent[..], out)?;
            writeln!(out, "{}}}", indent)?;
        }

        for (_, structure) in self.structs.iter() {
            if indent.is_empty() {
                if structure.rust_struct_name.contains("_") { writeln!(out, "#[allow(non_camel_case_types)] // We map Java inner classes to Outer_Inner")?; }
                if !structure.java.is_public() { writeln!(out, "#[allow(dead_code)] // We generate structs for private Java types too, just in case.")?; }
                writeln!(out, "#[allow(deprecated)] // We're generating deprecated types/methods")?;
            }

            if context.config.codegen.shard_structs {
                writeln!(out, "{}include!({:?});", indent, &structure.sharded_class_path)?;
                let out_path = context.config.output_dir.join(&structure.sharded_class_path);
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut out = File::create(out_path)?; // Hide outer out
                writeln!(out, "// GENERATED WITH jni-bindgen, I DO NOT RECOMMEND EDITING THIS BY HAND")?;
                writeln!(out, "")?;
                structure.write(context, "", &mut out)?;
            } else {
                structure.write(context, indent, out)?;
            };
        }

        Ok(())
    }
}
