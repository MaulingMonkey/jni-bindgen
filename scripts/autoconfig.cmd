:: Vars defined
::
:: Read by other software:
::   JAVA_HOME
::   RUST_BACKTRACE
::   RUSTUP_TOOLCHAIN
::   PATH
::
:: Internal to test scripts:
::   JNI_ANDROID_SYS_RUSTFLAGS
::   JNI_ANDROID_SYS_FEATURES
::   JBG_CARGO_BUILD_FLAGS
::   JBG_CONFIG
::   JBG_ERRORS
::   JBG_GRADLE_CONFIG
::   JBG_JAVA_FLAGS
::   JBG_JAVAC_FLAGS
::   JBG_PLATFORM
::   JBG_CHANNEL
::   JBG_WEB_PACK_FLAGS

@IF NOT DEFINED JAVA_HOME    IF DEFINED ProgramW6432    IF EXIST "%ProgramW6432%\Android\Android Studio\jre\"    set JAVA_HOME=%ProgramW6432%\Android\Android Studio\jre
@IF NOT DEFINED JAVA_HOME    IF DEFINED ProgramFiles    IF EXIST "%ProgramFiles%\Android\Android Studio\jre\"    set JAVA_HOME=%ProgramFiles%\Android\Android Studio\jre
@IF NOT DEFINED JAVA_HOME    echo Expected %%JAVA_HOME%%, couldn't auto-configure&& exit /b 1
@where javac >NUL 2>NUL || set PATH=%JAVA_HOME%\bin;%PATH%
@set RUST_BACKTRACE=1

@if not defined JBG_ERRORS    set JBG_ERRORS=0
@if not defined JBG_CONFIG    set JBG_CONFIG=debug
@if not defined JBG_PLATFORM  set JBG_PLATFORM=windows
@if not defined JBG_CHANNEL   set JBG_CHANNEL=msrv
@call :set-config-%JBG_CONFIG%       || goto :err-config
@call :set-platform-%JBG_PLATFORM%   || goto :err-platform
@call :set-toolchain-%JBG_CHANNEL%   || goto :err-toolchain
::@set JNI_
::@set JBG_
::@set RUSTUP_
::@set RUST_
::@set JAVA_
:: Check if we're interactive
@if "%~1" == "" goto :EOF
:: Nope, wrapper
@cmd %*
@exit %ERRORLEVEL%



:err-config
    @echo Expected %%JBG_CONFIG%% to be "debug" or "release", instead it was "%JBG_CONFIG%"
    @exit /b 1
:set-config-debug
    @set JBG_CARGO_BUILD_FLAGS= 
    @set JBG_GRADLE_CONFIG=Debug
    @set JBG_JAVA_FLAGS=-ea -esa
    @set JBG_JAVAC_FLAGS=-g
    @set JBG_WEB_PACK_FLAGS=--dev
    @exit /b 0
:set-config-release
    @set JBG_CARGO_BUILD_FLAGS=--release
    @set JBG_GRADLE_CONFIG=Release
    @set JBG_JAVA_FLAGS=-da -dsa
    @set JBG_JAVAC_FLAGS=-g:none
    @set JBG_WEB_PACK_FLAGS=--release
    @exit /b 0



:err-platform
    @echo Expected %%JBG_PLATFORM%% to be "android", "linux", or "windows", instead it was "%JBG_PLATFORM%"
    @exit /b 1
:set-platform-android
:set-platform-linux
:set-platform-windows
    @exit /b 0



:err-toolchain
    @echo Expected %%JBG_CHANNEL%% to be "msrv", "stable", "beta", or "nightly", instead it was "%JBG_CHANNEL%"
    @exit /b 1
:set-toolchain-msrv
    @set RUSTUP_TOOLCHAIN=1.36.0
    @set JNI_ANDROID_SYS_RUSTFLAGS= 
    @set JNI_ANDROID_SYS_FEATURES= 
    @exit /b 0
:set-toolchain-stable
    @set RUSTUP_TOOLCHAIN=stable
    @set JNI_ANDROID_SYS_RUSTFLAGS= 
    @set JNI_ANDROID_SYS_FEATURES= 
    @exit /b 0
:set-toolchain-beta
    @set RUSTUP_TOOLCHAIN=beta
    @set JNI_ANDROID_SYS_RUSTFLAGS= 
    @set JNI_ANDROID_SYS_FEATURES= 
    @exit /b 0
:set-toolchain-nightly
    @set RUSTUP_TOOLCHAIN=nightly
    @set JNI_ANDROID_SYS_RUSTFLAGS=-Ztime-passes
    @set "JNI_ANDROID_SYS_FEATURES= nightly"
    @exit /b 0
