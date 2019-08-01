:: XXX: VS Code's RLS might end up locking target/rls, so manually cleanup as much as we can, trying rls a few times.
@call :try-rmdir target\rls
@call :try-rmdir pkg
@call :try-rmdir target\debug
@call :try-rmdir target\doc
@call :try-rmdir target\package
@call :try-rmdir target\release
@call :try-rmdir target\rls
@call :try-rmdir target\wasm32-unknown-unknown
@call :try-rmdir target\x86_64-linux-android
@call :try-rmdir target\x86_64-pc-windows-msvc
@call :try-rmdir target\i686-linux-android
@call :try-rmdir target\i686-pc-windows-msvc
@call :try-rmdir target\rls
@call :try-rmdir target
@exit /b 0

:try-rmdir
@if not exist %1 exit /b 0
rmdir /s /q %1
@exit /b %ERRORLEVEL%
