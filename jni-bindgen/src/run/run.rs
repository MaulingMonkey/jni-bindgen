use crate::config::runtime::*;
use crate::emit_rust;
use crate::util;

use jreflection::*;
use lazy_static::*;

use std::collections::*;
use std::error::Error;
use std::fs::*;
use std::io;
use std::path::*;
use std::result::Result;

/// The result of calling [run].
/// 
/// [run]:      fn.run.html
pub struct RunResult {
    /// What features this crate assumes exist, and the features that feature is expected to depend on.
    pub features: BTreeMap<String, BTreeSet<String>>,
}

/// The core function of this library: Generate Rust code to access Java APIs.
pub fn run(config: impl Into<Config>) -> Result<RunResult, Box<dyn Error>> {
    let config : Config = config.into();
    if config.logging_verbose {
    }
    println!("output: {}", config.output_path.display());

    lazy_static! { static ref FILE_SET : util::ConcurrentDedupeFileSet = util::ConcurrentDedupeFileSet::new(); }
    let mut context = emit_rust::Context::new(&*FILE_SET, &config);
    for file in config.input_files.iter() {
        gather_file(&mut context, file)?;
    }

    {
        let mut out = Vec::new();
        out.reserve(4096);
        context.write(&mut out)?;
        util::write_generated(&context, &config.output_path, &out[..])?;
    }

    Ok(RunResult{
        features: context.features.clone(),
    })
}

fn gather_file(context: &mut emit_rust::Context, path: &Path) -> Result<(), Box<dyn Error>> {
    context.progress.lock().unwrap().update(format!("reading {}...", path.display()).as_str());

    let ext = if let Some(ext) = path.extension() {
        ext
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input files must have an extension"))?;
    };

    match ext.to_string_lossy().to_ascii_lowercase().as_str() {
        "class" => {
            let mut file = io::BufReader::new(File::open(path)?);
            let class = Class::read(&mut file)?;
            context.add_struct(class)?;
        },
        "jar" => {
            let mut jar = zip::ZipArchive::new(io::BufReader::new(File::open(path)?))?;
            let n = jar.len();

            for i in 0..n {
                let mut file = jar.by_index(i)?;
                if !file.name().ends_with(".class") { continue; }
                context.progress.lock().unwrap().update(format!("  reading {:3}/{}: {}...", i, n, file.name()).as_str());
                let class = Class::read(&mut file)?;
                context.add_struct(class)?;
            }
        },
        unknown => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Input files must have a '.class' or '.jar' extension, not a '.{}' extension", unknown)))?;
        }
    }
    Ok(())
}
