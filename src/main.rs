// src/main.rs

mod audio;
mod utils;

use audio::capture::AudioCapturer;
use audio::process::AudioProcessor;
use std::sync::mpsc::channel;

fn main() {
    // Nom de la source PulseAudio
    let source_name = "bluez_source.DC_E5_5B_25_03_64.a2dp_source";

    // Créer un canal pour transmettre les échantillons audio
    let (sender, receiver) = channel();

    // Initialiser le module de capture audio
    let audio_capturer = AudioCapturer::new(source_name);
    audio_capturer.start(sender);

    // Initialiser le module de traitement audio
    let audio_processor = AudioProcessor::new(receiver);
    audio_processor.start();

    // Le reste du programme (MIDI, LEDs, etc.)
}
