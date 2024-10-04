use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::time::Duration;

pub fn capture_audio() {
    // Initialisation du système hôte
    let host = cpal::default_host();

    // Choisir un périphérique d'entrée audio (microphone)
    let device = host.default_input_device().expect("Aucun périphérique d'entrée trouvé");

    println!("Utilisation du périphérique : {}", device.name().unwrap());

    // Configuration du format de capture
    let config = device.default_input_config().unwrap();

    println!("Format de capture audio : {:?}", config);

    // Créer et démarrer le flux audio
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_input_stream::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => build_input_stream::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => build_input_stream::<u16>(&device, &config.into()),
        _ => panic!("Format d'échantillon non pris en charge !"),
    }.expect("Erreur lors de la création du flux audio");

    // Lancer le flux
    stream.play().unwrap();
    println!("Capture en cours... Appuyez sur Ctrl+C pour arrêter.");

    // Pour maintenir le programme en cours, car le flux fonctionne en arrière-plan.
    std::thread::park();
}

// Fonction utilitaire pour créer un flux audio d'entrée en fonction du format des échantillons
fn build_input_stream<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: cpal::Sample + std::fmt::Debug + cpal::SizedSample, // Ajout de `cpal::SizedSample`
{
    device.build_input_stream(
        config,
        move |data: &[T], _: &cpal::InputCallbackInfo| {
            // Traitement des données audio capturées (ici, juste les afficher)
            process_audio_data(data);
        },
        move |err| {
            eprintln!("Erreur lors de la capture audio : {:?}", err);
        },
        Some(Duration::from_millis(10)), // Ajout d'une durée optionnelle pour la latence
    )
}

fn process_audio_data<T>(data: &[T])
where
    T: cpal::Sample + std::fmt::Debug + cpal::SizedSample,
{
    // Par exemple, ici on va juste afficher les premiers échantillons pour vérifier
    println!("Échantillons audio capturés : {:?}", &data[0..5]);
    // TODO: Transformer ces échantillons en fréquences pour transcription
}
