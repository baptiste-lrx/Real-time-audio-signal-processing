// src/audio/process.rs

use rustfft::{num_complex::Complex, FftPlanner};
use std::sync::mpsc::Receiver;
use crate::utils;

pub struct AudioProcessor {
    receiver: Receiver<Vec<i16>>,
}

impl AudioProcessor {
    pub fn new(receiver: Receiver<Vec<i16>>) -> Self {
        AudioProcessor { receiver }
    }

    pub fn start(&self) {
        let mut fft_planner = FftPlanner::<f32>::new();
        let fft_size = 1024; // Taille de la FFT
        let fft = fft_planner.plan_fft_forward(fft_size);

        let mut audio_buffer: Vec<i16> = Vec::new();

        loop {
            match self.receiver.recv() {
                Ok(samples) => {
                    audio_buffer.extend(samples);

                    // Traiter les données si suffisamment d'échantillons
                    while audio_buffer.len() >= fft_size {
                        let window_samples = audio_buffer.drain(..fft_size).collect::<Vec<_>>();

                        // Convertir en nombres complexes
                        let mut fft_input: Vec<Complex<f32>> = window_samples
                            .iter()
                            .map(|&s| Complex {
                                re: s as f32,
                                im: 0.0,
                            })
                            .collect();

                        // Appliquer une fenêtre (Hann)
                        utils::apply_hann_window(&mut fft_input);

                        // Exécuter la FFT
                        fft.process(&mut fft_input);

                        // Calculer les magnitudes
                        let magnitudes: Vec<f32> = fft_input.iter().map(|c| c.norm()).collect();

                        // Détecter la fréquence fondamentale
                        if let Some(freq) = utils::detect_fundamental_frequency(&magnitudes, 44100.0) {
                            let note = utils::frequency_to_note(freq);
                            println!("Note détectée : {}", note);

                            // Ici, vous pouvez transmettre la note au module MIDI ou LEDs
                        }
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
