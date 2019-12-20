@if DEFINED JBG_AUTOCONFIG exit /b 0
@set        JBG_AUTOCONFIG=1

@IF NOT DEFINED JAVA_HOME    IF DEFINED ProgramW6432    IF EXIST "%ProgramW6432%\Android\Android Studio\jre\"    set JAVA_HOME=%ProgramW6432%\Android\Android Studio\jre
@IF NOT DEFINED JAVA_HOME    IF DEFINED ProgramFiles    IF EXIST "%ProgramFiles%\Android\Android Studio\jre\"    set JAVA_HOME=%ProgramFiles%\Android\Android Studio\jre
@IF NOT DEFINED JAVA_HOME    echo Expected %%JAVA_HOME%%, couldn't auto-configure&& exit /b 1
@where javac >NUL 2>NUL || set PATH=%JAVA_HOME%\bin;%PATH%

@if not defined JBG_ERRORS set JBG_ERRORS=0
@if not defined JBG_CONFIG set JBG_CONFIG=debug
@call :set-config-%JBG_CONFIG% || goto :err-config
:: Check if we're interactive
@if "%~1" == "" goto :EOF
:: Nope, wrapper
@cmd %*
@exit %ERRORLEVEL%



:err-config
@echo Expected ^%JBG_CONFIG^% to be "debug" or "release", instead it was "%JBG_CONFIG%"
@exit /b 1

:set-config-debug
@set JBG_CARGO_BUILD_FLAGS= 
@set JBG_JAVA_FLAGS=-ea -esa
@set JBG_JAVAC_FLAGS=-g
@exit /b 0

:set-config-release
@set JBG_CARGO_BUILD_FLAGS=--release
@set JBG_JAVA_FLAGS=-da -dsa
@set JBG_JAVAC_FLAGS=-g:none
@exit /b 0
