pub mod process;

pub fn process_audio() {
    println!("Traitement du flux audio...");
    // Appel des fonctions du module `process`
    process::capture_audio();
}
