use chrono::Local;
use eframe::egui;
use rodio::{OutputStream, Sink, Source};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Morse code timing constants (in milliseconds)
const DOT_DURATION: u64 = 100;
const DASH_DURATION: u64 = DOT_DURATION * 3;
const SYMBOL_SPACE: u64 = DOT_DURATION;
const LETTER_SPACE: u64 = DOT_DURATION * 3;
const WORD_SPACE: u64 = DOT_DURATION * 7;
const TONE_FREQUENCY: f32 = 600.0;

#[derive(Clone, PartialEq)]
enum MorseSymbol {
    Dot,
    Dash,
    LetterSpace,
    WordSpace,
}

struct MorseTimeClockApp {
    current_time: String,
    morse_display: String,
    is_playing: Arc<Mutex<bool>>,
    last_played_time: String,
}

impl Default for MorseTimeClockApp {
    fn default() -> Self {
        Self {
            current_time: String::new(),
            morse_display: String::new(),
            is_playing: Arc::new(Mutex::new(false)),
            last_played_time: String::new(),
        }
    }
}

fn char_to_morse(c: char) -> Option<Vec<MorseSymbol>> {
    use MorseSymbol::*;
    match c.to_ascii_uppercase() {
        '0' => Some(vec![Dash, Dash, Dash, Dash, Dash]),
        '1' => Some(vec![Dot, Dash, Dash, Dash, Dash]),
        '2' => Some(vec![Dot, Dot, Dash, Dash, Dash]),
        '3' => Some(vec![Dot, Dot, Dot, Dash, Dash]),
        '4' => Some(vec![Dot, Dot, Dot, Dot, Dash]),
        '5' => Some(vec![Dot, Dot, Dot, Dot, Dot]),
        '6' => Some(vec![Dash, Dot, Dot, Dot, Dot]),
        '7' => Some(vec![Dash, Dash, Dot, Dot, Dot]),
        '8' => Some(vec![Dash, Dash, Dash, Dot, Dot]),
        '9' => Some(vec![Dash, Dash, Dash, Dash, Dot]),
        'A' => Some(vec![Dot, Dash]),
        'B' => Some(vec![Dash, Dot, Dot, Dot]),
        'C' => Some(vec![Dash, Dot, Dash, Dot]),
        'D' => Some(vec![Dash, Dot, Dot]),
        'E' => Some(vec![Dot]),
        'F' => Some(vec![Dot, Dot, Dash, Dot]),
        'G' => Some(vec![Dash, Dash, Dot]),
        'H' => Some(vec![Dot, Dot, Dot, Dot]),
        'I' => Some(vec![Dot, Dot]),
        'J' => Some(vec![Dot, Dash, Dash, Dash]),
        'K' => Some(vec![Dash, Dot, Dash]),
        'L' => Some(vec![Dot, Dash, Dot, Dot]),
        'M' => Some(vec![Dash, Dash]),
        'N' => Some(vec![Dash, Dot]),
        'O' => Some(vec![Dash, Dash, Dash]),
        'P' => Some(vec![Dot, Dash, Dash, Dot]),
        'Q' => Some(vec![Dash, Dash, Dot, Dash]),
        'R' => Some(vec![Dot, Dash, Dot]),
        'S' => Some(vec![Dot, Dot, Dot]),
        'T' => Some(vec![Dash]),
        'U' => Some(vec![Dot, Dot, Dash]),
        'V' => Some(vec![Dot, Dot, Dot, Dash]),
        'W' => Some(vec![Dot, Dash, Dash]),
        'X' => Some(vec![Dash, Dot, Dot, Dash]),
        'Y' => Some(vec![Dash, Dot, Dash, Dash]),
        'Z' => Some(vec![Dash, Dash, Dot, Dot]),
        ':' => Some(vec![Dash, Dash, Dash, Dot, Dot, Dot]),
        ' ' => Some(vec![WordSpace]),
        _ => None,
    }
}

fn text_to_morse(text: &str) -> Vec<MorseSymbol> {
    let mut result = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    
    for (i, c) in chars.iter().enumerate() {
        if let Some(mut symbols) = char_to_morse(*c) {
            result.append(&mut symbols);
            if i < chars.len() - 1 && *c != ' ' {
                result.push(MorseSymbol::LetterSpace);
            }
        }
    }
    
    result
}

fn morse_to_display_string(morse: &[MorseSymbol]) -> String {
    morse
        .iter()
        .map(|s| match s {
            MorseSymbol::Dot => "·",
            MorseSymbol::Dash => "−",
            MorseSymbol::LetterSpace => " ",
            MorseSymbol::WordSpace => "   ",
        })
        .collect()
}

