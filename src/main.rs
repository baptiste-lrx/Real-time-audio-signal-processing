// src/main.rs

mod audio;
mod midi;
mod utils;
mod recorder;

use audio::capture::AudioCapturer;
use audio::process::AudioProcessor;
use midi::transcription::MidiTranscriber;
use recorder::Recorder;
use std::error::Error;
use std::sync::mpsc::channel;

fn main() -> Result<(), Box<dyn Error>> {
    // Nom de la source PulseAudio ou ALSA (à ajuster selon votre configuration)
    let source_name = "hw:0,0"; // Remplacez par votre source de capture correcte

    // Créer un canal pour transmettre les échantillons audio
    let (sender, receiver) = channel();

    // Initialiser le module de capture audio
    let audio_capturer = AudioCapturer::new(source_name);
    audio_capturer.start(sender)?; // Assurez-vous que `start` retourne un Result

    // Initialiser le transcripteur MIDI avec le canal 0 (canal 1 MIDI)
    let mut midi_transcriber = MidiTranscriber::new(0)?; // Canal 0

    // Initialiser l'enregistreur de notes avec un taux d'échantillonnage de 44100 Hz et un fichier de sortie unique
    let recorder = Recorder::new(44100, "recorded_notes_all.wav")?;
    recorder.start_recording();

    // Initialiser le module de traitement audio
    let audio_processor = AudioProcessor::new(receiver, move |note_number| {
        // Convertir le numéro de note MIDI en fréquence
        if let Some(freq) = utils::frequency_to_midi_note_number_to_freq(note_number) {
            // Ajouter la note au Recorder
            recorder.add_note(freq, 100, 500); // Vélocité 100, durée 500 ms
        }

        // Jouer la note via FluidSynth
        if let Err(err) = midi_transcriber.play_note(note_number, 100, 500) {
            eprintln!("Erreur lors de la lecture de la note MIDI : {}", err);
        }
    });
    audio_processor.start();

    Ok(())
}
