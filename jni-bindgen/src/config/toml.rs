//! jni-bindgen.toml configuration file structures and parsing APIs.

use super::MethodManglingStyle;
use super::FieldManglingStyle;

use serde_derive::*;

use std::fs;
use std::io;
use std::path::*;



#[serde(rename_all = "snake_case")]
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum StaticEnvStyle {
    Explicit,
    Implicit,
    #[doc(hidden)] __NonExhaustive,
}

impl Default for StaticEnvStyle {
    fn default() -> Self { StaticEnvStyle::Explicit }
}

#[serde(rename_all = "snake_case")]
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum CodeShardingStyle {
    None,
    PerClass,
}

impl Default for CodeShardingStyle {
    fn default() -> Self { CodeShardingStyle::PerClass }
}

fn default_true() -> bool { true }
fn default_method_naming_style() -> MethodManglingStyle { MethodManglingStyle::Rustify }
fn default_method_naming_style_collision() -> MethodManglingStyle { MethodManglingStyle::RustifyShortSignature }

/// The \[codegen\] section.
#[derive(Debug, Clone, Deserialize)]
pub struct CodeGen {
    /// How static methods should accept their &Env.
    #[serde(default = "Default::default")]
    pub static_env: StaticEnvStyle,

    /// How methods should be named by default.
    #[serde(default = "default_method_naming_style")]
    pub method_naming_style: MethodManglingStyle,

    /// How methods should be named on name collision.
    #[serde(default = "default_method_naming_style_collision")]
    pub method_naming_style_collision: MethodManglingStyle,

    #[serde(default = "Default::default")]
    pub field_naming_style: FieldManglingStyle,

    #[serde(default = "default_true")]
    pub shard_structs: bool,

    #[serde(default = "default_true")]
    pub feature_per_struct: bool,
}

impl Default for CodeGen {
    fn default() -> Self {
        Self {
            static_env:                     Default::default(),
            method_naming_style:            default_method_naming_style(),
            method_naming_style_collision:  default_method_naming_style_collision(),
            field_naming_style:             Default::default(),
            shard_structs:                  true,
            feature_per_struct:             true,
        }
    }
}

fn default_empty()  -> String { String::new() }
fn default_slash()  -> String { String::from("/") }
fn default_period() -> String { String::from(".") }
fn default_comma()  -> String { String::from(",") }

/// A \[\[documentation.pattern\]\] section.
#[derive(Debug, Clone, Deserialize)]
pub struct DocumentationPattern {
    /// The URL to use for documenting a given class.  `{CLASS}` will be replaced with everything *after* the JNI prefix.
    /// 
    /// | Given:                | Use this if you want android documentation:   |
    /// | --------------------- | --------------------------------------------- |
    /// | jni_prefix = "java/"  | url_pattern = "https://developer.android.com/reference/java/{CLASS}.html"
    /// | jni_prefix = ""       | url_pattern = "https://developer.android.com/reference/{CLASS}.html"
    pub class_url_pattern: String,

    /// The URL to use for documenting a given class method.
    /// 
    /// * `{CLASS}` will be replaced with everything *after* the JNI prefix.
    /// * `{METHOD}` will be replaced with the method name.
    /// * `{ARGUMENTS}` will be replaced with the method arguments.
    /// 
    /// | Given:                | Use this if you want android documentation:   |
    /// | --------------------- | --------------------------------------------- |
    /// | jni_prefix = "java/"  | url_pattern = "https://developer.android.com/reference/java/{CLASS}.html#{METHOD}({ARGUMENTS})"
    /// | jni_prefix = ""       | url_pattern = "https://developer.android.com/reference/{CLASS}.html#{METHOD}({ARGUMENTS})"
    pub method_url_pattern: Option<String>,

    /// The URL to use for documenting a given class field.
    /// 
    /// * `{CLASS}` will be replaced with everything *after* the JNI prefix.
    /// * `{FIELD}` will be replaced with the field name.
    /// 
    /// | Given:                | Use this if you want android documentation:   |
    /// | --------------------- | --------------------------------------------- |
    /// | jni_prefix = "java/"  | url_pattern = "https://developer.android.com/reference/java/{CLASS}.html#{FIELD}"
    /// | jni_prefix = ""       | url_pattern = "https://developer.android.com/reference/{CLASS}.html#{FIELD}"
    pub field_url_pattern: Option<String>,

