#!/bin/bash
set -e

export ANDROID_HOME=~/android-sdk-tmp

print_run () {
    printf "\033[1;32m$(whoami)@$(hostname)\033[0m:\033[1;34m$(pwd)\033[0m$ "
    echo "$@"
    "$@"
}

install_rustup () {
    print_run curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=stable -y
}

install_java () {
    print_run sudo apt-get update
    print_run sudo apt-get -y install openjdk-8-jdk
}

install_android_sdk () {
    unzip --help >/dev/null 2>/dev/null || print_run sudo apt-get -y install unzip
    [ -d $ANDROID_HOME ] || mkdir $ANDROID_HOME
    pushd $ANDROID_HOME
    # Latest command line tools from https://developer.android.com/studio#downloads as of 8/22/2019
    print_run wget https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip
    print_run unzip sdk-tools-linux-4333796.zip

    # tools failed last time
    # cd $ANDROID_HOME
    # See ./android-sdk-linux/tools/android list sdk --all
    # https://developer.android.com/studio/command-line/sdkmanager
    popd
}

install_android_sdk_components () {
    PATH=$ANDROID_HOME/tools/bin:$PATH
    yes | print_run sdkmanager "platforms;android-28" "platform-tools" "ndk-bundle" "build-tools;29.0.2" | grep -v = || true
}

rustup --version >/dev/null 2>/dev/null     || install_rustup
java -? >/dev/null 2>/dev/null              || install_java
[ -f $ANDROID_HOME/tools/bin/sdkmanager ]   || install_android_sdk
install_android_sdk_components
