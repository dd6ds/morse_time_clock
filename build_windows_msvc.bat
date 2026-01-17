@echo off
REM Build script for Windows with MSVC
REM Run this on a Windows system with Visual Studio installed

echo Building Morse Time Clock for Windows MSVC targets...
echo.

REM Create releases directory if it doesn't exist
if not exist releases mkdir releases

REM Ensure targets are installed
rustup target add x86_64-pc-windows-msvc
rustup target add i686-pc-windows-msvc

REM Windows x86_64 MSVC
echo Building for Windows x86_64 MSVC...
cargo build --release --target x86_64-pc-windows-msvc
if %errorlevel% equ 0 (
    copy /Y target\x86_64-pc-windows-msvc\release\morse_time_clock.exe releases\morse_time_clock_win64_msvc.exe
    echo Done: Windows x86_64 MSVC -^> releases\morse_time_clock_win64_msvc.exe
) else (
    echo Failed: Windows x86_64 MSVC build
)
echo.

REM Windows i686 MSVC
echo Building for Windows i686 MSVC...
cargo build --release --target i686-pc-windows-msvc
if %errorlevel% equ 0 (
    copy /Y target\i686-pc-windows-msvc\release\morse_time_clock.exe releases\morse_time_clock_win32_msvc.exe
    echo Done: Windows i686 MSVC -^> releases\morse_time_clock_win32_msvc.exe
) else (
    echo Failed: Windows i686 MSVC build
)
echo.

echo ============================================
echo Build complete!
echo ============================================
echo.
echo Binaries available in releases\ directory:
dir /B releases\morse_time_clock_*.exe 2>nul || echo   (No binaries found)
echo.
pause
