use bugsalot::bug;

fn main() {
    std::panic::set_hook(Box::new(|panic|{ bug!("{:?}", panic); }));
    bindgen_jni::run(bindgen_jni::config::toml::File::from_current_directory().unwrap()).unwrap();
}
