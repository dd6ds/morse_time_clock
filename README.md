# Morse Time Clock

A cross-platform GUI application that displays the current time and plays it in Morse code with audio beeps.

## Features

- Real-time clock display
- Visual Morse code representation using dots (·) and dashes (−)
- Audio playback of time in Morse code
- Cross-platform: Linux, Windows (GNU & MSVC), macOS (x86_64 & ARM64)

## Documentation

- **[BUILD_MATRIX.md](BUILD_MATRIX.md)** - Complete build guide for all platforms
- **[BUILD_MSVC.md](BUILD_MSVC.md)** - Windows MSVC-specific build instructions
- **[INSTRUCTIONS.md](INSTRUCTIONS.md)** - Detailed usage and development guide
- **[MORSE_REFERENCE.md](MORSE_REFERENCE.md)** - Complete Morse code reference

## Dependencies

- Rust 1.70 or higher
- Audio system (ALSA on Linux, Windows Audio, CoreAudio on macOS)

## Building

### Native build (for your current platform)

```bash
cargo build --release
```

### Cross-compilation

First, install cross-compilation targets:

```bash
# Linux x86_64 (if not on Linux)
rustup target add x86_64-unknown-linux-gnu

# Windows x86_64 (GNU - for cross-compilation from Linux)
rustup target add x86_64-pc-windows-gnu

# Windows i686 (32-bit GNU)
rustup target add i686-pc-windows-gnu

# Windows x86_64 (MSVC - requires Windows + Visual Studio)
rustup target add x86_64-pc-windows-msvc

# Windows i686 (32-bit MSVC - requires Windows + Visual Studio)
rustup target add i686-pc-windows-msvc

# macOS x86_64
rustup target add x86_64-apple-darwin

# macOS ARM64 (Apple Silicon)
rustup target add aarch64-apple-darwin
```

#### Linux x86_64

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

#### Windows x86_64 (GNU - from Linux)

You'll need MinGW-w64 cross-compiler:

```bash
# On Ubuntu/Debian
sudo apt-get install mingw-w64

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

#### Windows i686 (32-bit GNU - from Linux)

```bash
cargo build --release --target i686-pc-windows-gnu
```

#### Windows x86_64/i686 (MSVC - on Windows only)

MSVC targets require Visual Studio or Build Tools on Windows.
See [BUILD_MSVC.md](BUILD_MSVC.md) for detailed instructions.

```batch
REM On Windows with Visual Studio installed
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc

REM Or use the provided batch script
build_windows_msvc.bat
```

#### macOS x86_64

For cross-compiling to macOS from Linux, you'll need osxcross:

```bash
# This requires macOS SDK - see https://github.com/tpoechtrager/osxcross
cargo build --release --target x86_64-apple-darwin
```

#### macOS ARM64 (Apple Silicon)

```bash
cargo build --release --target aarch64-apple-darwin
```

## Build Script

Convenience scripts to build all targets:

**On Linux:**
```bash
chmod +x build_all.sh
./build_all.sh
```

**On Windows (for MSVC):**
```batch
build_windows_msvc.bat
```

**Important:** MSVC targets require building on Windows with Visual Studio.
See [BUILD_MSVC.md](BUILD_MSVC.md) for complete instructions.

## Running

```bash
# Native
cargo run --release

# Or run the compiled binary
./target/release/morse_time_clock
```

## Usage

1. Launch the application
2. The current time is displayed and updates in real-time
3. Click "▶ Play Current Time" to hear the time in Morse code
4. The Morse code representation is displayed visually using:
   - · (dot) = short beep (dit)
   - − (dash) = long beep (dah)

## Morse Code Timings

- Dot duration: 100ms
- Dash duration: 300ms (3× dot)
- Gap between symbols: 100ms
- Gap between letters: 300ms
- Gap between words: 700ms
- Tone frequency: 600 Hz

## Technical Details

- GUI Framework: egui/eframe
- Audio: rodio
- Time handling: chrono
- Cross-platform native windows
