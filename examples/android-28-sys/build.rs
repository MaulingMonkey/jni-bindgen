fn main() {
    jni_bindgen::run(jni_bindgen::config::toml::File::from_current_directory().unwrap()).unwrap();
}
