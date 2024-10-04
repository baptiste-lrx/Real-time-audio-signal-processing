use libpulse_binding as pulse;

use pulse::context::{Context, FlagSet as ContextFlagSet};
use pulse::mainloop::standard::{IterateResult, Mainloop};
use pulse::proplist::Proplist;
use pulse::sample::{Format, Spec};
use pulse::stream::{PeekResult, State as StreamState, Stream, FlagSet as StreamFlagSet};
use std::io::{self, BufRead};
// Les imports inutilisés peuvent être supprimés
// use std::sync::{Arc, Mutex};
use libc::{c_int, ioctl, FIONREAD};
use hound; // Import du crate hound

fn main() {
    // Nom de la source PulseAudio
    let source_name = "bluez_source.DC_E5_5B_25_03_64.a2dp_source";

    // Créer le Proplist
    let mut proplist = Proplist::new().unwrap();
    proplist
        .set_str(
            pulse::proplist::properties::APPLICATION_NAME,
            "Audio Capture",
        )
        .unwrap();

    // Créer le Mainloop et le Context
    let mut mainloop = Mainloop::new().expect("Impossible de créer le Mainloop");
    let mut context = Context::new_with_proplist(&mainloop, "Audio Capture Context", &proplist)
        .expect("Impossible de créer le Context");

    // Se connecter au serveur PulseAudio
    context
        .connect(None, ContextFlagSet::NOFLAGS, None)
        .expect("Impossible de se connecter au serveur PulseAudio");

    // Attendre que le contexte soit prêt
    loop {
        match mainloop.iterate(false) {
            IterateResult::Success(_) => match context.get_state() {
                pulse::context::State::Ready => break,
                pulse::context::State::Failed | pulse::context::State::Terminated => {
                    eprintln!("Le contexte PulseAudio a échoué ou a été terminé");
                    return;
                }
                _ => {}
            },
            IterateResult::Quit(_) | IterateResult::Err(_) => {
                eprintln!("Erreur lors de l'itération du mainloop");
                return;
            }
        }
    }

    // Spécification du format audio
    let spec = Spec {
        format: Format::S16le, // 'le' en minuscule
        channels: 2,
        rate: 44100,
    };
    assert!(spec.is_valid());

    // Créer un nouveau Stream audio
    let mut stream = Stream::new(&mut context, "Audio Capture Stream", &spec, None)
        .expect("Impossible de créer le Stream");

    // Connecter le Stream en mode enregistrement
    stream
        .connect_record(Some(source_name), None, StreamFlagSet::NOFLAGS)
        .expect("Impossible de connecter le Stream pour l'enregistrement");

    // Attendre que le Stream soit prêt
    loop {
        match mainloop.iterate(false) {
            IterateResult::Success(_) => match stream.get_state() {
                StreamState::Ready => break,
                StreamState::Failed | StreamState::Terminated => {
                    eprintln!("Le Stream a échoué ou a été terminé");
                    return;
                }
                _ => {}
            },
            IterateResult::Quit(_) | IterateResult::Err(_) => {
                eprintln!("Erreur lors de l'itération du mainloop");
                return;
            }
        }
    }

    // Démarrer le Stream en utilisant trigger
    
    // Préparer l'entrée utilisateur pour arrêter l'enregistrement
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();

    // Créer le writer WAV
    let wav_spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut wav_writer = hound::WavWriter::create("enregistrement.wav", wav_spec)
        .expect("Impossible de créer le fichier WAV");

    println!("Enregistrement en cours... Appuyez sur Entrée pour arrêter.");

    // Boucle principale
    loop {
        match mainloop.iterate(true) {
            IterateResult::Success(_) => {
                // Lire les données du Stream
                if let Some(_) = stream.readable_size() {
                    match stream.peek() {
                        Ok(PeekResult::Data(data)) => {
                            let samples = unsafe {
                                std::slice::from_raw_parts(
                                    data.as_ptr() as *const i16,
                                    data.len() / std::mem::size_of::<i16>(),
                                )
                            };
                            // Écrire les échantillons dans le fichier WAV
                            for &sample in samples {
                                wav_writer
                                    .write_sample(sample)
                                    .expect("Erreur lors de l'écriture du fichier WAV");
                            }
                            stream.discard().unwrap();
                            println!("Nombre d'échantillons capturés : {}", samples.len());
                        }
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("Erreur lors de la lecture des données : {:?}", err);
                        }
                    }
                }
            }
            IterateResult::Quit(_) | IterateResult::Err(_) => {
                eprintln!("Erreur lors de l'itération du mainloop");
                break;
            }
        }

        // Vérifier si l'utilisateur a appuyé sur Entrée pour arrêter l'enregistrement
        if handle_has_data(&handle) {
            handle.read_line(&mut input).unwrap();
            break;
        }
    }

    // Fermer le writer WAV
    wav_writer
        .finalize()
        .expect("Erreur lors de la fermeture du fichier WAV");

    // Arrêter le Stream et le Mainloop
    stream.disconnect().unwrap();
    mainloop.quit(pulse::def::Retval(0));

    println!("Arrêt de l'enregistrement.");
}

// Fonction pour vérifier s'il y a des données disponibles sur stdin sans bloquer
fn handle_has_data(handle: &io::StdinLock<'_>) -> bool {
    use std::os::unix::io::AsRawFd;
    let fd = handle.as_raw_fd();
    let mut bytes: c_int = 0;
    unsafe {
        ioctl(fd, FIONREAD, &mut bytes);
    }
    bytes > 0
}
