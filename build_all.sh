#!/bin/bash

echo "Building Morse Time Clock for all platforms..."
echo ""
echo "Note: MSVC targets require Windows with Visual Studio."
echo "      Use build_windows_msvc.bat on Windows for MSVC builds."
echo ""

# Linux x86_64
echo "Building for Linux x86_64..."
cargo build --release --target x86_64-unknown-linux-gnu
echo "✓ Linux x86_64 built"
echo ""

# Windows GNU targets (cross-compile from Linux)
echo "Building for Windows x86_64 (GNU)..."
cargo build --release --target x86_64-pc-windows-gnu
echo "✓ Windows x86_64 GNU built"
echo ""

echo "Building for Windows i686 (GNU)..."
cargo build --release --target i686-pc-windows-gnu
echo "✓ Windows i686 GNU built"
echo ""

# macOS x86_64 (requires osxcross)
echo "Building for macOS x86_64..."
if cargo build --release --target x86_64-apple-darwin 2>/dev/null; then
    echo "✓ macOS x86_64 built"
else
    echo "⚠ macOS x86_64 build failed (osxcross may not be configured)"
fi
echo ""

# macOS ARM64 (requires osxcross)
echo "Building for macOS ARM64..."
if cargo build --release --target aarch64-apple-darwin 2>/dev/null; then
    echo "✓ macOS ARM64 built"
else
    echo "⚠ macOS ARM64 build failed (osxcross may not be configured)"
fi
echo ""

echo "Build complete!"
echo ""
echo "GNU/Linux Binaries:"
echo "  target/x86_64-unknown-linux-gnu/release/morse_time_clock"
echo "  target/x86_64-pc-windows-gnu/release/morse_time_clock.exe"
echo "  target/i686-pc-windows-gnu/release/morse_time_clock.exe"
echo ""
echo "macOS Binaries (if built):"
echo "  target/x86_64-apple-darwin/release/morse_time_clock"
echo "  target/aarch64-apple-darwin/release/morse_time_clock"
echo ""
echo "For MSVC builds, run build_windows_msvc.bat on Windows."
