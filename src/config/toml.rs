//! bindgen-jni.toml configuration file structures and parsing APIs.

use serde_derive::*;

use std::fs;
use std::io;
use std::path::*;



/// A \[\[documentation.patterns\]\] section.
#[derive(Debug, Clone, Deserialize)]
pub struct DocumentationPattern {
    /// The URL to use for documenting a given class.  `{PATH}` will be replaced with everything *after* the JNI prefix.
    /// 
    /// | Given:                | Use this if you want android documentation:   |
    /// | --------------------- | --------------------------------------------- |
    /// | jni_prefix = "java/"  | url_pattern = "https://developer.android.com/reference/kotlin/java/{PATH}.html"
    /// | jni_prefix = ""       | url_pattern = "https://developer.android.com/reference/kotlin/{PATH}.html"
    pub url_pattern:            String,

    /// What java class(es) to match against.  This takes the form of a simple prefix to a JNI path with no wildcards.
    /// 
    /// | To Match:                 | Use a JNI Prefix:                     |
    /// | ------------------------- | ------------------------------------- |
    /// | *                         | jni_prefix = ""
    /// | java.lang.*               | jni_prefix = "java/lang/"
    /// | name.spaces.OuterClass.*  | jni_prefix = "name/spaces/OuterClass$"
    pub jni_prefix:             Option<String>,

    /// What to use in URLs to seperate namespaces.  Defaults to "/".
    pub namespace_separator:    Option<String>,

    /// What to use in URLs to seperate inner classes from outer classes.  Defaults to ".".
    pub inner_class_seperator:  Option<String>,
}

/// The \[documentation\] section.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Documentation {
    /// Documentation sources.  Processed from top to bottom.
    pub patterns: Vec<DocumentationPattern>,
}

/// The \[input\] section.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Input {
    /// `.jar` or `.class` files to scan for JVM class info.
    /// 
    /// May in the future add support for `.apk`s, `.aab`s, etc.
    pub files: Vec<PathBuf>,
}

/// The \[output\] section.
#[derive(Debug, Clone, Deserialize)]
pub struct Output {
    /// Target `.rs` file to generate.
    pub path: PathBuf,
}

/// The \[logging\] section.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Logging {
    pub verbose: Option<bool>,
}

/// Format for a `bindgen-jni.toml` file or in-memory settings.
/// 
/// # Example File
/// 
/// ```toml
/// # For system libraries, you probably only want/need a single documentation URL... but as an example, I have
/// # overridden java.* to use the Oracle Java SE 7 docs instead of the android docs.  More useful if you have a
/// # slew of .jar s from different sources you want to bind all at once, or if the platform documentation is broken
/// # up by top level modules in strange ways.
/// 
/// [logging]
/// verbose = true
/// 
/// [[documentation.patterns]]
/// url_pattern             = "https://docs.oracle.com/javase/7/docs/api/index.html?java/{PATH}.html"
/// jni_prefix              = "java/"
/// namespace_separator     = "/"
/// inner_class_seperator   = "."
/// 
/// [[documentation.patterns]]
/// url_pattern             = "https://developer.android.com/reference/kotlin/{PATH}.html"
/// jni_prefix              = ""
/// namespace_separator     = "/"
/// inner_class_seperator   = "."
/// 
/// [input]
/// files = [
///     "%LOCALAPPDATA%/Android/Sdk/platforms/android-28/android.jar"
/// ]
/// 
/// [output]
/// path = "android28.rs"
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct File {
    /// Documentation settings.
    pub documentation: Option<Documentation>,

    /// Input(s) into the bindgen-jni process.
    pub input: Input,

    /// Logging settings
    pub logging: Option<Logging>,

    /// Output(s) from the bindgen-jni process.
    pub output: Output,
}

impl File {
    /// Read from I/O, under the assumption that it's in the "bindgen-jni.toml" file format.
    pub fn read(file: &mut impl io::Read) -> io::Result<Self> {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?; // Apparently toml can't stream.
        Self::read_str(&buffer[..])
    }

