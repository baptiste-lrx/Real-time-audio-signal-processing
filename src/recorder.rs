// src/recorder.rs

use hound::{WavWriter, WavSpec, SampleFormat};
use std::path::Path;
use std::error::Error;
use std::f32::consts::PI;
use std::io::BufWriter;
use std::fs::File;
use std::sync::{Arc, Mutex};

/// Structure représentant une note active
pub struct ActiveNote {
    pub frequency: f32,
    pub velocity: u8,
    pub phase: f32,
    pub remaining_samples: usize,
}

pub struct Recorder {
    writer: Arc<Mutex<Option<WavWriter<BufWriter<File>>>>>,
    active_notes: Arc<Mutex<Vec<ActiveNote>>>,
    sample_rate: u32,
}

impl Recorder {
    pub fn new(sample_rate: u32, output_path: &str) -> Result<Self, Box<dyn Error>> {
        let spec = WavSpec {
            channels: 1,
            sample_rate: sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        let path = Path::new(output_path);
        let writer = WavWriter::create(path, spec)?;

        Ok(Recorder {
            writer: Arc::new(Mutex::new(Some(writer))),
            active_notes: Arc::new(Mutex::new(Vec::new())),
            sample_rate,
        })
    }

    pub fn add_note(&self, frequency: f32, velocity: u8, duration_ms: u64) {
        let duration_samples = ((duration_ms as f32 / 1000.0) * self.sample_rate as f32) as usize;
        let mut notes = self.active_notes.lock().unwrap();
        notes.push(ActiveNote {
            frequency,
            velocity,
            phase: 0.0,
            remaining_samples: duration_samples,
        });
        println!("Note ajoutée : {} Hz, vélocité {}, durée {} ms", frequency, velocity, duration_ms);
    }

    pub fn generate_sample(active_notes: &Mutex<Vec<ActiveNote>>, sample_rate: u32) -> i16 {
        let mut mixed_sample = 0.0;
        let mut notes_to_remove = Vec::new();

        {
            let mut notes = active_notes.lock().unwrap();
            for (i, note) in notes.iter_mut().enumerate() {
                if note.remaining_samples == 0 {
                    notes_to_remove.push(i);
                    continue;
                }

                // Calcul de l'onde sinusoïdale
                let sample = (2.0 * PI * note.frequency * note.phase / sample_rate as f32).sin();
                mixed_sample += sample * (note.velocity as f32 / 127.0);

                // Mise à jour de la phase et des compteurs
                note.phase += 1.0;
                if note.phase >= sample_rate as f32 / note.frequency {
                    note.phase -= sample_rate as f32 / note.frequency;
                }
                note.remaining_samples -= 1;
            }

            // Supprimer les notes terminées
            for &i in notes_to_remove.iter().rev() {
                notes.remove(i);
                println!("Note terminée et supprimée du Recorder.");
            }
        }

        // Normaliser le mélange si nécessaire
        mixed_sample = mixed_sample.clamp(-1.0, 1.0);

        // Convertir en échantillon 16-bit
        (mixed_sample * i16::MAX as f32) as i16
    }

    pub fn start_recording(&self) {
        let writer = Arc::clone(&self.writer);
        let active_notes = Arc::clone(&self.active_notes);
        let sample_rate = self.sample_rate;

        std::thread::spawn(move || {
            // Définir une durée d'enregistrement (par exemple, 1 minute pour le test)
            let total_samples = sample_rate * 60 * 1; // 1 minute
            println!("Début de l'enregistrement pour {} échantillons.", total_samples);
            for i in 0..total_samples {
                let sample = Recorder::generate_sample(&active_notes, sample_rate);

                // Écrire l'échantillon dans le fichier WAV
                {
                    let mut writer_guard = writer.lock().unwrap();
                    if let Some(ref mut wav_writer) = *writer_guard {
                        if let Err(e) = wav_writer.write_sample(sample) {
                            eprintln!("Erreur lors de l'écriture du fichier WAV: {}", e);
                            break;
                        }
                    }
                }

                // Log pour chaque seconde d'échantillons écrits
                if i % sample_rate == 0 && i != 0 {
                    println!("Échantillons écrits : {}", i);
                }
            }

            // Finaliser l'écriture du fichier WAV
            {
                let mut writer_guard = writer.lock().unwrap();
                if let Some(wav_writer) = writer_guard.take() { // Suppression de `mut`
                    if let Err(e) = wav_writer.finalize() {
                        eprintln!("Erreur lors de la finalisation du fichier WAV: {}", e);
                    } else {
                        println!("Fichier WAV finalisé avec succès.");
                    }
                }
            }

            println!("Enregistrement terminé.");
        });
    }
}
