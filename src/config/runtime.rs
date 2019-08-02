use super::*;

use std::ffi::*;
use std::path::*;



pub(crate) struct DocPattern {
    url_pattern:            String,
    jni_prefix:             String,
    namespace_separator:    String,
    inner_class_seperator:  String,
}

impl From<toml::DocumentationPattern> for DocPattern {
    fn from(file: toml::DocumentationPattern) -> Self {
        Self {
            url_pattern:            file.url_pattern,
            jni_prefix:             file.jni_prefix             .unwrap_or(String::from("")),
            namespace_separator:    file.namespace_separator    .unwrap_or(String::from("/")),
            inner_class_seperator:  file.inner_class_seperator  .unwrap_or(String::from(".")),
        }
    }
}



pub struct Config {
    pub(crate) doc_patterns:       Vec<DocPattern>,
    pub(crate) input_files:        Vec<PathBuf>,
    pub(crate) output_path:        PathBuf,
    pub(crate) logging_verbose:    bool,
}

impl From<toml::FileWithContext> for Config {
    fn from(fwc: toml::FileWithContext) -> Self {
        let file = fwc.file;
        let dir  = fwc.directory;

        let documentation   = file.documentation.unwrap_or(Default::default());
        let logging         = file.logging.unwrap_or(Default::default());

        Self {
            doc_patterns:       documentation.patterns.into_iter().map(|pat| pat.into()).collect(),
            input_files:        file.input.files.into_iter().map(|file| resolve_file(file, &dir)).collect(),
            output_path:        resolve_file(file.output.path, &dir),
            logging_verbose:    logging.verbose.unwrap_or(false),
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
