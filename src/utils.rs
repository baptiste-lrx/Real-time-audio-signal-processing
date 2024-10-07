// src/utils.rs

use std::f32::consts::PI;


pub fn frequency_to_midi_note_number(freq: f32) -> Option<u8> {
    if freq <= 0.0 {
        return None;
    }
    let a4_freq = 440.0;
    let note_number = 12.0 * ((freq / a4_freq).log2()) + 69.0;
    let note_index = note_number.round() as i32;

    if note_index < 0 || note_index > 127 {
        return None;
    }
    Some(note_index as u8)
}


pub fn apply_hann_window_samples(samples: &mut [i16]) {
    let len = samples.len();
    for (n, sample) in samples.iter_mut().enumerate() {
        let multiplier = 0.5 * (1.0 - (2.0 * PI * n as f32 / (len as f32 - 1.0)).cos());
        *sample = (*sample as f32 * multiplier) as i16;
    }
}


pub fn detect_pitch_autocorrelation(samples: &[i16], sample_rate: f32) -> Option<f32> {
    // Convertir les échantillons en f32
    let samples: Vec<f32> = samples.iter().map(|&s| s as f32).collect();
    let size = samples.len();

    // Calcul de l'autocorrélation
    let mut autocorrelation = vec![0.0; size];

    // Soustraire la moyenne pour éliminer la composante DC
    let mean = samples.iter().sum::<f32>() / size as f32;
    let samples: Vec<f32> = samples.iter().map(|&s| s - mean).collect();

    // Calcul de l'autocorrélation normalisée
    for lag in 0..size {
        let mut sum = 0.0;
        for i in 0..(size - lag) {
            sum += samples[i] * samples[i + lag];
        }
        autocorrelation[lag] = sum;
    }

    // Trouver le premier minimum local pour éviter les faux pics
    let mut d = 0;
    while d < size - 1 && autocorrelation[d] >= autocorrelation[d + 1] {
        d += 1;
    }

    // Trouver le pic après le minimum
    let mut max_pos = d;
    let mut max_val = autocorrelation[d];
    for i in (d + 1)..size {
        if autocorrelation[i] > max_val {
            max_val = autocorrelation[i];
            max_pos = i;
        }
    }

    if max_pos == 0 {
        return None;
    }

    // Calculer la fréquence fondamentale
    let fundamental_freq = sample_rate / max_pos as f32;
    Some(fundamental_freq)
}


pub fn midi_note_number_to_name(note_number: u8) -> String {
    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let note_index = note_number % 12;
    let octave = (note_number / 12) - 1;
    let note_name = note_names[note_index as usize];
    format!("{}{}", note_name, octave)
}