use super::*;
use config::runtime::*;
use java::*;

use std::result::Result;
use std::error::Error;
use std::fs::*;
use std::io;
use std::path::*;

/// The core function of this library: Generate Rust code to access Java APIs.
pub fn run(config: impl Into<Config>) -> Result<(), Box<dyn Error>> {
    let config : Config = config.into();
    if config.logging_verbose {
        println!("output: {}", config.output_path.display());
    }

    for file in config.input_files.iter() {
        println!("cargo:rerun-if-changed={}", file.display());
    }

    let mut context = emit_rust::Context::new(&config);
    for file in config.input_files.iter() {
        gather_file(&mut context, file)?;
    }

    {
        let mut out = util::GeneratedFile::new(&context, &config.output_path).unwrap();
        context.write(&mut out)?;
        context.completed_file(out)?;
    }

    Ok(())
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
            let mut file = File::open(path)?;
            let class = Class::read(&mut file)?;
            context.add_struct(class)?;
        },
        "jar" => {
            let mut jar = File::open(path)?;
            let mut jar = zip::ZipArchive::new(&mut jar)?;
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
