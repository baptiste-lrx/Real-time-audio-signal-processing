use cpal::traits::{DeviceTrait, HostTrait};
use std::sync::{Arc, Mutex};
use std::error::Error;

pub fn capture_audio() -> Result<(), Box<dyn Error>> {
    // Initialise l'hôte audio par défaut
    let host = cpal::default_host();

    // Sélectionne le périphérique "monitor" de PulseAudio
    let device = host
        .devices()?
        .filter(|d| d.name().unwrap_or_default().contains("Monitor"))  // On filtre le périphérique qui contient "Monitor"
        .next()
        .expect("Aucun périphérique monitor trouvé.");

    // Récupération de la configuration par défaut du périphérique
    let config = device.default_input_config()?;

    // Crée une structure partagée pour stocker les données audio capturées
    let audio_data: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));

    // Crée le flux d'entrée
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_input_stream::<f32>(&device, &config.into(), audio_data.clone())?,
        cpal::SampleFormat::I16 => {
            // Change audio_data pour correspondre au type i16
            let audio_data_i16: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
            build_input_stream::<i16>(&device, &config.into(), audio_data_i16.clone())?
        },
        cpal::SampleFormat::U16 => {
            // Change audio_data pour correspondre au type u16
            let audio_data_u16: Arc<Mutex<Vec<u16>>> = Arc::new(Mutex::new(Vec::new()));
            build_input_stream::<u16>(&device, &config.into(), audio_data_u16.clone())?
        },
        _ => panic!("Format d'échantillons non supporté"),
    };

    // Lecture du stream pour démarrer la capture audio
    println!("Capture audio en cours...");

    // Démarre le flux
    stream.play()?;

    // Attendre indéfiniment ou exécuter d'autres tâches
    std::thread::sleep(std::time::Duration::from_secs(10));

    Ok(())
}

// Fonction générique pour gérer la capture de différents formats d'échantillons
fn build_input_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    audio_data: Arc<Mutex<Vec<T>>>,
) -> Result<cpal::Stream, Box<dyn Error>>
where
    T: cpal::Sample + std::fmt::Debug + Send + 'static + cpal::SizedSample,
{
    let stream = device.build_input_stream(
        config,
        move |data: &[T], _: &cpal::InputCallbackInfo| {
            // Ici, les données capturées sont mises à jour en temps réel
            let mut audio_buffer = audio_data.lock().unwrap();

            // Ajoute les nouvelles données au buffer audio
            audio_buffer.extend_from_slice(data);

            // Affiche un lot d'échantillons capturés pour vérifier que les données sont bien mises à jour
            println!("Nouveau lot de données audio : {:?}", &data[0..std::cmp::min(10, data.len())]);
        },
        move |err| {
            eprintln!("Erreur de capture audio : {:?}", err);
        },
        None,
    )?;

    Ok(stream)
}
