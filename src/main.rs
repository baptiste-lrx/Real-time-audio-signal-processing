mod audio;
mod midi;
mod leds;

fn main() {
    println!("Démarrage du projet de traitement audio en temps réel !");

    // Initialiser les modules
    audio::process_audio();
    midi::transcribe_midi();
    leds::control_leds();
}
