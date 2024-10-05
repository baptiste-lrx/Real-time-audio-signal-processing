// src/utils.rs

use rustfft::num_complex::Complex;
use std::f32::consts::PI;

pub fn apply_hann_window(buffer: &mut [Complex<f32>]) {
    let len = buffer.len();
    for (n, c) in buffer.iter_mut().enumerate() {
        let multiplier = 0.5 * (1.0 - (2.0 * PI * n as f32 / (len as f32 - 1.0)).cos());
        c.re *= multiplier;
        c.im *= multiplier;
    }
}

pub fn detect_fundamental_frequency(magnitudes: &[f32], sample_rate: f32) -> Option<f32> {
    // Trouver l'indice avec la magnitude maximale
    let max_index = magnitudes
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)?;

    let fft_size = magnitudes.len() * 2; // Taille de la FFT
    let freq_resolution = sample_rate / fft_size as f32;

    Some(max_index as f32 * freq_resolution)
}
pub fn frequency_to_note(freq: f32) -> String {
    if freq <= 0.0 {
        return String::from("Fréquence invalide");
    }

    let a4_freq = 440.0;
    let note_number = 12.0 * ((freq / a4_freq).log2()) + 69.0;
    let note_index = note_number.round() as i32;

    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    let octave = (note_index / 12) - 1;
    let note_idx = ((note_index % 12) + 12) % 12;
    let note_name = note_names[note_idx as usize];

    format!("{}{}", note_name, octave)
}
