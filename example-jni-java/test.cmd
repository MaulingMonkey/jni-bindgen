@setlocal && pushd "%~dp0"
@call ..\scripts\autoconfig.cmd || exit /b 1

:: Prepare
@mkdir ..\target\%JBG_CONFIG%\java\jars 2>NUL
@mkdir ..\target\%JBG_CONFIG%\java\classes 2>NUL
@mkdir ..\target\%JBG_CONFIG%\java\headers 2>NUL
@mkdir ..\target\%JBG_CONFIG%\java\source  2>NUL

:: Build
@call :build javac %JBG_JAVAC_FLAGS% java\*.java -d ..\target\%JBG_CONFIG%\java\classes -s ..\target\%JBG_CONFIG%\java\source -h ..\target\%JBG_CONFIG%\java\headers
@call :build cargo build %JBG_CARGO_BUILD_FLAGS% -p example-jni-java
@call :build jar cfe ..\target\%JBG_CONFIG%\java\jars\example-jni-java.jar com.maulingmonkey.jni_bindgen.example_jni_java.Program -C ..\target\%JBG_CONFIG%\java\classes com\maulingmonkey\jni_bindgen\example_jni_java
:: Windows doesn't auto-extract jar DLLs anywhere, so this isn't as useful as on Android:  -C build\rust\%JBG_CONFIG% com_maulingmonkey_jni_1bindgen_example_1jni.dll

:: Build Cleanup
@rmdir /Q ..\target\%JBG_CONFIG%\java\classes 2>NUL
@rmdir /Q ..\target\%JBG_CONFIG%\java\headers 2>NUL
@rmdir /Q ..\target\%JBG_CONFIG%\java\source  2>NUL

:: Test
@IF NOT "%JBG_ERRORS%" == "0" goto :skip-test
@set PATH=..\target\%JBG_CONFIG%\;%PATH%
@call :build java %JBG_JAVA_FLAGS% -jar ..\target\%JBG_CONFIG%\java\jars\example-jni-java.jar
:skip-test

:: Exit
@popd && endlocal && exit /b %JBG_ERRORS%



:build
%*
@if ERRORLEVEL 1 set /A JBG_ERRORS = JBG_ERRORS + 1
@exit /b %ERRORLEVEL%
