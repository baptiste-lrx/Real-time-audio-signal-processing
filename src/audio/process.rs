// src/audio/process.rs

use std::sync::mpsc::Receiver;
use rustfft::{FftPlanner, num_complex::Complex};
use crate::utils; // Import correct du module utils

pub struct AudioProcessor<F>
where
    F: FnMut(u8) + Send + 'static,
{
    receiver: Receiver<Vec<i16>>,
    note_callback: F,
}

impl<F> AudioProcessor<F>
where
    F: FnMut(u8) + Send + 'static,
{
    pub fn new(receiver: Receiver<Vec<i16>>, note_callback: F) -> Self {
        AudioProcessor {
            receiver,
            note_callback,
        }
    }

    pub fn start(mut self) {
        println!("Démarrage du thread de traitement audio");
        loop {
            match self.receiver.recv() {
                Ok(samples) => {
                    println!("Échantillons reçus : {} échantillons", samples.len());

                    // Appliquer une fenêtre de Hamming pour réduire les effets de fuite spectrale
                    let mut windowed_samples = samples.clone();
                    apply_hamming_window(&mut windowed_samples);

                    // Détecter la fréquence fondamentale avec FFT
                    if let Some(freq) = detect_pitch_fft(&windowed_samples, 44100.0) {
                        println!("Fréquence détectée : {:.2} Hz", freq);

                        if let Some(note_number) = utils::frequency_to_midi_note_number(freq) {
                            if note_number >= 21 && note_number <= 108 {
                                let note_name = utils::midi_note_number_to_name(note_number);
                                println!("Note détectée : {} (MIDI {})", note_name, note_number);

                                // Appeler le callback avec le numéro de note MIDI
                                (self.note_callback)(note_number);
                            } else {
                                println!("Note MIDI en dehors de la plage de piano : {}", note_number);
                            }
                        } else {
                            println!("Conversion de fréquence en note MIDI a échoué");
                        }
                    } else {
                        println!("Aucune fréquence détectée");
                    }
                }
                Err(_) => {
                    eprintln!("Le canal a été fermé");
                    break;
                }
            }
        }
    }
}

/// Applique une fenêtre de Hamming aux échantillons
fn apply_hamming_window(samples: &mut [i16]) {
    let len = samples.len() as f32;
    for (n, sample) in samples.iter_mut().enumerate() {
        let window = 0.54 - 0.46 * (2.0 * std::f32::consts::PI * n as f32 / (len - 1.0)).cos();
        *sample = ((*sample as f32) * window).round() as i16;
    }
}

/// Fonction de détection de pitch avec FFT
fn detect_pitch_fft(samples: &[i16], sample_rate: f32) -> Option<f32> {
    let len = samples.len();
    if len == 0 {
        return None;
    }

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(len);

    // Convertir les échantillons en complexes
    let mut buffer: Vec<Complex<f32>> = samples.iter().map(|&s| Complex::new(s as f32, 0.0)).collect();
    // Appliquer FFT
    fft.process(&mut buffer);

    // Calculer les amplitudes
    let amplitudes: Vec<f32> = buffer.iter().map(|c| c.norm()).collect();

    // Trouver le pic maximal dans les amplitudes (exclure la DC component)
    let max_index = amplitudes[1..]
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i + 1)?;

    // Calculer la fréquence correspondante
    let freq = max_index as f32 * sample_rate / len as f32;
    Some(freq)
}
