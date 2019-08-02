use bugsalot::*;

use bindjava::emit_rust::*;
use bindjava::gather_java::*;

use std::env;
use std::error::Error;
use std::fs::{File};
use std::io::{self};
use std::path::*;
use std::process::Command;

fn main() {
    std::panic::set_hook(Box::new(|panic|{ bug!("{:?}", panic); }));

    let local_app_data = env::var("LOCALAPPDATA").unwrap();
    let android_jar : PathBuf = [local_app_data.as_str(), "Android/Sdk/platforms/android-28/android.jar"].iter().collect();
    let mut android_jar = File::open(&android_jar).unwrap();
    let android_jar = zip::ZipArchive::new(&mut android_jar).unwrap();

    let mut output = File::create("examples/generated-android-sdk/generated.rs").unwrap();
    write(android_jar, &mut output).unwrap();
}

fn write<R: io::Read + io::Seek>(mut android_jar: zip::ZipArchive<R>, out: &mut impl io::Write) -> Result<(), Box<dyn Error>> {
    writeln!(out, "// GENERATED FILE - I DO NOT RECOMMEND EDITNIG THIS BY HAND")?;
    writeln!(out, "// Generated with:")?;
    write!(out, "//    ")?;
    for arg in std::env::args() {
        if arg.contains(' ') {
            write!(out, " \"{}\"", arg)?;
        } else {
            write!(out, " {}", arg)?;
        }
    }
    writeln!(out, "")?;

    writeln!(out, "")?;
    writeln!(out, "")?;
    writeln!(out, "")?;

    let mut context = Context::new();

    let n = android_jar.len();
    for i in 0..n {
        if i % 100 == 0 { println!("Processing {} of {} files...", i, n); }

        let mut file = android_jar.by_index(i)?;
        if !file.name().ends_with(".class") { continue; }

        let class = Class::try_read_all(&mut file)?;
        context.add_struct(class)?;
    }
    println!("");

    context.write(out)?;

    let _ = Command::new("cargo").args(&["+nightly", "build", "--example", "generated-android-sdk"]).status().unwrap();

    Ok(())
}