// Simple sine wave generator for morse beeps
struct ToneSource {
    frequency: f32,
    sample_rate: u32,
    num_samples: usize,
    current_sample: usize,
}

impl ToneSource {
    fn new(frequency: f32, duration_ms: u64) -> Self {
        let sample_rate = 44100;
        let num_samples = (sample_rate as u64 * duration_ms / 1000) as usize;
        
        Self {
            frequency,
            sample_rate,
            num_samples,
            current_sample: 0,
        }
    }
}

impl Iterator for ToneSource {
    type Item = f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sample >= self.num_samples {
            return None;
        }
        
        let sample = (self.current_sample as f32 * self.frequency * 2.0 * std::f32::consts::PI 
                     / self.sample_rate as f32).sin() * 0.15;
        
        self.current_sample += 1;
        Some(sample)
    }
}

impl Source for ToneSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.num_samples - self.current_sample)
    }
    
    fn channels(&self) -> u16 {
        1
    }
    
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_millis(
            (self.num_samples as u64 * 1000) / self.sample_rate as u64
        ))
    }
}

fn play_morse_code(morse: Vec<MorseSymbol>) {
    thread::spawn(move || {
        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            let sink = Sink::try_new(&stream_handle).unwrap();
            
            for symbol in morse {
                match symbol {
                    MorseSymbol::Dot => {
                        let source = ToneSource::new(TONE_FREQUENCY, DOT_DURATION);
                        sink.append(source);
                        sink.sleep_until_end();
                        thread::sleep(Duration::from_millis(SYMBOL_SPACE));
                    }
                    MorseSymbol::Dash => {
                        let source = ToneSource::new(TONE_FREQUENCY, DASH_DURATION);
                        sink.append(source);
                        sink.sleep_until_end();
                        thread::sleep(Duration::from_millis(SYMBOL_SPACE));
                    }
                    MorseSymbol::LetterSpace => {
                        thread::sleep(Duration::from_millis(LETTER_SPACE));
                    }
                    MorseSymbol::WordSpace => {
                        thread::sleep(Duration::from_millis(WORD_SPACE));
                    }
                }
            }
        }
    });
}

impl eframe::App for MorseTimeClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update time every frame
        let now = Local::now();
        self.current_time = now.format("%H:%M:%S").to_string();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🕐 Morse Time Clock");
            ui.add_space(20.0);
            
            ui.horizontal(|ui| {
                ui.label("Current Time:");
                ui.label(
                    egui::RichText::new(&self.current_time)
                        .size(48.0)
                        .color(egui::Color32::from_rgb(100, 200, 255))
                );
            });
            
            ui.add_space(20.0);
            
            ui.horizontal(|ui| {
                ui.label("Morse Code:");
                ui.label(
                    egui::RichText::new(&self.morse_display)
                        .size(32.0)
                        .color(egui::Color32::from_rgb(255, 200, 100))
                        .family(egui::FontFamily::Monospace)
                );
            });
            
            ui.add_space(30.0);
            
            let is_playing = *self.is_playing.lock().unwrap();
            
            ui.horizontal(|ui| {
                if ui.button("▶ Play Current Time").clicked() && !is_playing {
                    *self.is_playing.lock().unwrap() = true;
                    
                    let morse = text_to_morse(&self.current_time);
                    self.morse_display = morse_to_display_string(&morse);
                    self.last_played_time = self.current_time.clone();
                    
                    let is_playing_clone = Arc::clone(&self.is_playing);
                    thread::spawn(move || {
                        play_morse_code(morse);
                        thread::sleep(Duration::from_secs(1));
                        *is_playing_clone.lock().unwrap() = false;
                    });
                }
                
                if is_playing {
                    ui.label("🔊 Playing...");
                }
            });
            
            ui.add_space(10.0);
            
            if !self.last_played_time.is_empty() {
                ui.label(format!("Last played: {}", self.last_played_time));
            }
            
            ui.add_space(30.0);
            ui.separator();
            ui.add_space(10.0);
            
            ui.label("Morse Code Reference:");
            ui.label("· = dit (short beep)");
            ui.label("− = dah (long beep)");
        });
        
        // Request repaint to keep time updated
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_min_inner_size([500.0, 300.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Morse Time Clock",
        options,
        Box::new(|_cc| Ok(Box::new(MorseTimeClockApp::default()))),
    )
}
