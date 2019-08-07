use bugsalot::bug;

fn main() {
    std::panic::set_hook(Box::new(|panic|{ bug!("{:?}", panic); }));
    jni_bindgen::run(jni_bindgen::config::toml::File::from_current_directory().unwrap()).unwrap();
}