    /// What java class(es) to match against.  This takes the form of a simple prefix to a JNI path with no wildcards.
    /// 
    /// | To Match:                 | Use a JNI Prefix:                     |
    /// | ------------------------- | ------------------------------------- |
    /// | *                         | jni_prefix = ""
    /// | java.lang.*               | jni_prefix = "java/lang/"
    /// | name.spaces.OuterClass.*  | jni_prefix = "name/spaces/OuterClass$"
    #[serde(default = "default_empty")]
    pub jni_prefix: String,

    /// What to use in the {CLASS} portion of URLs to separate namespaces.  Defaults to "/".
    #[serde(default = "default_slash")]
    pub class_namespace_separator: String,

    /// What to use in the {CLASS} portion of URLs to separate inner classes from outer classes.  Defaults to ".".
    #[serde(default = "default_period")]
    pub class_inner_class_seperator: String,

    /// What to use in the {ARGUMENTS} portion of URLs to separate namespaces.  Defaults to ".".
    #[serde(default = "default_period")]
    pub argument_namespace_separator: String,

    /// What to use in the {ARGUMENTS} portion of URLs to separate inner classes from outer classes.  Defaults to ".".
    #[serde(default = "default_period")]
    pub argument_inner_class_seperator: String,

    /// What to use in the {ARGUMENTS} portion of URLs to separate namespaces.  Defaults to ".".
    #[serde(default = "default_period")]
    pub method_namespace_separator: String,

    /// What to use in the {ARGUMENTS} portion of URLs to separate inner classes from outer classes.  Defaults to ".".
    #[serde(default = "default_period")]
    pub method_inner_class_seperator: String,

    /// What to use in the {ARGUMENTS} portion of URLs to separate inner classes from outer classes.  Defaults to ",".
    #[serde(default = "default_comma")]
    pub method_argument_seperator: String,
}

/// The \[documentation\] section.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Documentation {
    /// Documentation sources.  Processed from top to bottom.
    #[serde(rename = "pattern")] #[serde(default = "Vec::new")]
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
    #[serde(default = "Default::default")]
    pub verbose: bool,
}

/// An \[[ignore\]] section.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Ignore {
    pub class:     String,
    pub field:     Option<String>,
    pub method:    Option<String>,
    pub signature: Option<String>,
}

/// A \[[rename\]] section.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Rename {
    pub to:         String,
    pub class:      String,
    pub field:      Option<String>,
    pub method:     Option<String>,
    pub signature:  Option<String>,
}

/// Format for a `jni-bindgen.toml` file or in-memory settings.
/// 
/// # Example File
/// 
/// ```toml
/// # For system libraries, you probably only want/need a single documentation URL... but as an example, I have
/// # overridden java.* to use the Oracle Java SE 7 docs instead of the android docs.  More useful if you have a
/// # slew of .jar s from different sources you want to bind all at once, or if the platform documentation is broken
/// # up by top level modules in strange ways.
/// 
/// [codegen]
/// static_env                      = "implicit"
/// method_naming_style             = "java"
/// method_naming_style_collision   = "rustify_long_signature"
/// 
/// [logging]
/// verbose = true
/// 
/// [[documentation.pattern]]
/// class_url_pattern       = "https://docs.oracle.com/javase/7/docs/api/index.html?java/{PATH}.html"
/// jni_prefix              = "java/"
/// namespace_separator     = "/"
/// inner_class_seperator   = "."
/// 
/// [[documentation.pattern]]
/// class_url_pattern       = "https://developer.android.com/reference/kotlin/{PATH}.html"
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
/// 
/// 
/// 
/// [[ignore]]
/// class = "some/java/Class"
/// 
/// [[ignore]]
/// class = "some/java/Class"
/// method = "someMethod"
/// 
/// [[ignore]]
/// class = "some/java/Class"
/// method = "someOtherMethod"
/// signature = "()V"
///
/// 
/// 
/// [[rename]]
/// class = "some/java/Class"
/// to    = "class"
///
/// [[rename]]
/// class  = "some/java/Class"
/// method = "someMethod"
/// to     = "some_method"
///
/// [[rename]]
/// class     = "some/java/Class"
/// method    = "someOtherMethod"
/// signature = "()V"
/// to        = "some_other_method"
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct File {
    #[serde(default = "Default::default")]
    pub codegen: CodeGen,

    /// Documentation settings.
    #[serde(default = "Default::default")]
    pub documentation: Documentation,

    /// Input(s) into the jni-bindgen process.
    pub input: Input,

    /// Logging settings
    #[serde(default = "Default::default")]
    pub logging: Logging,

    /// Output(s) from the jni-bindgen process.
    pub output: Output,

    /// Classes and class methods to ignore.
    #[serde(rename = "ignore")] #[serde(default = "Vec::new")]
    pub ignores: Vec<Ignore>,

    /// Classes and class methods to rename.
    #[serde(rename = "rename")] #[serde(default = "Vec::new")]
    pub renames: Vec<Rename>,
}

