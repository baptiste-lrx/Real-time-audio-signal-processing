pub mod control;

pub fn control_leds() {
    println!("Contrôle des LEDs en fonction des notes...");
    // Appel des fonctions du module `control`
    control::update_leds();
}
