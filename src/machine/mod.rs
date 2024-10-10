pub mod rotor;     // Submodule rotor
pub mod reflector; // Submodule reflector

pub fn run() {
    println!("Starting the Enigma Machine...");
    rotor::initialize();
    reflector::initialize();
}
