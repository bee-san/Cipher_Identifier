fn main() {
    // Run the cipher analyzer CLI
    if let Err(e) = cipher_identifier::cipher_analyzer::main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
