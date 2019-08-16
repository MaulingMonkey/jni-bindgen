@call "%~dp0test.cmd" "stable" "release" "windows" || exit /b 1

@setlocal
@set DRY=1
@set PUBLISH_FLAGS= 
:flags-next
    @if "%~1" == "--wet-run" (set DRY=0) else if "%~1" == "" (goto :flags-done) else (set PUBLISH_FLAGS=%PUBLISH_FLAGS% %1)
    @shift /1
    @goto :flags-next
:flags-done
@if "%DRY%" == "1" set PUBLISH_FLAGS=%PUBLISH_FLAGS% --dry-run



@pushd "%~dp0.."
cd "%~dp0..\jni-glue"
cargo publish %PUBLISH_FLAGS%
cd "%~dp0..\jni-bindgen"
cargo publish %PUBLISH_FLAGS%
cd "%~dp0..\jni-android-sys"
cargo publish %PUBLISH_FLAGS%
@popd
@endlocal
