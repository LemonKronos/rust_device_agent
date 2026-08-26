//! 
//! Atomic versioning for Linux
//! 

use std::fs;
use std::path::Path;
use crate::permission::secure_staging_binary;

pub fn replace_binary(target_path: &str, downloaded_binary_path: &str) {
    let staging_path = format!("{}.tmp", target_path);
    let backup_path = format!("{}.bak", target_path);

    // Write aside
    if let Err(e) = fs::copy(downloaded_binary_path, &staging_path) {
        log::error!("Failed to copy binary to staging area: {}", e);
        return;
    }

    // Clean up temporaty downloaded file
    let _ = fs::remove_file(downloaded_binary_path);

    // Staging lock down
    if !secure_staging_binary(&staging_path) {
        log::error!("Failed to secure staging binary, aborting update");
        let _ = fs::remove_file(&staging_path);
        return;
    }

    // Backup
    if Path::new(target_path).exists() {
        if let Err(e) = fs::rename(target_path, &backup_path) {
            log::error!("Failed to backup current binary: {}", e);
            let _ = fs::remove_file(&staging_path);
            return;
        }
    }

    // Atomic swap
    if let Err(e) = fs::rename(&staging_path, target_path) {
        log::error!("Atomic swap failed: {}. Attempting rollback...", e);

        // Try to secure old version
        if Path::new(&backup_path).exists() {
            let _ = fs::rename(&backup_path, target_path);
        }

        let _ = fs::remove_file(&staging_path);
        return;
    }

    log::info!("Atomic binary swap completed successfully for {}", target_path);
}

