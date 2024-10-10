use midir::{MidiOutput, MidiOutputConnection};
use std::error::Error;
use std::thread;
use std::time::Duration;

pub struct MidiTranscriber {
    conn_out: MidiOutputConnection,
    channel: u8, // Canal MIDI (0-15)
}

impl MidiTranscriber {
    pub fn new(channel: u8) -> Result<Self, Box<dyn Error>> {
        if channel > 15 {
            return Err("Le canal MIDI doit être compris entre 0 et 15.".into());
        }

        // Initialiser la sortie MIDI
        let midi_out = MidiOutput::new("My MIDI Output")?; 
        // Choisir le port de sortie MIDI (par exemple, FluidSynth)
        let out_ports = midi_out.ports();
        if out_ports.is_empty() {
            return Err("Aucun port de sortie MIDI disponible".into());
        }

        // Afficher les ports disponibles pour choisir le bon
        for (i, port) in out_ports.iter().enumerate() {
            println!("Port {}: {}", i, midi_out.port_name(port)?);
        }

        // Demander à l'utilisateur de choisir le port
        println!("Entrez le numéro du port MIDI à utiliser:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let port_index: usize = input.trim().parse()?;
        if port_index >= out_ports.len() {
            return Err("Index de port invalide".into());
        }

        let port = &out_ports[port_index];
        let conn_out = midi_out.connect(port, "midir-test")?;

        Ok(MidiTranscriber { conn_out, channel })
    }

    pub fn play_note(&mut self, note: u8, velocity: u8, duration_ms: u64) -> Result<(), Box<dyn Error>> {
        // Construire les messages Note ON et Note OFF en spécifiant le canal
        let note_on = 0x90 + self.channel; // 0x90 est le code pour Note ON sur le canal 1
        let note_off = 0x80 + self.channel; // 0x80 est le code pour Note OFF sur le canal 1

        // Note ON
        self.conn_out.send(&[note_on, note, velocity])?;
        // Attendre la durée spécifiée
        thread::sleep(Duration::from_millis(duration_ms));
        // Note OFF
        self.conn_out.send(&[note_off, note, 0])?;
        Ok(())
    }
}
