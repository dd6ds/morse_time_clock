# Complete Build Matrix for Morse Time Clock

## Supported Target Platforms

| Platform | Architecture | Target Triple | Build From | Build Tool | Executable |
|----------|-------------|---------------|------------|------------|------------|
| **Linux** | x86_64 | `x86_64-unknown-linux-gnu` | Linux | Native | morse_time_clock |
| **Windows (GNU)** | x86_64 | `x86_64-pc-windows-gnu` | Linux/Windows | MinGW | morse_time_clock.exe |
| **Windows (GNU)** | i686 | `i686-pc-windows-gnu` | Linux/Windows | MinGW | morse_time_clock.exe |
| **Windows (MSVC)** | x86_64 | `x86_64-pc-windows-msvc` | Windows | MSVC | morse_time_clock.exe |
| **Windows (MSVC)** | i686 | `i686-pc-windows-msvc` | Windows | MSVC | morse_time_clock.exe |
| **macOS** | x86_64 | `x86_64-apple-darwin` | macOS/Linux* | Native/osxcross | morse_time_clock |
| **macOS** | ARM64 | `aarch64-apple-darwin` | macOS/Linux* | Native/osxcross | morse_time_clock |

\* Cross-compilation from Linux requires osxcross

## Quick Build Commands

### From Linux (Recommended for most targets)
```bash
# Native Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Windows GNU (cross-compile)
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target i686-pc-windows-gnu

# Build all Linux targets
./build_all.sh
```

### From Windows (For MSVC targets)
```batch
REM Windows MSVC (native)
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc

REM Or use batch script
build_windows_msvc.bat
```

### From macOS (Native builds)
```bash
# Native macOS
cargo build --release --target x86_64-apple-darwin    # Intel
cargo build --release --target aarch64-apple-darwin   # Apple Silicon

# Can also cross-compile Windows GNU
cargo build --release --target x86_64-pc-windows-gnu
```

## Build Requirements by Platform

### Linux Host
**Installed by default:**
- ✅ x86_64-unknown-linux-gnu (native)

**Additional packages needed:**
```bash
# For Windows cross-compilation
sudo apt-get install mingw-w64

# For audio support
sudo apt-get install libasound2-dev
```

**Rust targets to add:**
```bash
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-gnu
```

### Windows Host
**Visual Studio Requirements for MSVC:**
- Visual Studio 2017 or later, OR
- Build Tools for Visual Studio 2022
- "Desktop development with C++" workload

**Rust targets to add:**
```bash
rustup target add x86_64-pc-windows-msvc
rustup target add i686-pc-windows-msvc
```

### macOS Host
**Installed by default:**
- ✅ x86_64-apple-darwin (Intel Macs)
- ✅ aarch64-apple-darwin (Apple Silicon)

**Rust targets to add:**
```bash
rustup target add x86_64-pc-windows-gnu  # For Windows cross-compile
```

## Binary Size Comparison (Release + LTO + Strip)

| Target | Expected Size | Notes |
|--------|--------------|-------|
| Linux x86_64 | ~4.0 MB | With system libraries |
| Windows GNU x86_64 | ~4.5 MB | Static linked |
| Windows GNU i686 | ~4.0 MB | Static linked |
| Windows MSVC x86_64 | ~3.5 MB | Smallest Windows binary |
| Windows MSVC i686 | ~3.0 MB | Smallest overall |
| macOS x86_64 | ~4.0 MB | With frameworks |
| macOS ARM64 | ~3.5 MB | Native Apple Silicon |

## Feature Compatibility

All builds support:
- ✅ GUI with egui/eframe
- ✅ Audio playback with rodio
- ✅ Real-time clock updates
- ✅ Morse code generation
- ✅ Cross-platform file paths

## Distribution Checklist

### For Windows
- [ ] Build both GNU and MSVC versions
- [ ] Test on Windows 10 and 11
- [ ] Verify audio output works
- [ ] Check antivirus doesn't flag binary
- [ ] Consider code signing certificate

### For Linux
- [ ] Build for x86_64
- [ ] Test on Ubuntu/Debian
- [ ] Test on Fedora/RHEL
- [ ] Verify ALSA audio works
- [ ] Create .deb or .rpm packages (optional)

### For macOS
- [ ] Build universal binary (x86_64 + ARM64)
- [ ] Test on Intel and Apple Silicon Macs
- [ ] Sign the application
- [ ] Create .dmg installer (optional)
- [ ] Notarize for Gatekeeper

## Automated Build Scripts

| Script | Platform | Targets Built |
|--------|----------|---------------|
| `build_all.sh` | Linux | Linux, Windows GNU, macOS* |
| `build_windows_msvc.bat` | Windows | Windows MSVC x86_64 & i686 |

\* macOS builds require osxcross

## CI/CD Recommendations

### GitHub Actions Example
```yaml
name: Build All Platforms

on: [push, pull_request]

jobs:
  build-linux-windows:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: ./build_all.sh
      
  build-windows-msvc:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: build_windows_msvc.bat
      
  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release --target x86_64-apple-darwin
      - run: cargo build --release --target aarch64-apple-darwin
```

## Troubleshooting

### Common Issues

**"linker not found"**
- Install MinGW for Windows targets
- Install Xcode Command Line Tools for macOS targets

**"ALSA lib not found"**
```bash
sudo apt-get install libasound2-dev
```

**"link.exe not found"**
- Install Visual Studio with C++ tools
- See [BUILD_MSVC.md](BUILD_MSVC.md) for details

**macOS "osxcross not configured"**
- Follow osxcross installation guide
- Or build natively on macOS

## Complete One-Line Builds

```bash
# Build everything possible from Linux
cargo build --release --target x86_64-unknown-linux-gnu && \
cargo build --release --target x86_64-pc-windows-gnu && \
cargo build --release --target i686-pc-windows-gnu

# Build everything on Windows
cargo build --release --target x86_64-pc-windows-msvc && \
cargo build --release --target i686-pc-windows-msvc && \
cargo build --release --target x86_64-pc-windows-gnu && \
cargo build --release --target i686-pc-windows-gnu

# Build everything on macOS
cargo build --release --target x86_64-apple-darwin && \
cargo build --release --target aarch64-apple-darwin && \
cargo build --release --target x86_64-pc-windows-gnu
```