    /// Read from a memory buffer, under the assumption that it's in the "bindgen-jni.toml" file format.
    pub fn read_str(buffer: &str) -> io::Result<Self> {
        let file : File = toml::from_str(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(file)
    }

    /// Search the current directory - or failing that, it's ancestors - until we find "bindgen-jni.toml" or reach the
    /// filesystem and cannot continue.
    pub fn from_current_directory() -> io::Result<FileWithContext> {
        let cwd = std::env::current_dir()?;
        let mut path = cwd.clone();
        loop {
            path.push("bindgen-jni.toml");
            if path.exists() {
                let file = File::read(&mut fs::File::open(&path)?)?;
                path.pop();
                return Ok(FileWithContext { file, directory: path });
            }
            if !path.pop() || !path.pop() {
                Err(io::Error::new(io::ErrorKind::NotFound, format!("Failed to find bindgen-jni.toml in \"{}\" or any of it's parent directories.", cwd.display())))?;
            }
        }
    }
}

#[test] fn load_well_configured_toml() {
    let well_configured_toml = r#"
        # For system libraries, you probably only want/need a single documentation URL... but as an example, I have
        # overridden java.* to use the Oracle Java SE 7 docs instead of the android docs.  More useful if you have a
        # slew of .jar s from different sources you want to bind all at once, or if the platform documentation is broken
        # up by top level modules in strange ways.

        [logging]
        verbose = true

        [[documentation.patterns]]
        url_pattern             = "https://docs.oracle.com/javase/7/docs/api/index.html?java/{PATH}.html"
        jni_prefix              = "java/"
        namespace_separator     = "/"
        inner_class_seperator   = "."

        [[documentation.patterns]]
        url_pattern             = "https://developer.android.com/reference/kotlin/{PATH}.html"

        [input]
        files = [
            "%LOCALAPPDATA%/Android/Sdk/platforms/android-28/android.jar"
        ]

        [output]
        path = "android28.rs"
    "#;
    let file = File::read_str(well_configured_toml).unwrap();

    assert!(file.logging.is_some());
    assert_eq!(file.logging.as_ref().unwrap().verbose, Some(true));

    assert!(file.documentation.is_some());
    assert_eq!(file.documentation.as_ref().unwrap().patterns.len(), 2);

    assert_eq!(file.documentation.as_ref().unwrap().patterns[0].url_pattern,            "https://docs.oracle.com/javase/7/docs/api/index.html?java/{PATH}.html");
    assert_eq!(file.documentation.as_ref().unwrap().patterns[0].jni_prefix,             Some("java/".to_owned()));
    assert_eq!(file.documentation.as_ref().unwrap().patterns[0].namespace_separator,    Some("/".to_owned()));
    assert_eq!(file.documentation.as_ref().unwrap().patterns[0].inner_class_seperator,  Some(".".to_owned()));

    assert_eq!(file.documentation.as_ref().unwrap().patterns[1].url_pattern,            "https://developer.android.com/reference/kotlin/{PATH}.html"           );
    assert_eq!(file.documentation.as_ref().unwrap().patterns[1].jni_prefix,             None);
    assert_eq!(file.documentation.as_ref().unwrap().patterns[1].namespace_separator,    None);
    assert_eq!(file.documentation.as_ref().unwrap().patterns[1].inner_class_seperator,  None);

    assert_eq!(file.input.files, &[Path::new("%LOCALAPPDATA%/Android/Sdk/platforms/android-28/android.jar")]);
    assert_eq!(file.output.path, Path::new("android28.rs"));
}

#[test] fn load_minimal_toml() {
    let minimal_toml = r#"
        [input]
        files = ["%LOCALAPPDATA%/Android/Sdk/platforms/android-28/android.jar"]

        [output]
        path = "android28.rs"
    "#;
    let file = File::read_str(minimal_toml).unwrap();
    assert!(file.logging.is_none());
    assert!(file.documentation.is_none());
    assert_eq!(file.input.files, &[Path::new("%LOCALAPPDATA%/Android/Sdk/platforms/android-28/android.jar")]);
    assert_eq!(file.output.path, Path::new("android28.rs"));
}

#[derive(Debug, Clone)]
pub struct FileWithContext {
    pub file:       File,
    pub directory:  PathBuf,
}
