#!/bin/bash
set -e

export ANDROID_HOME=~/android-sdk-tmp
export RUST_BACKTRACE=1

print_run () {
    printf "\033[1;32m$(whoami)@$(hostname)\033[0m:\033[1;34m$(pwd)\033[0m$ "
    echo "$@"
    "$@"
}

pushd $(dirname "$0")
cd ..

print_run rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
print_run cargo build --all --release
print_run cargo test  --all --release
pushd jni-android-sys-gen
print_run ../target/release/jni-android-sys-gen generate
popd
pushd jni-android-sys
print_run cargo build --features "all api-level-28 force-define"
popd
pushd jni-android-sys/examples/android-studio/basic
chmod +x ./gradlew
print_run ./gradlew assembleDebug --console=plain
popd

popd
