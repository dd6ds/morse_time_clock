# Morse Time Clock - Usage Instructions

## Quick Start

1. **Build and run the application:**
   ```bash
   cd /home/developer/rust/morse_time_clock
   cargo run --release
   ```

2. **The GUI will open showing:**
   - Current time (updates every 100ms)
   - Play button to hear the time in Morse code
   - Visual Morse code display
   - Morse code reference guide

## How It Works

### Time Display
- The current time is displayed in HH:MM:SS format
- Updates in real-time continuously

### Playing Morse Code
1. Click the "▶ Play Current Time" button
2. The application will:
   - Convert the current time to Morse code
   - Display the Morse representation visually
   - Play audio tones representing each character
   - Show "🔊 Playing..." indicator while playing

### Morse Code Representation
- **· (dot/dit)**: Short beep (100ms)
- **− (dash/dah)**: Long beep (300ms)
- **Space**: Gap between letters
- **Longer space**: Gap between numbers/characters

## Cross-Platform Building

### Prerequisites

#### For Windows Cross-Compilation (from Linux):
```bash
sudo apt-get install mingw-w64
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-gnu
```

#### For macOS Cross-Compilation (from Linux):
This requires osxcross toolkit:
```bash
# Follow instructions at: https://github.com/tpoechtrager/osxcross
# After setup, add targets:
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### Building for Specific Platforms

```bash
# Linux x86_64 (native or cross)
cargo build --release --target x86_64-unknown-linux-gnu

# Windows 64-bit
cargo build --release --target x86_64-pc-windows-gnu

# Windows 32-bit
cargo build --release --target i686-pc-windows-gnu

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# macOS Apple Silicon (M1/M2/M3)
cargo build --release --target aarch64-apple-darwin
```

### Build All Targets
Use the provided script:
```bash
./build_all.sh
```

## Technical Specifications

### Audio Parameters
- **Frequency**: 600 Hz (standard CW tone)
- **Sample Rate**: 44.1 kHz
- **Dot Duration**: 100ms
- **Dash Duration**: 300ms (3× dot length)
- **Inter-symbol Gap**: 100ms
- **Inter-letter Gap**: 300ms
- **Inter-word Gap**: 700ms

### Morse Code Support
- Numbers: 0-9
- Letters: A-Z (full alphabet)
- Colon: : (for time separator)
- Space character

### Dependencies
- **eframe/egui**: Cross-platform GUI framework
- **rodio**: Cross-platform audio playback
- **chrono**: Time and date handling

## Project Structure

```
morse_time_clock/
├── .cargo/
│   └── config.toml        # Cross-compilation linker settings
├── src/
│   └── main.rs            # Main application code
├── Cargo.toml             # Project dependencies
├── README.md              # Project documentation
├── INSTRUCTIONS.md        # This file
├── build_all.sh           # Multi-platform build script
└── .gitignore             # Git ignore rules
```

## Development Notes

### Adding New Characters
To add support for new characters, edit the `char_to_morse()` function in `src/main.rs`:

```rust
fn char_to_morse(c: char) -> Option<Vec<MorseSymbol>> {
    use MorseSymbol::*;
    match c.to_ascii_uppercase() {
        // Add your character here
        '!' => Some(vec![Dash, Dot, Dash, Dot, Dash, Dash]),
        // ... existing mappings
    }
}
```

### Adjusting Timing
Edit the timing constants at the top of `src/main.rs`:

```rust
const DOT_DURATION: u64 = 100;        // Adjust dit length
const TONE_FREQUENCY: f32 = 600.0;    // Adjust beep tone
```

### GUI Customization
The GUI layout can be modified in the `update()` method of `MorseTimeClockApp`.

## Troubleshooting

### Audio Not Working
- **Linux**: Ensure ALSA is installed: `sudo apt-get install libasound2-dev`
- **Windows**: Audio should work out of the box
- **macOS**: CoreAudio is built-in

### Build Errors
- Ensure you have the latest Rust: `rustup update`
- For cross-compilation errors, verify toolchains are installed
- Check that required development libraries are present

### Window Not Appearing
- Check that you have a display server running (X11/Wayland)
- Try running with: `RUST_LOG=debug cargo run` for detailed logging

## License

This project is created for educational and amateur radio purposes.

## Contributing

Feel free to extend this project with:
- Additional Morse code characters
- Configurable WPM (words per minute) settings
- Practice mode with random time generation
- Recording functionality
- Different audio waveforms (sine, square, sawtooth)
