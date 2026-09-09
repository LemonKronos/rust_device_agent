use ed25519_dalek::{Signer, SigningKey};
use rand::{Rng, SeedableRng}; // Swapped Rng for RngCore
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::fs;

fn main() {
    println!("--- Ed25519 Mock Dev Signer ---\n");

    // 1. Generate a completely reproducible secret using a fixed seed
    // By using [0x42; 32], you will get the exact same keys every single time.
    let mut rng = rand::rngs::StdRng::from_seed([0x42; 32]);
    let mut secret = [0u8; 32];
    
    // Fill the array with reproducible random bytes
    rng.fill_bytes(&mut secret); // Changed to fill_bytes
    
    // Construct the signing key from those raw bytes
    let signing_key = SigningKey::from_bytes(&secret);
    let verifying_key = signing_key.verifying_key();

    // 2. Format the Public Key exactly how you need it for versioning/mod.rs
    let pub_bytes = verifying_key.to_bytes();
    println!("1. Copy this into your launcher's versioning code:");
    print!("pub const SERVER_PUB_KEY: [u8; 32] = [\n    ");
    for (i, b) in pub_bytes.iter().enumerate() {
        print!("0x{:02x}, ", b);
        if (i + 1) % 8 == 0 && i != 31 {
            print!("\n    ");
        }
    }
    println!("\n];\n");

    // 3. Read the dummy binary you put in /tmp/
    let target_file = "/tmp/gsoft-agent/main_worker";
    let file_bytes = match fs::read(target_file) {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("FATAL: Could not read {}: {}", target_file, e);
            println!("Make sure you created the dummy file before running this!");
            return;
        }
    };

    // 4. Cryptographically sign the raw bytes
    let signature = signing_key.sign(&file_bytes);
    let sig_base64 = BASE64.encode(signature.to_bytes());

    println!("2. Copy this Base64 signature to your server JSON response/IPC payload:");
    println!("{}\n", sig_base64);
    
    println!("Mock IPC Payload Example:");
    println!("UPDATE|main_worker|{}|{}", target_file, sig_base64);
}