fn empty_vec<T>() -> Vec<T> { Vec::new() }

impl File {
    /// Read from I/O, under the assumption that it's in the "jni-bindgen.toml" file format.
    pub fn read(file: &mut impl io::Read) -> io::Result<Self> {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?; // Apparently toml can't stream.
        Self::read_str(&buffer[..])
    }

    /// Read from a memory buffer, under the assumption that it's in the "jni-bindgen.toml" file format.
    pub fn read_str(buffer: &str) -> io::Result<Self> {
        let file : File = toml::from_str(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(file)
    }

    /// Search the current directory - or failing that, it's ancestors - until we find "jni-bindgen.toml" or reach the
    /// root of the filesystem and cannot continue.
    pub fn from_current_directory() -> io::Result<FileWithContext> {
        Self::from_directory(std::env::current_dir()?.as_path())
    }

    /// Search the specified directory - or failing that, it's ancestors - until we find "jni-bindgen.toml" or reach the
    /// root of the filesystem and cannot continue.
    pub fn from_directory(path: &Path) -> io::Result<FileWithContext> {
        let original = path;
        let mut path = path.to_owned();
        loop {
            path.push("jni-bindgen.toml");
            println!("cargo:rerun-if-changed={}", path.display());
            if path.exists() {
                let file = File::read(&mut fs::File::open(&path)?)?;
                path.pop();
                return Ok(FileWithContext { file, directory: path });
            }
            if !path.pop() || !path.pop() {
                Err(io::Error::new(io::ErrorKind::NotFound, format!("Failed to find jni-bindgen.toml in \"{}\" or any of it's parent directories.", original.display())))?;
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

        [codegen]
        static_env                      = "implicit"
        method_naming_style             = "java"
        method_naming_style_collision   = "rustify_long_signature"

        [logging]
        verbose = true

        [[documentation.pattern]]
        class_url_pattern             = "https://docs.oracle.com/javase/7/docs/api/index.html?java/{CLASS}.html"
        jni_prefix                    = "java/"
        class_namespace_separator     = "/"
        class_inner_class_seperator   = "."
        argument_namespace_separator  = "."
        argument_inner_class_seperator= "."

        [[documentation.pattern]]
        class_url_pattern             = "https://developer.android.com/reference/kotlin/{CLASS}.html"

        [input]
        files = [
            "%LOCALAPPDATA%/Android/Sdk/platforms/android-28/android.jar"
        ]

        [output]
        path = "android28.rs"



        [[ignore]]
        class = "some/java/Class"

        [[ignore]]
        class  = "some/java/Class"
        method = "someMethod"

        [[ignore]]
        class     = "some/java/Class"
        method    = "someOtherMethod"
        signature = "()V"



        [[rename]]
        class = "some/java/Class"
        to    = "class"

        [[rename]]
        class  = "some/java/Class"
        method = "someMethod"
        to     = "some_method"

        [[rename]]
        class     = "some/java/Class"
        method    = "someOtherMethod"
        signature = "()V"
        to        = "some_other_method"
    "#;
    let file = File::read_str(well_configured_toml).unwrap();

    assert_eq!(file.codegen.static_env,                     StaticEnvStyle::Implicit);
    assert_eq!(file.codegen.method_naming_style,            MethodManglingStyle::Java);
    assert_eq!(file.codegen.method_naming_style_collision,  MethodManglingStyle::RustifyLongSignature);

    assert_eq!(file.logging.verbose, true);

    assert_eq!(file.documentation.patterns.len(), 2);

    assert_eq!(file.documentation.patterns[0].class_url_pattern,            "https://docs.oracle.com/javase/7/docs/api/index.html?java/{CLASS}.html");
    assert_eq!(file.documentation.patterns[0].jni_prefix,                   "java/");
    assert_eq!(file.documentation.patterns[0].class_namespace_separator,    "/");
    assert_eq!(file.documentation.patterns[0].class_inner_class_seperator,  ".");

    assert_eq!(file.documentation.patterns[1].class_url_pattern,            "https://developer.android.com/reference/kotlin/{CLASS}.html"           );
    assert_eq!(file.documentation.patterns[1].jni_prefix,                   "");
    assert_eq!(file.documentation.patterns[1].class_namespace_separator,    "/");
    assert_eq!(file.documentation.patterns[1].class_inner_class_seperator,  ".");

    assert_eq!(file.input.files, &[Path::new("%LOCALAPPDATA%/Android/Sdk/platforms/android-28/android.jar")]);
    assert_eq!(file.output.path, Path::new("android28.rs"));

    assert_eq!(file.ignores.len(), 3);

    assert_eq!(file.ignores[0].class,      "some/java/Class");
    assert_eq!(file.ignores[0].method,     None);
    assert_eq!(file.ignores[0].signature,  None);

    assert_eq!(file.ignores[1].class,      "some/java/Class");
    assert_eq!(file.ignores[1].method,     Some("someMethod".to_owned()));
    assert_eq!(file.ignores[1].signature,  None);

    assert_eq!(file.ignores[2].class,      "some/java/Class");
    assert_eq!(file.ignores[2].method,     Some("someOtherMethod".to_owned()));
    assert_eq!(file.ignores[2].signature,  Some("()V".to_owned()));

    assert_eq!(file.renames.len(), 3);

    assert_eq!(file.renames[0].class,      "some/java/Class");
    assert_eq!(file.renames[0].method,     None);
    assert_eq!(file.renames[0].signature,  None);
    assert_eq!(file.renames[0].to,         "class");

    assert_eq!(file.renames[1].class,      "some/java/Class");
    assert_eq!(file.renames[1].method,     Some("someMethod".to_owned()));
    assert_eq!(file.renames[1].signature,  None);
    assert_eq!(file.renames[1].to,         "some_method");

    assert_eq!(file.renames[2].class,      "some/java/Class");
    assert_eq!(file.renames[2].method,     Some("someOtherMethod".to_owned()));
    assert_eq!(file.renames[2].signature,  Some("()V".to_owned()));
    assert_eq!(file.renames[2].to,         "some_other_method");
}

#[test] fn load_minimal_toml() {
    let minimal_toml = r#"
        [input]
        files = ["%LOCALAPPDATA%/Android/Sdk/platforms/android-28/android.jar"]

        [output]
        path = "android28.rs"
    "#;
    let file = File::read_str(minimal_toml).unwrap();

    assert_eq!(file.codegen.static_env,                     StaticEnvStyle::Explicit);
    assert_eq!(file.codegen.method_naming_style,            MethodManglingStyle::Rustify);
    assert_eq!(file.codegen.method_naming_style_collision,  MethodManglingStyle::RustifyShortSignature);

    assert_eq!(file.logging.verbose, false);
    assert_eq!(file.documentation.patterns.len(), 0);
    assert_eq!(file.input.files, &[Path::new("%LOCALAPPDATA%/Android/Sdk/platforms/android-28/android.jar")]);
    assert_eq!(file.output.path, Path::new("android28.rs"));
    assert_eq!(file.ignores.len(), 0);
    assert_eq!(file.renames.len(), 0);
}

#[derive(Debug, Clone)]
pub struct FileWithContext {
    pub file:       File,
    pub directory:  PathBuf,
}
