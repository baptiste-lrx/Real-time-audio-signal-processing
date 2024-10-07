// src/midi/transcription.rs

use midir::{MidiOutput, MidiOutputConnection};
use std::error::Error;
use std::thread;
use std::time::Duration;

pub struct MidiTranscriber {
    conn_out: MidiOutputConnection,
}

impl MidiTranscriber {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Initialiser la sortie MIDI
        let midi_out = MidiOutput::new("My MIDI Output")?; 
        // Choisir le port de sortie MIDI (par exemple, le premier disponible)
        let out_ports = midi_out.ports();
        if out_ports.is_empty() {
            return Err("Aucun port de sortie MIDI disponible".into());
        }

        // Pour une application réelle, vous devriez permettre à l'utilisateur de choisir le port
        let port = &out_ports[0];
        let conn_out = midi_out.connect(port, "midir-test")?;

        Ok(MidiTranscriber { conn_out })
    }

    pub fn play_note(&mut self, note: u8, velocity: u8, duration_ms: u64) -> Result<(), Box<dyn Error>> {
        // Note ON
        self.conn_out.send(&[0x90, note, velocity])?;
        // Attendre la durée spécifiée
        thread::sleep(Duration::from_millis(duration_ms));
        // Note OFF
        self.conn_out.send(&[0x80, note, 0])?;
        Ok(())
    }
}
