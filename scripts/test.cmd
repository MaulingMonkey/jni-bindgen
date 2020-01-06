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
:: nightly
@if not defined CHANNEL set CHANNEL=*

@set "CONFIG=%~2"
:: debug
:: release
@if not defined CONFIG set CONFIG=*

@set "PLATFORM=%~3"
:: windows
@if not defined PLATFORM set PLATFORM=*

@echo :: Re-run test.cmd with the last set of arguments used>"%~dp0retest.cmd"
@echo @call "%%~dp0test.cmd" "%CHANNEL%" "%CONFIG%" "%PLATFORM%">>"%~dp0retest.cmd"

:: Handle wildcards

@if not "%CHANNEL%" == "*" goto :skip-channel-wildcard
    @call :build stable  "%CONFIG%" "%PLATFORM%"
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

:: Skip some builds due to earlier errors
@if not "%ERRORS%" == "0" goto :build-one-skipped

:: Parameters -> Settings

@set CARGO_FLAGS= 
@if /i "%CONFIG%" == "release"   set CARGO_FLAGS=%CARGO_FLAGS% --release
@if /i "%CHANNEL%" == "nightly"  set CARGO_FLAGS=%CARGO_FLAGS% --features "nightly"

@if /i "%CONFIG%" == "debug"     set GRADLE_CONFIG_FRAGMENT=Debug
@if /i "%CONFIG%" == "release"   set GRADLE_CONFIG_FRAGMENT=Release


:: Build

@if /i not "%PLATFORM%" == "windows" goto :skip-windows
    @call :try-cargo +%CHANNEL% build --all             %CARGO_FLAGS% || goto :build-one-error
    @call :try-cargo +%CHANNEL% test  --all             %CARGO_FLAGS% || goto :build-one-error
    @call :try-cargo +%CHANNEL% doc   --all --no-deps   %CARGO_FLAGS% || goto :build-one-error
    @cd "%~dp0../jni-android-sys"
    ..\target\%CONFIG%\jni-bindgen --android-api-levels=7-28 generate
    @call :try-cargo +%CHANNEL% build            --features "all api-level-28 force-define" %CARGO_FLAGS% || goto :build-one-error
    @call :try-cargo +%CHANNEL% doc   --no-deps  --features "all api-level-28 force-define" %CARGO_FLAGS% || goto :build-one-error
    @cd "%~dp0../example_android_studio"
    @set JAVA_HOME=%ProgramFiles%\Android\Android Studio\jre\
    @call "gradlew.bat" assemble%GRADLE_CONFIG_FRAGMENT%
    @goto :build-one-successful
:skip-windows



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
:try-cargo
cargo %*
@exit /b %ERRORLEVEL%
