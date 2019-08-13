:: Main entry point
@setlocal
@pushd "%~dp0.."
@if defined CI echo on
@if defined CI set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"

@set ERRORS=0
@set BUILDS_LOG="%TEMP%\gamepads-builds-list.txt"
@echo Channel    Config     Platform   Result>%BUILDS_LOG%
@echo ---------------------------------------->>%BUILDS_LOG%
@call :build %*
@if "%ERRORS%" == "0" goto :all-builds-succeeded
@goto :some-builds-failed

:all-builds-succeeded
@echo.
@echo.
@type %BUILDS_LOG%
@echo.
@echo.
@echo Build succeeded!
@echo.
@echo.
@endlocal && popd && exit /b 0

:some-builds-failed
@echo.
@echo.
@type %BUILDS_LOG%
@echo.
@echo.
@echo Build failed!
@echo.
@echo.
@endlocal && popd && exit /b 1



:build
@setlocal
@for /f "" %%t in ('time /t') do @set BUILD_START_TIME=%%t

:: Parameters

@set "CHANNEL=%~1"
:: stable
:: beta
:: nightly
@if not defined CHANNEL set CHANNEL=*

@set "CONFIG=%~2"
:: debug
:: release
@if not defined CONFIG set CONFIG=*

@set "PLATFORM=%~3"
:: windows
:: android
:: linux (WSL)
:: wasm
@if not defined PLATFORM set PLATFORM=*

@echo :: Re-run test.cmd with the last set of arguments used>"%~dp0retest.cmd"
@echo @call "%%~dp0test.cmd" "%CHANNEL%" "%CONFIG%" "%PLATFORM%">>"%~dp0retest.cmd"

:: Handle wildcards

@if not "%CHANNEL%" == "*" goto :skip-channel-wildcard
    @call :build stable  "%CONFIG%" "%PLATFORM%"
    @call :build beta    "%CONFIG%" "%PLATFORM%"
    @call :build nightly "%CONFIG%" "%PLATFORM%"
    @endlocal && set ERRORS=%ERRORS%&& exit /b 0
:skip-channel-wildcard

@if not "%CONFIG%" == "*" goto :skip-config-wildcard
    @call :build "%CHANNEL%" debug   "%PLATFORM%"
    @call :build "%CHANNEL%" release "%PLATFORM%"
    @endlocal && set ERRORS=%ERRORS%&& exit /b 0
:skip-config-wildcard

@if not "%PLATFORM%" == "*" goto :skip-platform-wildcard
    @call :build "%CHANNEL%" "%CONFIG%" windows
    @call :build "%CHANNEL%" "%CONFIG%" linux
    @call :build "%CHANNEL%" "%CONFIG%" wasm
    @call :build "%CHANNEL%" "%CONFIG%" android
    @endlocal && set ERRORS=%ERRORS%&& exit /b 0
:skip-platform-wildcard

:: If we got this far, CHANNEL, CONFIG, and PLATFORM are all non-wildcards.

@set "PAD=                      "
@set "PAD_CHANNEL=%CHANNEL%%PAD%"
@set "PAD_CONFIG=%CONFIG%%PAD%"
@set "PAD_PLATFORM=%PLATFORM%%PAD%"
@set "PAD_CHANNEL=%PAD_CHANNEL:~0,10%"
@set "PAD_CONFIG=%PAD_CONFIG:~0,10%"
@set "PAD_PLATFORM=%PAD_PLATFORM:~0,10%"

:: Skip some builds due to earlier errors, non-gamepads bugs, being too lazy to install the beta toolchain, etc.

@if not "%ERRORS%" == "0" goto :build-one-skipped
@if /I "%CHANNEL%"  == "beta"                echo Skipping %CHANNEL% %CONFIG% %PLATFORM%: Beta toolchain&& goto :build-one-skipped
@if /I "%PLATFORM%" == "linux" if defined CI echo Skipping %CHANNEL% %CONFIG% %PLATFORM%: Appveyor doesn't have WSL installed&& goto :build-one-skipped

:: Parameters -> Settings

@set CARGO_FLAGS= 
@if /i "%CONFIG%" == "release"   set CARGO_FLAGS=%CARGO_FLAGS% --release
::@if /i "%CHANNEL%" == "nightly"  set CARGO_FLAGS=%CARGO_FLAGS% -C -Ztime-passes
::@if /i "%CHANNEL%" == "nightly"  set "RUSTFLAGS=-Z time"

