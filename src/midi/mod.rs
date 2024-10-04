pub mod transcription;

pub fn transcribe_midi() {
    println!("Transcription des notes MIDI...");
    // Appel des fonctions du module `transcription`
    transcription::convert_audio_to_midi();
}
