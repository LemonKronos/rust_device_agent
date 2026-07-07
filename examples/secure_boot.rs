use std::fs;

fn check_secure_boot() -> bool {
    let path = "/sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c";
    
    // Read the file as raw bytes. The 5th byte (index 4) holds the 1 or 0.
    match fs::read(path) {
        Ok(bytes) if bytes.len() >= 5 => bytes[4] == 1,
        _ => false, // Failsafe: if missing or unreadable, assume disabled
    }
}

fn main() {
    println!("Secure Boot Enabled: {}", check_secure_boot());
}