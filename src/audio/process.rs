// src/audio/process.rs

use std::sync::mpsc::Receiver;
use crate::utils;

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
        AudioProcessor { receiver, note_callback }
    }

    pub fn start(mut self) {
        println!("Démarrage du thread de traitement audio");
        let fft_size = 1024; // Taille de la fenêtre (choisie pour une meilleure résolution)
        let overlap = fft_size / 2; // 50% de recouvrement
        let step_size = fft_size - overlap;
    
        let mut audio_buffer: Vec<i16> = Vec::new();
    
        loop {
            match self.receiver.recv() {
                Ok(samples) => {
                    println!("Échantillons reçus : {} échantillons", samples.len());
                    audio_buffer.extend(samples);
    
                    // Traiter les données si suffisamment d'échantillons
                    while audio_buffer.len() >= fft_size {
                        println!("Traitement d'une fenêtre de taille {}", fft_size);
                    
                        let mut window_samples = audio_buffer[..fft_size].to_vec();
                    
                        // Appliquer une fenêtre (Hann)
                        utils::apply_hann_window_samples(&mut window_samples);
                    
                        // Détecter la fréquence fondamentale avec l'autocorrélation
                        match utils::detect_pitch_autocorrelation(&window_samples, 44100.0) {
                            Some(freq) => {
                                println!("Fréquence détectée : {:.2} Hz", freq);
                            
                                if let Some(note_number) = utils::frequency_to_midi_note_number(freq) {
                                    if note_number >= 21 && note_number <= 108 {
                                        let note_name = utils::midi_note_number_to_name(note_number);
                                        println!("Note détectée : {} (MIDI {})", note_name, note_number);
                    
                                        // Appeler le callback avec le numéro de note MIDI
                                        (self.note_callback)(note_number);
                                    }
                                }
                            },
                            None => {
                                println!("Aucune fréquence détectée");
                            },
                        }
                    
                        audio_buffer.drain(..step_size);
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
