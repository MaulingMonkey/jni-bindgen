use crate::util::{IoContext};
use crate::java::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::PathBuf;

pub struct Sdk {
    pub id:         String,
    pub label:      String,
    pub classes:    BTreeMap<String, Class>,
}

impl Sdk {
    pub fn gather_classes(id: String, label: String, classes: impl Iterator<Item = Class>) -> Self {
        Self {
            id,
            label,
            classes: classes.map(|c| (c.path.as_str().to_string(), c)).collect()
        }
    }

    pub fn gather_paths(id: String, label: String, ioc: &mut IoContext, paths: impl Iterator<Item = PathBuf>) -> Self {
        let mut classes = Vec::new();

        for path in paths {
            ioc.update(format!("reading {}...", path.display()).as_str());

            let ext = expect!(some: path.extension(); else continue; "Input file missing extension: {}", path.display());

            match ext.to_string_lossy().to_ascii_lowercase().as_str() {
                "class" => {
                    let mut file = expect!(ok: File::open(&path); else continue; "I/O error reading {}: {error}", path.display());
                    let class = expect!(ok: Class::read(&mut file); else continue; "Error parsing {}: {error}", path.display());
                    classes.push(class);
                },
                "jar" => {
                    let mut jar = expect!(ok: File::open(&path); else continue; "I/O error reading {}: {error}", path.display());
                    let mut jar = expect!(ok: zip::ZipArchive::new(&mut jar); else continue; "ZIP format error reading {}: {error}", path.display());
                    let n = jar.len();

                    for i in 0..n {
                        let mut file = expect!(ok: jar.by_index(i); else continue; "Error reading file #{} of {}: {error}", i, path.display());
                        if !file.name().ends_with(".class") { continue; }
                        ioc.update(format!("  reading {:3}/{}: {}...", i, n, file.name()).as_str());
                        let class = expect!(ok: Class::read(&mut file); else continue; "Error parsing {}: {error}", file.name());
                        classes.push(class);
                    }
                },
                _ext => {
                    expect!(failed: continue; "Input file must have a '.class' or '.jar' extension: {}", path.display());
                }
            }
        }

        Sdk::gather_classes(id, label, classes.into_iter())
    }
}
