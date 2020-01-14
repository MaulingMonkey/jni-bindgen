:: Flags of note
::  --wet-run       Actually publish the build
::  --skip-build    Skip scripts\test.cmd - you probably should only do this after you've successfully dry-run this publish script once.

@setlocal
set SKIP_BUILD=0
@set DRY=1
@set PUBLISH_FLAGS= 
:flags-next
    @if "%~1" == "--wet-run" (
        @set DRY=0
    ) else if "%~1" == "--skip-build" (
        @set SKIP_BUILD=1
    ) else if "%~1" == "" (
        @goto :flags-done
    ) else (
        @set PUBLISH_FLAGS=%PUBLISH_FLAGS% %1
    )
    @shift /1
    @goto :flags-next
:flags-done
@if "%DRY%" == "1" set PUBLISH_FLAGS=%PUBLISH_FLAGS% --dry-run



@if "%SKIP_BUILD%" == "0" call "%~dp0test.cmd" "stable" "release" "windows" || exit /b 1



@pushd "%~dp0.."
cd "%~dp0..\jni-glue"
cargo publish %PUBLISH_FLAGS%
cd "%~dp0..\jni-bindgen-reflection"
cargo publish %PUBLISH_FLAGS%
cd "%~dp0..\jni-bindgen"
cargo publish %PUBLISH_FLAGS%
:: Delay publish of jni-android-sys long enough for dependencies to be available on crates.io, maybe.
if "%DRY%" == "0" ping localhost -n 10 >NUL 2>NUL
cd "%~dp0..\jni-android-sys"
cargo publish %PUBLISH_FLAGS%
@popd
@endlocal
