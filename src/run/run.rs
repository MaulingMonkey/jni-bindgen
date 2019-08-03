use super::*;
use config::runtime::*;
use gather_java::*;

use std::result::Result;
use std::error::Error;
use std::fs::*;
use std::io;
use std::path::*;
use std::time::*;

/// The core function of this library: Generate Rust code to access Java APIs.
pub fn run(config: impl Into<Config>) -> Result<(), Box<dyn Error>> {
    let config : Config = config.into();
    if config.logging_verbose {
        println!("output: {}", config.output_path.display());
        for file in config.input_files.iter() {
            println!("input:  {}", file.display());
        }
    }

    let mut context = emit_rust::Context::new();
    for file in config.input_files.iter() {
        gather_file(&mut context, &config, file)?;
    }

    println!("writing: {}...", config.output_path.display());

    let mut out = File::create(&config.output_path).unwrap();
    context.write(&mut out)?;

    // let _ = Command::new("cargo").args(&["+nightly", "build", "--example", "generated-android-sdk"]).status().unwrap();

    Ok(())
}

fn gather_file(context: &mut emit_rust::Context, config: &Config, path: &Path) -> Result<(), Box<dyn Error>> {
    println!("reading {}...", path.display());

    let ext = if let Some(ext) = path.extension() {
        ext
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input files must have an extension"))?;
    };

    match ext.to_string_lossy().to_ascii_lowercase().as_str() {
        "class" => {
            let mut file = File::open(path)?;
            let class = Class::try_read_all(&mut file)?;
            context.add_struct(class)?;
        },
        "jar" => {
            let mut jar = File::open(path)?;
            let mut jar = zip::ZipArchive::new(&mut jar)?;
            let n = jar.len();
            let mut next_log = Instant::now() + Duration::from_secs(1);
            for i in 0..n {
                let mut file = jar.by_index(i)?;
                if !file.name().ends_with(".class") { continue; }
                if config.logging_verbose || Instant::now() > next_log {
                    println!("  reading {:3}/{}: {}...", i, n, file.name());
                    next_log = Instant::now() + Duration::from_secs(1);
                }
                let class = Class::try_read_all(&mut file)?;
                context.add_struct(class)?;
            }
        },
        unknown => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Input files must have a '.class' or '.jar' extension, not a '.{}' extension", unknown)))?;
        }
    }
    Ok(())
}
