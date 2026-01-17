#!/bin/bash

echo "Building Morse Time Clock for all platforms..."
echo ""
echo "Note: MSVC targets require Windows with Visual Studio."
echo "      Use build_windows_msvc.bat on Windows for MSVC builds."
echo ""

# Create releases directory if it doesn't exist
mkdir -p releases

# Linux x86_64
echo "Building for Linux x86_64..."
cargo build --release --target x86_64-unknown-linux-gnu
if [ $? -eq 0 ]; then
    cp target/x86_64-unknown-linux-gnu/release/morse_time_clock releases/morse_time_clock_linux_x86_64
    echo "✓ Linux x86_64 built → releases/morse_time_clock_linux_x86_64"
else
    echo "✗ Linux x86_64 build failed"
fi
echo ""

# Windows GNU targets (cross-compile from Linux)
echo "Building for Windows x86_64 (GNU)..."
cargo build --release --target x86_64-pc-windows-gnu
if [ $? -eq 0 ]; then
    cp target/x86_64-pc-windows-gnu/release/morse_time_clock.exe releases/morse_time_clock_win64_gnu.exe
    echo "✓ Windows x86_64 GNU built → releases/morse_time_clock_win64_gnu.exe"
else
    echo "✗ Windows x86_64 GNU build failed"
fi
echo ""

echo "Building for Windows i686 (GNU)..."
cargo build --release --target i686-pc-windows-gnu
if [ $? -eq 0 ]; then
    cp target/i686-pc-windows-gnu/release/morse_time_clock.exe releases/morse_time_clock_win32_gnu.exe
    echo "✓ Windows i686 GNU built → releases/morse_time_clock_win32_gnu.exe"
else
    echo "✗ Windows i686 GNU build failed"
fi
echo ""

# macOS x86_64 (requires osxcross)
echo "Building for macOS x86_64..."
if cargo build --release --target x86_64-apple-darwin 2>/dev/null; then
    cp target/x86_64-apple-darwin/release/morse_time_clock releases/morse_time_clock_macos_x86_64
    echo "✓ macOS x86_64 built → releases/morse_time_clock_macos_x86_64"
else
    echo "⚠ macOS x86_64 build failed (osxcross may not be configured)"
fi
echo ""

# macOS ARM64 (requires osxcross)
echo "Building for macOS ARM64..."
if cargo build --release --target aarch64-apple-darwin 2>/dev/null; then
    cp target/aarch64-apple-darwin/release/morse_time_clock releases/morse_time_clock_macos_aarch64
    echo "✓ macOS ARM64 built → releases/morse_time_clock_macos_aarch64"
else
    echo "⚠ macOS ARM64 build failed (osxcross may not be configured)"
fi
echo ""

echo "============================================"
echo "Build complete!"
echo "============================================"
echo ""
echo "Binaries available in releases/ directory:"
ls -lh releases/morse_time_clock_* 2>/dev/null || echo "  (No binaries found)"
echo ""
echo "For MSVC builds, run build_windows_msvc.bat on Windows."
