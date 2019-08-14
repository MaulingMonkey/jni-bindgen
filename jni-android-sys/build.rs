use jni_bindgen::config::toml::FileWithContext;
use std::path::*;

fn requested_api_level() -> i32 {
    if      cfg!(feature = "api-level-29") { 29 }
    else if cfg!(feature = "api-level-28") { 28 }
    else if cfg!(feature = "api-level-27") { 27 }
    else if cfg!(feature = "api-level-26") { 26 }
    else if cfg!(feature = "api-level-25") { 25 }
    else if cfg!(feature = "api-level-24") { 24 }
    else if cfg!(feature = "api-level-23") { 23 }
    else if cfg!(feature = "api-level-22") { 22 }
    else if cfg!(feature = "api-level-21") { 21 }
    else if cfg!(feature = "api-level-20") { 20 }
    else if cfg!(feature = "api-level-19") { 19 }
    else if cfg!(feature = "api-level-18") { 18 }
    else if cfg!(feature = "api-level-17") { 17 }
    else if cfg!(feature = "api-level-16") { 16 }
    else if cfg!(feature = "api-level-15") { 15 }
    else if cfg!(feature = "api-level-14") { 14 }
    else if cfg!(feature = "api-level-13") { 13 }
    else if cfg!(feature = "api-level-12") { 12 }
    else if cfg!(feature = "api-level-11") { 11 }
    else if cfg!(feature = "api-level-10") { 10 }
    else if cfg!(feature = "api-level-9" ) {  9 }
    else if cfg!(feature = "api-level-8" ) {  8 }
    else if cfg!(feature = "api-level-7" ) {  7 }
    else if cfg!(feature = "api-level-6" ) {  6 }
    else if cfg!(feature = "api-level-5" ) {  5 }
    else if cfg!(feature = "api-level-4" ) {  4 }
    else if cfg!(feature = "api-level-3" ) {  3 }
    else if cfg!(feature = "api-level-2" ) {  2 }
    else if cfg!(feature = "api-level-1" ) {  1 }
    else                                   {  0 }
}

fn main() {
    if !cfg!(any(target_os = "android", feature = "force-define")) {
        return; // Skip - not a target OS
    }

    let api_level = match requested_api_level() {
        0 => {
            println!("cargo:warning=Specify a minimum api-level-N feature, defaulting to 28");
            28
        },
        n => n,
    };

    if api_level < 7 {
        println!("cargo:warning=Untested api-level-{} (<7).  If you've found where I can grab such an early version of the Android SDK/APIs, please comment on / reopen https://github.com/MaulingMonkey/jni-bindgen/issues/10 !", api_level);
    }

    if cfg!(feature = "locally-verify") {
        let mut config_file = load_config_file(api_level);
        config_file.file.output.path = PathBuf::from(format!("src/locally-generated/api-level-{}.rs", api_level));
        jni_bindgen::run(config_file).unwrap();
    } else if cfg!(feature = "locally-generate") {
        let mut config_file = load_config_file(api_level);
        config_file.file.output.path = PathBuf::from(format!("src/locally-generated/api-level-{}.rs", api_level));
        jni_bindgen::run(config_file).unwrap();
    } else {
        // Do nothing
    }
}

fn load_config_file(api_level: i32) -> FileWithContext {
    // Upversion android sdk if installed one is missing?  Or try to install with:
    //  set ANDROID_HOME=%LOCALAPPDATA%\Android\Sdk\
    //  set JAVA_HOME=%ProgramFiles%\Android\Android Studio\jre\
    //  set PATH=%ANDROID_HOME%\tools\bin\;%PATH%
    //  sdkmanager --install "platforms;android-NN"
    // ?
    let sdk_android_jar = if std::env::var_os("ANDROID_HOME").is_some() {
        format!("%ANDROID_HOME%/platforms/android-{}/android.jar", api_level)
    } else if cfg!(windows) {
        format!("%LOCALAPPDATA%/Android/Sdk/platforms/android-{}/android.jar", api_level)
    } else {
        panic!("ANDROID_HOME not defined and not automatically inferrable on this platform");
    };

    let mut config_file = jni_bindgen::config::toml::File::from_current_directory().unwrap();
    config_file.file.input.files.clear();
    config_file.file.input.files.push(PathBuf::from(sdk_android_jar));
    config_file
}
