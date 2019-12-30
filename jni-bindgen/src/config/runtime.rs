//! Runtime configuration formats.  By design, this is mostly opaque - create these from tomls instead.

use crate::config::*;

use std::collections::*;
use std::ffi::*;
use std::path::*;



pub(crate) struct DocPattern {
    pub(crate) class_url_pattern:               String,
    pub(crate) method_url_pattern:              Option<String>,
    pub(crate) constructor_url_pattern:         Option<String>,
    pub(crate) field_url_pattern:               Option<String>,
    pub(crate) jni_prefix:                      String,
    pub(crate) class_namespace_separator:       String,
    pub(crate) class_inner_class_seperator:     String,
    pub(crate) argument_namespace_separator:    String,
    pub(crate) argument_inner_class_seperator:  String,
    pub(crate) argument_seperator:              String,
}

impl From<toml::DocumentationPattern> for DocPattern {
    fn from(file: toml::DocumentationPattern) -> Self {
        Self {
            class_url_pattern:              file.class_url_pattern,
            method_url_pattern:             file.method_url_pattern,
            constructor_url_pattern:        file.constructor_url_pattern,
            field_url_pattern:              file.field_url_pattern,
            jni_prefix:                     file.jni_prefix,
            class_namespace_separator:      file.class_namespace_separator,
            class_inner_class_seperator:    file.class_inner_class_seperator,
            argument_namespace_separator:   file.argument_namespace_separator,
            argument_inner_class_seperator: file.argument_inner_class_seperator,
            argument_seperator:             file.argument_seperator,
        }
    }
}



/// Runtime configuration.  Create from a toml::File.
pub struct Config {
    pub(crate) codegen:                     toml::CodeGen,
    pub(crate) doc_patterns:                Vec<DocPattern>,
    pub(crate) input_files:                 Vec<PathBuf>,
    pub(crate) output_path:                 PathBuf,
    pub(crate) output_dir:                  PathBuf,
    pub(crate) logging_verbose:             bool,

    pub(crate) ignore_classes:              HashSet<String>,
    pub(crate) ignore_class_fields:         HashSet<String>,
    pub(crate) ignore_class_methods:        HashSet<String>,
    pub(crate) ignore_class_method_sigs:    HashSet<String>,

    pub(crate) rename_classes:              HashMap<String, String>,
    pub(crate) rename_class_fields:         HashMap<String, String>,
    pub(crate) rename_class_methods:        HashMap<String, String>,
    pub(crate) rename_class_method_sigs:    HashMap<String, String>,
}

impl From<toml::FileWithContext> for Config {
    fn from(fwc: toml::FileWithContext) -> Self {
        let file = fwc.file;
        let dir  = fwc.directory;

        let documentation   = file.documentation;
        let logging         = file.logging;

        let mut ignore_classes              = HashSet::new();
        let mut ignore_class_fields         = HashSet::new();
        let mut ignore_class_methods        = HashSet::new();
        let mut ignore_class_method_sigs    = HashSet::new();
        for ignore in file.ignores {
            // TODO: Warn if sig && !method
            // TODO: Warn if field && method
            if let Some(method) = ignore.method.as_ref() {
                if let Some(sig) = ignore.signature.as_ref() {
                    ignore_class_method_sigs.insert(format!("{}\x1f{}\x1f{}", ignore.class, method, sig));
                } else {
                    ignore_class_methods.insert(format!("{}\x1f{}", ignore.class, method));
                }
            } else if let Some(field) = ignore.field.as_ref() {
                ignore_class_fields.insert(format!("{}\x1f{}", ignore.class, field));
            } else {
                ignore_classes.insert(ignore.class.clone());
            }
        }

        let mut rename_classes              = HashMap::new();
        let mut rename_class_fields         = HashMap::new();
        let mut rename_class_methods        = HashMap::new();
        let mut rename_class_method_sigs    = HashMap::new();
        for rename in file.renames {
            // TODO: Warn if sig && !method
            // TODO: Warn if field && method
            if let Some(method) = rename.method.as_ref() {
                if let Some(sig) = rename.signature.as_ref() {
                    rename_class_method_sigs.insert(format!("{}\x1f{}\x1f{}", rename.class, method, sig), rename.to.clone());
                } else {
                    rename_class_methods.insert(format!("{}\x1f{}", rename.class, method), rename.to.clone());
                }
            } else if let Some(field) = rename.field.as_ref() {
                rename_class_fields.insert(format!("{}\x1f{}", rename.class, field), rename.to.clone());
            } else {
                rename_classes.insert(rename.class.clone(), rename.to.clone());
            }
        }

        let output_path = resolve_file(file.output.path, &dir);
        let output_dir = if let Some(p) = output_path.parent() {
            p.to_owned()
        } else {
            PathBuf::new()
        };

        Self {
            codegen:                file.codegen.clone(),
            doc_patterns:           documentation.patterns.into_iter().map(|pat| pat.into()).collect(),
            input_files:            file.input.files.into_iter().map(|file| resolve_file(file, &dir)).collect(),
            output_path,
            output_dir,
            logging_verbose:        logging.verbose,
            ignore_classes,
            ignore_class_fields,
            ignore_class_methods,
            ignore_class_method_sigs,
            rename_classes,
            rename_class_fields,
            rename_class_methods,
            rename_class_method_sigs,
        }
    }
}

fn resolve_file(path: PathBuf, dir: &PathBuf) -> PathBuf {
    let path : PathBuf = match path.into_os_string().into_string() {
        Ok(string) => OsString::from(expand_vars(string)),
        Err(os_string) => os_string,
    }.into();

    let path = if path.is_relative() { dir.clone().join(path) } else { path };
    path
}

fn expand_vars(string: String) -> String {
    let mut buf = String::new();

    let mut expanding = false;
    for segment in string.split('%') {
        if expanding {
            if let Ok(replacement) = std::env::var(segment) {
                buf.push_str(&replacement[..]);
            } else {
                println!("cargo:rerun-if-env-changed={}", segment);
                buf.push('%');
                buf.push_str(segment);
                buf.push('%');
            }
        } else {
            buf.push_str(segment);
        }
        expanding = !expanding;
    }
    assert!(expanding, "Uneven number of %s in path: {:?}, would mis-expand into: {:?}", &string, &buf);
    buf
}
