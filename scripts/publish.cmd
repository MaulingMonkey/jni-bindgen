@pushd "%~dp0.."
cd "%~dp0..\jni-glue"
cargo publish %*
cd "%~dp0..\jni-bindgen"
cargo publish %*
cd "%~dp0..\jni-android-sys"
cargo publish %*
@popd
