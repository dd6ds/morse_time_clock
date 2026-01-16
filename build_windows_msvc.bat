@echo off
REM Build script for Windows with MSVC
REM Run this on a Windows system with Visual Studio installed

echo Building Morse Time Clock for Windows MSVC targets...
echo.

REM Ensure targets are installed
rustup target add x86_64-pc-windows-msvc
rustup target add i686-pc-windows-msvc

REM Windows x86_64 MSVC
echo Building for Windows x86_64 MSVC...
cargo build --release --target x86_64-pc-windows-msvc
echo Done: Windows x86_64 MSVC
echo.

REM Windows i686 MSVC
echo Building for Windows i686 MSVC...
cargo build --release --target i686-pc-windows-msvc
echo Done: Windows i686 MSVC
echo.

echo Build complete!
echo.
echo Binaries are located in:
echo   target\x86_64-pc-windows-msvc\release\morse_time_clock.exe
echo   target\i686-pc-windows-msvc\release\morse_time_clock.exe
echo.
pause
