use super::*;
use config::runtime::*;
use jar_parser::*;

use std::result::Result;
use std::error::Error;
use std::fs::*;
use std::io::{self, BufRead, BufReader};
use std::path::*;
use std::time::*;

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

    println!("writing: {}...", config.output_path.display());

    {
        let mut out = File::create(&config.output_path).unwrap();
        context.write(&mut out)?;
    }

    if let Some(reference) = config.output_reference_path.as_ref() {
        let mut output    = BufReader::new(File::open(&config.output_path).map_err(|e| format!("Unable to open output at {}: {:?}", config.output_path.display(), e))?);
        let mut reference = BufReader::new(File::open(&reference).map_err(|e| format!("Unable to open reference at {}: {:?}", reference.display(), e))?);
        let mut output_line    = String::new();
        let mut reference_line = String::new();

        let mut line_no = 0;
        loop {
            line_no += 1;
            match (output.read_line(&mut output_line)?, reference.read_line(&mut reference_line)?) {
                (0, 0) => { break; },
                _ => {
                    if output_line != reference_line {
                        Err(format!("line {}: Expected output to match reference:\n    Output:    {}\n    Reference: {}\n", line_no, &output_line, &reference_line))?;
                    }
                },
            }
        }
    }

    Ok(())
}

fn gather_file(context: &mut emit_rust::Context,path: &Path) -> Result<(), Box<dyn Error>> {
    println!("reading {}...", path.display());

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
            let mut next_log = Instant::now() + Duration::from_secs(1);
            for i in 0..n {
                let mut file = jar.by_index(i)?;
                if !file.name().ends_with(".class") { continue; }
                if context.config.logging_verbose || Instant::now() > next_log {
                    println!("  reading {:3}/{}: {}...", i, n, file.name());
                    next_log = Instant::now() + Duration::from_secs(1);
                }
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
