//!
//! Binary management: update, rollback, promote.
//! 

use std::fs;
use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use shared_libs::path::AgentPath;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

/// Hardcoded Public Key 
const SERVER_PUB_KEY: [u8; 32] = [
    0xd2, 0x6c, 0x4e, 0x8e, 0x15, 0x33, 0x45, 0xff, 
    0x60, 0xf8, 0x6c, 0x03, 0xc6, 0x73, 0xb8, 0x4d, 
    0x15, 0x2b, 0x2d, 0x13, 0xee, 0x24, 0x13, 0x41, 
    0x0f, 0xb9, 0x12, 0xf1, 0x42, 0x67, 0xc7, 0xea, 
];

/// Verify update binary and do the file swapping\
/// Expected update_msg: "UPDATE|<target_binary>|<signature_base64>"\
/// Ex: "UPDATE|main_worker|<signature_base64>"
pub fn handle_update(update_msg: &str) {
    let parts: Vec<&str> = update_msg.split('|').collect();
    if parts.len() != 3 {
        log::error!("Invalid update msg format: {}", update_msg);
        return;
    }

    let target_binary = parts[1];
    let downloaded_binary_path = format!("{}/{}", AgentPath::TEMP_DOWNLOADED_PATH, target_binary);
    let downloaded_binary = downloaded_binary_path.as_str();
    let signature = parts[2];

    log::info!("Start update for {} from {}", target_binary, downloaded_binary);

    if !validate_crytography(downloaded_binary, signature) {
        log::error!("Security check failed, reject update and delete temporary file");
        let _ = fs::remove_file(downloaded_binary);
        return;
    }

    let target_path = match target_binary {
        "main_worker" => AgentPath::MAIN_WORKER_EXE,
        "admin_fetcher" => AgentPath::ADMIN_FETCHER_EXE,
        "proxy_scanner" => AgentPath::PROXY_SCANNER,
        // "launcher" => AgentPath::LAUNCHER, // Complicated
        s => {
            log::error!("Invalid target binary: {}", s);
            return;
        }
    };

    replace_binary(target_path, downloaded_binary);
}

/// ED25519 file verification
fn validate_crytography(file_path: &str, sign_base64: &str) -> bool {
    // Decode Base64 signature string into raw bytes
    let sign_byte = match BASE64.decode(sign_base64) {
        Ok(b) => b,
        Err(e) => {
            log::error!("Failed to decode signature: {}", e);
            return false;
        }
    };

    let signature = match Signature::from_slice(&sign_byte) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Invalid Ed25519 signature format: {}", e);
            return false;
        }
    };

    // Load the mathematical public key
    let verifying_key = match VerifyingKey::from_bytes(&SERVER_PUB_KEY) {
        Ok(k) => k,
        Err(_) => {
            log::error!("FATAL: Hardcoded public key is invalid");
            return false;
        }
    };

    // Read the downloaded binary file to memory
    let file_bytes = match fs::read(file_path) {
        Ok(b) => b,
        Err(e) => {
            log::error!("Failed to read temp file for cryptographic hashing: {}", e);
            return false;
        }
    };

    // Verify raw binary bytes against the sinature
    match verifying_key.verify(&file_bytes, &signature) {
        Ok(_) => {
            log::info!("Cryptographic verification PASSED for {}", file_path);
            true
        },
        Err(_) => {
            log::error!("Cryptographic verification FAILED for {}", file_path);
            false
        }
    }
}
