// src/audio/capture.rs

use libpulse_binding as pulse;

use pulse::context::{Context, FlagSet as ContextFlagSet};
use pulse::mainloop::standard::{IterateResult, Mainloop};
use pulse::proplist::Proplist;
use pulse::sample::{Format, Spec};
use pulse::stream::{PeekResult, State as StreamState, Stream, FlagSet as StreamFlagSet};
use std::sync::mpsc::Sender;
use std::thread;
use hound;

pub struct AudioCapturer {
    source_name: String,
}

impl AudioCapturer {
    pub fn new(source_name: &str) -> Self {
        AudioCapturer {
            source_name: source_name.to_string(),
        }
    }

    pub fn start(&self, sender: Sender<Vec<i16>>) {
        let source_name = self.source_name.clone();
        thread::spawn(move || {
            // Le code de capture audio

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
            let mut context =
                Context::new_with_proplist(&mainloop, "Audio Capture Context", &proplist)
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
                format: Format::S16le,
                channels: 2,
                rate: 44100,
            };
            assert!(spec.is_valid());

            // Créer un nouveau Stream audio
            let mut stream =
                Stream::new(&mut context, "Audio Capture Stream", &spec, None)
                    .expect("Impossible de créer le Stream");

            // Connecter le Stream en mode enregistrement
            stream
                .connect_record(Some(&source_name), None, StreamFlagSet::NOFLAGS)
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

            // Configuration du writer WAV
            let wav_spec = hound::WavSpec {
                channels: 2,
                sample_rate: 44100,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };

            let mut wav_writer = hound::WavWriter::create("enregistrement.wav", wav_spec)
                .expect("Impossible de créer le fichier WAV");

            // Boucle principale de capture
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

                                    // Envoyer les échantillons au processeur
                                    if sender.send(samples.to_vec()).is_err() {
                                        eprintln!("Le récepteur a été déconnecté");
                                        break;
                                    }

                                    // Écrire les échantillons dans le fichier WAV
                                    for &sample in samples {
                                        wav_writer
                                            .write_sample(sample)
                                            .expect("Erreur lors de l'écriture du fichier WAV");
                                    }

                                    stream.discard().unwrap();
                                }
                                Ok(_) => {}
                                Err(err) => {
                                    eprintln!(
                                        "Erreur lors de la lecture des données : {:?}",
                                        err
                                    );
                                }
                            }
                        }
                    }
                    IterateResult::Quit(_) | IterateResult::Err(_) => {
                        eprintln!("Erreur lors de l'itération du mainloop");
                        break;
                    }
                }
            }

            // Fermer le writer WAV
            wav_writer
                .finalize()
                .expect("Erreur lors de la fermeture du fichier WAV");

            // Arrêter le Stream et le Mainloop
            stream.disconnect().unwrap();
            mainloop.quit(pulse::def::Retval(0));
        });
    }
}
