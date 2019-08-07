use super::*;

use std::collections::*;
use std::io;

#[derive(Debug, Default)]
pub(crate) struct Module {
    // For consistent diffs / printing order, these should *not* be HashMaps
    pub(crate) structs: BTreeMap<String, Struct>,
    pub(crate) modules: BTreeMap<String, Module>,
}

impl Module {
    pub(crate) fn write(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        let next_indent = format!("{}    ", indent);

        for (name, module) in self.modules.iter() {
            writeln!(out, "")?;
            if indent.is_empty() {
                writeln!(out, "#[allow(non_camel_case_types)] // We map Java inner classes to Outer_Inner")?;
                writeln!(out, "#[allow(dead_code)] // We generate structs for private Java types too, just in case.")?;
            }
            writeln!(out, "{}pub mod {} {{", indent, name)?;
            writeln!(out, "{}    #[allow(unused_imports)] use super::__jni_bindgen;", indent)?;
            module.write(context, &next_indent[..], out)?;
            writeln!(out, "{}}}", indent)?;
        }

        for (name, structure) in self.structs.iter() {
            if indent.is_empty() {
                if name.contains("_") { writeln!(out, "#[allow(non_camel_case_types)] // We map Java inner classes to Outer_Inner")?; }
                if !structure.java_class.access_flags().contains(ClassAccessFlags::PUBLIC) { writeln!(out, "#[allow(dead_code)] // We generate structs for private Java types too, just in case.")?; }
            }
            structure.write(context, indent, out)?;
        }

        Ok(())
    }
}
