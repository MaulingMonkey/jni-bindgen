@pushd "%~dp0.."
@for /f "tokens=3" %%n in ('type Cargo.lock ^| findstr name') do @call :dependency %%n
@popd && exit /b 0

:dependency
@set "DEPENDENCY=%~1"
@if /i "%DEPENDENCY%" == "gamepads" exit /b 0
@if /i "%DEPENDENCY%" == "get_gamepads_dumper" exit /b 0
@set "DEPENDENCY_PADDED=[%DEPENDENCY%](https://crates.io/crates/%DEPENDENCY%)                                                                                             "
@set "LICENSE_PADDED=![License: %DEPENDENCY%](https://img.shields.io/crates/l/%DEPENDENCY%.svg)                                                                       "
::@echo Dependency: %DEPENDENCY%
@echo ^| %DEPENDENCY_PADDED:~0,97% ^| %LICENSE_PADDED:~0,125%% ^|
@exit /b 0
