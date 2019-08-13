//! Runtime configuration formats.  By design, this is mostly opaque - create these from tomls instead.

use super::*;

use std::collections::*;
use std::ffi::*;
use std::path::*;



pub(crate) struct DocPattern {
    pub(crate) class_url_pattern:               String,
    pub(crate) method_url_pattern:              Option<String>,
    pub(crate) field_url_pattern:               Option<String>,
    pub(crate) jni_prefix:                      String,
    pub(crate) class_namespace_separator:       String,
    pub(crate) class_inner_class_seperator:     String,
    pub(crate) method_namespace_separator:      String,
    pub(crate) method_inner_class_seperator:    String,
    pub(crate) method_argument_seperator:       String,
}

impl From<toml::DocumentationPattern> for DocPattern {
    fn from(file: toml::DocumentationPattern) -> Self {
        Self {
            class_url_pattern:              file.class_url_pattern,
            method_url_pattern:             file.method_url_pattern,
            field_url_pattern:              file.field_url_pattern,
            jni_prefix:                     file.jni_prefix,
            class_namespace_separator:      file.class_namespace_separator,
            class_inner_class_seperator:    file.class_inner_class_seperator,
            method_namespace_separator:     file.method_namespace_separator,
            method_inner_class_seperator:   file.method_inner_class_seperator,
            method_argument_seperator:      file.method_argument_seperator,
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
    pub(crate) output_reference_path:       Option<PathBuf>,
    pub(crate) logging_verbose:             bool,

    pub(crate) ignore_classes:              HashSet<String>,
    pub(crate) ignore_class_methods:        HashSet<String>,
    pub(crate) ignore_class_method_sigs:    HashSet<String>,

    pub(crate) rename_classes:              HashMap<String, String>,
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
        let mut ignore_class_methods        = HashSet::new();
        let mut ignore_class_method_sigs    = HashSet::new();
        for ignore in file.ignores {
            if let Some(method) = ignore.method.as_ref() {
                if let Some(sig) = ignore.signature.as_ref() {
                    ignore_class_method_sigs.insert(format!("{}\x1f{}\x1f{}", ignore.class, method, sig));
                } else {
                    ignore_class_methods.insert(format!("{}\x1f{}", ignore.class, method));
                }
            } else {
                ignore_classes.insert(ignore.class.clone());
            }
        }

        let mut rename_classes              = HashMap::new();
        let mut rename_class_methods        = HashMap::new();
        let mut rename_class_method_sigs    = HashMap::new();
        for rename in file.renames {
            if let Some(method) = rename.method.as_ref() {
                if let Some(sig) = rename.signature.as_ref() {
                    rename_class_method_sigs.insert(format!("{}\x1f{}\x1f{}", rename.class, method, sig), rename.to.clone());
                } else {
                    rename_class_methods.insert(format!("{}\x1f{}", rename.class, method), rename.to.clone());
                }
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
            output_reference_path:  file.output.reference_path.map(|p| resolve_file(p, &dir)),
            logging_verbose:        logging.verbose,
            ignore_classes,
            ignore_class_methods,
            ignore_class_method_sigs,
            rename_classes,
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
