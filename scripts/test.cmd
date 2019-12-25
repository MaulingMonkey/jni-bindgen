:: Main entry point
@setlocal
@pushd "%~dp0.."
@if defined CI echo on
@if defined CI set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"

@set JBG_ERRORS=0
@set BUILDS_LOG="%TEMP%\gamepads-builds-list.txt"
@echo Channel    Config     Platform   Result>%BUILDS_LOG%
@echo ---------------------------------------->>%BUILDS_LOG%
@call :build %*
@if "%JBG_ERRORS%" == "0" goto :all-builds-succeeded
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

:: Parameters
@call :var-default JBG_CHANNEL  "%~1" * msrv stable beta nightly
@call :var-default JBG_CONFIG   "%~2" * debug release
@call :var-default JBG_PLATFORM "%~3" * android linux windows

@if not defined JBG_INNER echo :: Re-run test.cmd with the last set of arguments used>"%~dp0retest.cmd"
@if not defined JBG_INNER echo @call "%%~dp0test.cmd" "%JBG_CHANNEL%" "%JBG_CONFIG%" "%JBG_PLATFORM%">>"%~dp0retest.cmd"
@if not defined JBG_INNER set JBG_INNER=1

:: Handle wildcards

@if not "%JBG_CHANNEL%" == "*" goto :skip-channel-wildcard
    @call :build msrv    "%JBG_CONFIG%" "%JBG_PLATFORM%"
    @call :build stable  "%JBG_CONFIG%" "%JBG_PLATFORM%"
    @call :build beta    "%JBG_CONFIG%" "%JBG_PLATFORM%"
    @call :build nightly "%JBG_CONFIG%" "%JBG_PLATFORM%"
    @endlocal && set JBG_ERRORS=%JBG_ERRORS%&& exit /b 0
:skip-channel-wildcard

@if not "%JBG_CONFIG%" == "*" goto :skip-config-wildcard
    @call :build "%JBG_CHANNEL%" debug   "%JBG_PLATFORM%"
    @call :build "%JBG_CHANNEL%" release "%JBG_PLATFORM%"
    @endlocal && set JBG_ERRORS=%JBG_ERRORS%&& exit /b 0
:skip-config-wildcard

@if not "%JBG_PLATFORM%" == "*" goto :skip-platform-wildcard
    @call :build "%JBG_CHANNEL%" "%JBG_CONFIG%" windows
    @call :build "%JBG_CHANNEL%" "%JBG_CONFIG%" linux
    @call :build "%JBG_CHANNEL%" "%JBG_CONFIG%" android
    @endlocal && set JBG_ERRORS=%JBG_ERRORS%&& exit /b 0
:skip-platform-wildcard

:: If we got this far, JBG_CHANNEL, JBG_CONFIG, and JBG_PLATFORM are all non-wildcards.

@set "PAD=                      "
@set "PAD_CHANNEL=%JBG_CHANNEL%%PAD%"
@set "PAD_CONFIG=%JBG_CONFIG%%PAD%"
@set "PAD_PLATFORM=%JBG_PLATFORM%%PAD%"
@set "PAD_CHANNEL=%PAD_CHANNEL:~0,10%"
@set "PAD_CONFIG=%PAD_CONFIG:~0,10%"
@set "PAD_PLATFORM=%PAD_PLATFORM:~0,10%"

:: Skip some builds due to earlier errors, non-gamepads bugs, being too lazy to install the beta toolchain, etc.

@if not "%JBG_ERRORS%" == "0" goto :build-one-skipped
@if /I "%JBG_CHANNEL%"  == "beta" echo Skipping %JBG_CHANNEL% %JBG_CONFIG% %JBG_PLATFORM%: Beta toolchain&& goto :build-one-skipped

:: Parameters -> Settings

@call "%~dp0autoconfig.cmd"

:: Build

@set BUILD_START_TIME=%TIME:~0,8%
@set RUSTFLAGS=
@goto :build-%JBG_PLATFORM% || goto :build-one-error

:build-android
    @cd "%~dp0../jni-android-sys-gen"
    ..\target\%JBG_CONFIG%\jni-android-sys-gen generate
    @cd "%~dp0../jni-android-sys"
    @set RUSTFLAGS=%JNI_ANDROID_SYS_RUSTFLAGS%
    @call :try-cargo build --features "all api-level-28 force-define %JNI_ANDROID_SYS_FEATURES%" %JBG_CARGO_BUILD_FLAGS% || goto :build-one-error
    @set RUSTFLAGS=
    @call :try-cargo doc --no-deps  --features "all api-level-28 force-define" %JBG_CARGO_BUILD_FLAGS% || goto :build-one-error
    @cd "%~dp0../jni-android-sys/examples/android-studio/basic"
    @call "gradlew.bat" assemble%JBG_GRADLE_CONFIG%
@goto :build-one-successful

:build-linux
    echo Skipping %JBG_CHANNEL% %JBG_CONFIG% %JBG_PLATFORM%: WSL builds not currently scripted&& goto :build-one-skipped
@goto :build-one-skipped

:build-windows
    @call :try-cargo build --all            %JBG_CARGO_BUILD_FLAGS% || goto :build-one-error
    @call :try-cargo test  --all            %JBG_CARGO_BUILD_FLAGS% || goto :build-one-error
    @call :try-cargo doc   --no-deps        %JBG_CARGO_BUILD_FLAGS% || goto :build-one-error
@goto :build-one-successful




:: Exit from :build
:build-one-skipped
@echo %PAD_CHANNEL% %PAD_CONFIG% %PAD_PLATFORM% skipped>>%BUILDS_LOG%
@cd "%~dp0.."
@endlocal && set JBG_ERRORS=%JBG_ERRORS%&& exit /b 0

:build-one-successful
@set BUILD_END_TIME=%TIME:~0,8%
@echo %PAD_CHANNEL% %PAD_CONFIG% %PAD_PLATFORM% ok (took %BUILD_START_TIME% .. %BUILD_END_TIME%)>>%BUILDS_LOG%
@cd "%~dp0.."
@endlocal && set JBG_ERRORS=%JBG_ERRORS%&& exit /b 0

:build-one-error
@echo %PAD_CHANNEL% %PAD_CONFIG% %PAD_PLATFORM% JBG_ERRORS>>%BUILDS_LOG%
@cd "%~dp0.."
@endlocal && set /A JBG_ERRORS=%JBG_ERRORS% + 1&& exit /b 1



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

:var-default
:: Set based on param if available
@if not "%~2"=="" set "%~1=%~2"
:: Set based on constant fallback
@if not defined %1 set "%~1=%~3"
:: %4+ are extra legal values
@exit /b 0
