@if not exist "%ProgramFiles%\Android\Android Studio\jre\" goto :error-no-android-jre
@if not exist "%LOCALAPPDATA%\Android\Sdk\tools\bin\"      goto :error-no-android-sdk

@setlocal
@set JAVA_HOME=%ProgramFiles%\Android\Android Studio\jre\
@set PATH=%LOCALAPPDATA%\Android\Sdk\tools\bin\;%PATH%

@call sdkmanager --install ^
  "platforms;android-7" ^
  "platforms;android-8" ^
  "platforms;android-9" ^
  "platforms;android-10" ^
  "platforms;android-11" ^
  "platforms;android-12" ^
  "platforms;android-13" ^
  "platforms;android-14" ^
  "platforms;android-15" ^
  "platforms;android-16" ^
  "platforms;android-17" ^
  "platforms;android-18" ^
  "platforms;android-19" ^
  "platforms;android-20" ^
  "platforms;android-21" ^
  "platforms;android-22" ^
  "platforms;android-23" ^
  "platforms;android-24" ^
  "platforms;android-25" ^
  "platforms;android-26" ^
  "platforms;android-27" ^
  "platforms;android-28" ^
  "platforms;android-29"

@endlocal
@exit /b %ERRORLEVEL%

:error-no-android-jre
:error-no-android-sdk
@echo Make sure you've installed Android Studio before running this script!
@echo     https://developer.android.com/studio/install
@echo.
@exit /b 1