@set WEB_PACK_FLAGS= 
@if /i "%CONFIG%" == "debug"     set WEB_PACK_FLAGS=%WEB_PACK_FLAGS% --dev
@if /i "%CONFIG%" == "release"   set WEB_PACK_FLAGS=%WEB_PACK_FLAGS% --release


:: Build

@if /i not "%PLATFORM%" == "windows" goto :skip-windows
    @call :try-cargo +%CHANNEL% test  -p jni-bindgen -p jni-glue            %CARGO_FLAGS% || goto :build-one-error
    @call :try-cargo +%CHANNEL% doc   -p jni-bindgen -p jni-glue --no-deps  %CARGO_FLAGS% || goto :build-one-error
    @cd jni-android-sys
    @call :try-cargo +%CHANNEL% build            --features "api-level-28 locally-generate force-define" || goto :build-one-error
    @call :try-cargo +%CHANNEL% doc   --no-deps  --features "api-level-28 locally-generate force-define" || goto :build-one-error
    @goto :build-one-successful
:skip-windows



@if /i not "%PLATFORM%" == "android" goto :skip-platform-android
    @set JAVA_HOME=%ProgramFiles%\Android\Android Studio\jre\
    @cd "%~dp0..\jni-android-sys\examples\android-studio\basic\rust\cfg\windows-android"
    @call :try-cargo +%CHANNEL% build || goto :build-one-error
    @call "..\..\..\gradlew.bat" assembleDebug || goto :build-one-error
    @goto :build-one-successful
:skip-platform-android



@if /i "%PLATFORM%" == "linux" call :try-linux-bash cargo +%CHANNEL% test             %CARGO_FLAGS% || goto :build-one-error
@if /i "%PLATFORM%" == "linux" call :try-linux-bash cargo +%CHANNEL% build --examples %CARGO_FLAGS% || goto :build-one-error
@if /i "%PLATFORM%" == "linux" goto :build-one-successful



@if /i not "%PLATFORM%" == "wasm" goto :skip-wasm
    @call :install-cargo-web                                                                                                                                || goto :build-one-error
    @call :add-chrome-to-path                                                                                                                               || goto :build-one-error
    @cd "%~dp0..\examples\get_gamepads_dumper"                                                                                                              || goto :build-one-error
    @call :try wasm-pack build --target no-modules --out-dir ../../target/wasm32-unknown-unknown/debug/examples/get_gamepads_dumper/pkg %WEB_PACK_FLAGS%    || goto :build-one-error
    @goto :build-one-successful
:skip-wasm



@echo Unrecognized %%PLATFORM%%: %PLATFORM%
@goto :build-one-error

:: Exit from :build
:build-one-skipped
@echo %PAD_CHANNEL% %PAD_CONFIG% %PAD_PLATFORM% skipped>>%BUILDS_LOG%
@cd "%~dp0.."
@endlocal && set ERRORS=%ERRORS%&& exit /b 0

:build-one-successful
@for /f "" %%t in ('time /t') do @set BUILD_END_TIME=%%t
@echo %PAD_CHANNEL% %PAD_CONFIG% %PAD_PLATFORM% ok (took %BUILD_START_TIME% .. %BUILD_END_TIME%)>>%BUILDS_LOG%
@cd "%~dp0.."
@endlocal && set ERRORS=%ERRORS%&& exit /b 0

:build-one-error
@echo %PAD_CHANNEL% %PAD_CONFIG% %PAD_PLATFORM% ERRORS>>%BUILDS_LOG%
@cd "%~dp0.."
@endlocal && set /A ERRORS=%ERRORS% + 1&& exit /b 1



:: Utilities
:add-chrome-to-path
@where chrome >NUL 2>NUL && exit /b 0
@if exist "%ProgramFiles(x86)%\Google\Chrome\Application\chrome.exe" set "PATH=%ProgramFiles(x86)%\Google\Chrome\Application\;%PATH%" && exit /b 0
@if exist      "%ProgramFiles%\Google\Chrome\Application\chrome.exe" set      "PATH=%ProgramFiles%\Google\Chrome\Application\;%PATH%" && exit /b 0
@echo ERROR: Cannot find chrome.exe
@exit /b 1

:install-cargo-web
@where cargo-web >NUL 2>NUL && exit /b 0
cargo install cargo-web && exit /b 0
@echo ERROR: Cannot find nor install cargo-web
@exit /b 1

:try
%*
@exit /b %ERRORLEVEL%

:try-cargo
cargo %*
@exit /b %ERRORLEVEL%

:try-linux-bash
@call :try "%WINDIR%\System32\bash" --login -c '%*'
@exit /b %ERRORLEVEL%
