// src/main.rs

mod audio;
mod midi;
mod utils;

use audio::capture::AudioCapturer;
use audio::process::AudioProcessor;
use midi::transcription::MidiTranscriber;
use std::error::Error;
use std::sync::mpsc::channel;

fn main() -> Result<(), Box<dyn Error>> {
    // Nom de la source PulseAudio
    let source_name = "bluez_source.DC_E5_5B_25_03_64.a2dp_source";

    // Créer un canal pour transmettre les échantillons audio
    let (sender, receiver) = channel();

    // Initialiser le module de capture audio
    let audio_capturer = AudioCapturer::new(source_name);
    audio_capturer.start(sender);

    // Initialiser le transcripteur MIDI
    let mut midi_transcriber = MidiTranscriber::new()?;

    // Initialiser le module de traitement audio
    let audio_processor = AudioProcessor::new(receiver, move |note_number| {
        // Callback appelé lorsque une note est détectée
        // Envoyer la note MIDI
        if let Err(err) = midi_transcriber.play_note(note_number, 100, 500) {
            eprintln!("Erreur lors de la lecture de la note MIDI : {}", err);
        }
    });
    audio_processor.start();

    Ok(())
}