// src/utils.rs

/// Convertit une fréquence en numéro de note MIDI
pub fn frequency_to_midi_note_number(freq: f32) -> Option<u8> {
    if freq <= 0.0 {
        return None;
    }
    let a4 = 440.0;
    let a4_midi = 69.0;
    let midi_num = 12.0 * (freq / a4).log2() + a4_midi;
    let midi_num_rounded = midi_num.round() as i32;
    if midi_num_rounded < 0 || midi_num_rounded > 127 {
        None
    } else {
        Some(midi_num_rounded as u8)
    }
}

/// Convertit un numéro de note MIDI en fréquence (Hz)
pub fn frequency_to_midi_note_number_to_freq(note_number: u8) -> Option<f32> {
    if note_number > 127 {
        return None;
    }
    let a4 = 69.0;
    let a4_freq = 440.0;
    Some(a4_freq * 2.0_f32.powf((note_number as f32 - a4) / 12.0))
}

/// Convertit un numéro de note MIDI en nom de note (ex. C4, A#3)
pub fn midi_note_number_to_name(note_number: u8) -> String {
    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let note_index = note_number % 12;
    let octave = (note_number / 12).saturating_sub(1); // Utiliser saturating_sub pour éviter le sous-dépassement
    let note_name = note_names[note_index as usize];
    format!("{}{}", note_name, octave)
}
