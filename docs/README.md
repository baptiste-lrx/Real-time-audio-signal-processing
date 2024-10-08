
# Real-Time Audio Processing with Pitch Detection

Welcome to the **Real-Time Audio Processing with Pitch Detection** project. This Rust project captures audio in real time, analyzes the audio signals to detect the notes played, and converts these notes into the corresponding MIDI numbers.

## Table of contents

- [Description](#description)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Use](#use)
- [Implementation details](#implementation-details)
- [Troubleshooting](#troubleshooting)
- [Contribute](#contribute)
- [License](#license)
- [Acknowledgements](#acknowledgements)

## Description

This project is a real-time audio processing application developed in Rust. It captures the audio stream from the microphone, analyzes the data to detect the fundamental frequencies of the notes played, and converts these frequencies into corresponding MIDI note numbers. The application is particularly useful for musical applications such as computer-aided composition tools or music transcription systems.

## Features

- **Realtime Audio Capture**: Use `cpal` to capture audio from the microphone.
- **Pitch Detection**: Implementation of the YIN algorithm via the `rust_yin` library for accurate detection of fundamental frequencies.
- **Frequency → MIDI note conversion**: Conversion of detected frequencies into corresponding MIDI note numbers.
- **Audio filtering**: Application of low-pass filters to improve pitch detection accuracy.
- **Detailed logging**: Display of debugging information for monitoring audio processing.

## Prerequisites

Before you start, please make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (stable version)
- A compatible development environment (text editor or IDE)
- Access to a microphone for audio capture

## Installation

1. **Click on the Project Directory**.

   ```bash
   git clone https://github.com/votre_nom_utilisateur/real_time_audio_signal_processing.git
   cd real_time_audio_signal_processing
   ```

2. **Configuring dependencies**

   Make sure your `Cargo.toml` file contains the necessary dependencies:

   ```toml
   [package]
   name = “real_time_audio_signal_processing”
   version = “0.1.0
   edition = “2021”

   [dependencies]
   cpal = “0.15.0”
   rust_yin = “0.1.0” # Check the latest version on [crates.io](https://crates.io/crates/rust_yin)
   biquad = “0.3.1”
   ```

3. **Compile the Project**

   ```bash
   cargo build
   ```

   If you encounter compilation errors related to permissions or dependencies, be sure to follow the troubleshooting steps below.

## Usage

1. **Launch Application**

   ```bash
   cargo run
   ```

2. **Test Note Detection**

   - Play a single note on an instrument (e.g. a piano).
   - Observe the outputs in the terminal, which indicate the detected frequency and the corresponding MIDI note.

   Example output:

   ```
   Start audio processing thread
   Audio data received: 2560 bytes
   First sample: 997
   Received samples: 640 samples
   Processing a window of size 1024
   Frequency detected: 440.00 Hz
   Note detected: A4 (MIDI 69)
   ```

## Implementation details

### Audio Capture

The project uses the `cpal` crate to capture audio in real time from the microphone. Audio data is read in bytes, converted to `i16` samples, and then processed for pitch detection.

### Pitch detection with YIN

The YIN algorithm is implemented via the `rust_yin` library. This algorithm is renowned for its accuracy in detecting fundamental frequencies, even in the presence of noise and harmonics.

```rust
use rust_yin::Yin;

pub fn detect_pitch(samples: &[i16], sample_rate: f32) -> Option<f32> {
    let samples_f32: Vec<f32> = samples.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
    let mut yin = Yin::new(sample_rate as usize, samples_f32.len());
    match yin.get_pitch(&samples_f32) {
        Some(pitch) if pitch > 0.0 => Some(pitch),
        _ => None,
    }
}
```

### Frequency → MIDI note conversion

Detected frequencies are converted to MIDI note numbers using the standard formula:

```rust
pub fn frequency_to_midi_note_number(freq: f32) -> Option<u8> {
    if freq <= 0.0 {
        return None;
    }
    let midi_number = 69.0 + 12.0 * (freq / 440.0).log2();
    Some(midi_number.round() as u8)
}
```

### Audio filtering

A low-pass filter is applied to audio samples to attenuate high frequencies and reduce unwanted harmonics, thus improving pitch detection accuracy.

```rust
use biquad::{Biquad, Coefficients, ToHertz, Type::LowPass};

// Create the low-pass filter
let cutoff_freq = 2000.0;
let sample_rate = 44100.0;
let coefficients = Coefficients::<f32>::from_params(
    LowPass,
    sample_rate.hz(),
    cutoff_freq.hz(),
    0.707, // Q factor
).unwrap();
let mut biquad = Biquad::from_coefficients(coeffs);

// Apply filter
let filtered_samples: Vec<f32> = window_samples
    .iter()
    .map(|&sample| biquad.run(sample as f32))
    .collect();
```

## Troubleshooting

### Compilation errors with `aubio

If you have encountered errors when using the `aubio` library, we recommend switching to an alternative such as `rust_yin`, which is better integrated with Rust.

### File permissions

If you can't modify project files, check file and directory permissions:

```bash
ls -l src/
chmod u+w src/*.rs
```

Also check that your code editor or IDE has the necessary permissions to write to the project directory.

### Pitch detection problems

- No frequency detected** : Make sure the microphone is working properly and that the input volume is sufficient.
- Incorrect frequencies detected**: Check audio filtering and sample normalization. Use a test signal (such as a frequency generator) to validate the algorithm.

## Contribute

Contributions are welcome! If you would like to improve this project, please follow these steps:

1. **Fork the Directory**
2. **Create a Functionality Branch**
   ```bash
   git checkout -b feature/improvement
   ```

3. **Commit your changes**

   ```bash
   git commit -m “Add YIN algorithm for pitch detection”
   ```

4. **Push the Branch**

   ```bash
   git push origin feature/improvement
   ```

5. **Open a Pull Request**

## License

This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.

## Acknowledgements

- **Crates Used**:
  - [`cpal`](https://crates.io/crates/cpal) for audio capture.
  - [`rust_yin`](https://crates.io/crates/rust_yin) for pitch detection.
  - [`biquad`](https://crates.io/crates/biquad) for audio filtering.
- Rust Community** for support and useful resources.
- aubio** for their audio processing tools (although we have chosen an alternative).

---

*This README has been generated to help you better structure and document your real-time audio processing project with pitch detection in Rust. Feel free to adapt it to your project's specificities and evolutions*.
