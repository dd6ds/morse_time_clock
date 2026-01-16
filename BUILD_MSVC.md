# Building Windows MSVC Versions

## Overview

The MSVC (Microsoft Visual C++) targets produce native Windows binaries that integrate better with Windows systems. However, they require building on Windows with Visual Studio or Build Tools installed.

## Prerequisites on Windows

### Option 1: Visual Studio (Recommended)
1. Download Visual Studio 2022 Community (free): https://visualstudio.microsoft.com/
2. During installation, select "Desktop development with C++"
3. This includes:
   - MSVC compiler
   - Windows SDK
   - CMake tools

### Option 2: Build Tools for Visual Studio (Minimal)
1. Download Build Tools: https://visualstudio.microsoft.com/downloads/
2. Select "Build Tools for Visual Studio 2022"
3. Install with "Desktop development with C++" workload

### Install Rust on Windows
1. Download rustup: https://rustup.rs/
2. Run the installer and follow prompts
3. Restart terminal after installation

## Building MSVC Targets

### On Windows:

```batch
REM Install targets
rustup target add x86_64-pc-windows-msvc
rustup target add i686-pc-windows-msvc

REM Build 64-bit MSVC
cargo build --release --target x86_64-pc-windows-msvc

REM Build 32-bit MSVC
cargo build --release --target i686-pc-windows-msvc

REM Or use the provided batch script
build_windows_msvc.bat
```

### Quick Build Script
The project includes `build_windows_msvc.bat` that will:
1. Install required targets
2. Build both x86_64 and i686 MSVC versions
3. Show output locations

## Cross-Compiling MSVC from Linux (Advanced)

Cross-compiling MSVC targets from Linux is complex but possible using `xwin`:

### Method 1: Using xwin (Experimental)
```bash
# Install xwin
cargo install xwin

# Download Windows SDK and MSVC tools
xwin --accept-license splat --output /opt/xwin

# Set up environment
export XWIN_ARCH=x86_64
export XWIN_SDK_ROOT=/opt/xwin

# Create .cargo/config.toml with xwin linker
# (See detailed config below)

# Build
cargo build --release --target x86_64-pc-windows-msvc
```

### Method 2: Using Docker (Recommended for Linux)
A Docker container with Windows cross-compilation tools:

```dockerfile
# Save as Dockerfile.msvc
FROM rust:latest

RUN cargo install xwin && \
    xwin --accept-license splat --output /opt/xwin

ENV XWIN_ARCH=x86_64
ENV XWIN_SDK_ROOT=/opt/xwin

COPY . /app
WORKDIR /app

CMD ["cargo", "build", "--release", "--target", "x86_64-pc-windows-msvc"]
```

```bash
# Build container and compile
docker build -f Dockerfile.msvc -t morse-msvc .
docker run -v $(pwd)/target:/app/target morse-msvc
```

## MSVC vs GNU: Which Should I Use?

### Use MSVC when:
- ✅ Building on Windows natively
- ✅ Better Windows integration needed
- ✅ Using Windows-specific APIs extensively
- ✅ Distributing to enterprise Windows users
- ✅ Smaller binary size is priority
- ✅ Best performance on Windows

### Use GNU (MinGW) when:
- ✅ Cross-compiling from Linux/macOS
- ✅ Open-source toolchain preference
- ✅ Easier build process
- ✅ No Visual Studio dependency
- ✅ Portable across different Windows versions

## Binary Comparisons

Expected sizes (release builds with LTO and strip):
- MSVC x86_64: ~3.5-4.0 MB
- GNU x86_64: ~4.5-5.0 MB
- MSVC i686: ~3.0-3.5 MB  
- GNU i686: ~4.0-4.5 MB

## Troubleshooting

### "link.exe not found"
- Install Visual Studio or Build Tools
- Make sure "Desktop development with C++" is selected
- Restart terminal after installation

### "VCINSTALLDIR not set"
- Run from "x64 Native Tools Command Prompt for VS"
- Or run: `"%ProgramFiles(x86)%\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"`

### Windows SDK missing
- Install Windows 10/11 SDK from Visual Studio Installer
- Or download standalone: https://developer.microsoft.com/windows/downloads/windows-sdk/

### Rust toolchain issues
```batch
REM Reset Rust toolchain
rustup self update
rustup update
rustup target add x86_64-pc-windows-msvc
```

## Output Locations

After building, executables will be in:
```
target/x86_64-pc-windows-msvc/release/morse_time_clock.exe
target/i686-pc-windows-msvc/release/morse_time_clock.exe
```

## Distribution

MSVC binaries can be distributed as standalone .exe files. For best compatibility:
1. Use the release build (with optimizations)
2. Include Visual C++ Redistributable if targeting older systems
3. Or use static linking (already configured in Cargo.toml)

## Additional Resources

- Rust Windows documentation: https://doc.rust-lang.org/rustc/platform-support.html
- Visual Studio downloads: https://visualstudio.microsoft.com/downloads/
- xwin project: https://github.com/Jake-Shadle/xwin
- MSVC toolchain guide: https://rust-lang.github.io/rustup/installation/windows.html
