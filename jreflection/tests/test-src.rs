use jreflection::{Class, Source};
use std::path::*;
use std::time::*;

#[cfg(windows)] #[test] fn program_files_java() {
    let java = program_files().join("Java");
    if !java.exists() { return; }

    let subdirs = java.read_dir().unwrap_or_else(|err| panic!("error reading {}: {}", java.display(), err));
    for subdir in subdirs.filter_map(|de| de.ok()) {
        let path = subdir.path();
        test_jdk(&path, || Source::from_jdk_dir(&path).unwrap());
    }
}

#[cfg(windows)] #[test] fn program_files_adopt_open_jdk() {
    let aojdk = program_files().join("AdoptOpenJDK");
    if !aojdk.exists() { return; }

    let subdirs = aojdk.read_dir().unwrap_or_else(|err| panic!("error reading {}: {}", aojdk.display(), err));
    for subdir in subdirs.filter_map(|de| de.ok()) {
        let path = subdir.path();
        test_jdk(&path, || Source::from_jdk_dir(&path).unwrap());
    }
}

#[cfg(windows)] #[test] fn local_app_data_android() {
    let android_platforms = local_app_data().join("Android").join("Sdk").join("platforms");
    if !android_platforms.exists() { return; }
    let subdirs = android_platforms.read_dir().unwrap_or_else(|err| panic!("error reading {}: {}", android_platforms.display(), err));
    for subdir in subdirs.filter_map(|de| de.ok()) {
        let path = subdir.path().join("android.jar");
        test_jdk(&path, || Source::from_jar(&path).unwrap());
    }
}

#[cfg(windows)] fn program_files() -> PathBuf {
    let pf = if cfg!(target_arch = "x86_64") { "ProgramW6432" } else { "ProgramFiles(x86)" };
    PathBuf::from(std::env::var_os(pf).or_else(|| std::env::var_os("ProgramFiles")).expect("%ProgramFiles% not set"))
}

#[cfg(windows)] fn local_app_data() -> PathBuf {
    PathBuf::from(std::env::var_os("LOCALAPPDATA").expect("%LOCALAPPDATA% not set"))
}

fn test_jdk(path: &Path, src_from: impl FnOnce() -> Source) {
    let stdout;
    let _lock;
    if std::env::args().any(|s| s == "--nocapture") {
        stdout = std::io::stdout();
        _lock = stdout.lock();
    }
    println!();
    println!("{}", path.display());
    let jdk     = bench("    load jdk:                              ", || src_from());
    let _obj    = bench("    deserialize         java/lang/Object:  ", || jdk.read_class("java/lang/Object").unwrap());
    let _obj2   = bench("    fail to deserialize java/lang/Object2: ", || jdk.read_class("java/lang/Object2").unwrap_err());
    let classes = bench("    gather all classes of jdk:             ", || jdk.classes::<Vec<String>>().unwrap());
    let classes = bench("    deserialize all classes of jdk:        ", || classes.iter().map(|c| jdk.read_class(&c).unwrap()).collect::<Vec<Class>>());
    let _ = classes;
}

fn bench<R>(prefix: &str, f: impl FnOnce() -> R) -> R {
    let start = Instant::now();
    let result = f();
    let end = Instant::now();
    let micros = (end-start).as_micros();
    println!("{}{:5}.{:03} ms", prefix, micros/1000, micros%1000);
    result
}
