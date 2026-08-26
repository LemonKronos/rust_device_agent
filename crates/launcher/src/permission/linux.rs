//!
//! Linux file permission.
//! Grant permission according to [`AgentPath`]
//! 

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

use shared_libs::path::AgentPath;

pub fn ensure_file_permission() -> bool {
    // 1. Strict Root Check
    let is_root = match Command::new("id").arg("-u").output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim() == "0",
        Err(_) => false,
    };

    if !is_root {
        log::error!("FATAL: Launcher must be run as root to set file permissions.");
        return false;
    }

    // 2. Identify the directories that need to be owned by gsoft-agent
    // We safely extract the parent directory of the config file.
    let config_dir = Path::new(AgentPath::CONFIG_FILE)
        .parent()
        .unwrap_or_else(|| Path::new("."));

    let secured_dirs = vec![
        Path::new(AgentPath::LOG_PATH),
        Path::new(AgentPath::RUN_PATH),
        config_dir,
    ];

    let user_group = format!("{}:{}", AgentPath::AGENT_USER_GROUP, AgentPath::AGENT_USER_GROUP);

    // 3. Create and secure the data/log directories
    for dir in secured_dirs {
        if let Err(e) = fs::create_dir_all(dir) {
            log::error!("Failed to create directory {:?}: {}", dir, e);
            return false;
        }

        // Set 755 (Owner: rwx, Group/Others: r-x)
        if let Err(e) = fs::set_permissions(dir, fs::Permissions::from_mode(0o755)) {
            log::error!("Failed to set permissions on {:?}: {}", dir, e);
            return false;
        }

        // Change ownership to gsoft-agent so main_worker can write here
        let output = Command::new("chown")
            .arg("-R")
            .arg(&user_group)
            .arg(dir)
            .output();

        match output {
            Ok(out) if !out.status.success() => {
                log::error!("Failed to chown {:?}: {}", dir, String::from_utf8_lossy(&out.stderr));
                return false;
            }
            Err(e) => {
                log::error!("Failed to execute chown on {:?}: {}", dir, e);
                return false;
            }
            _ => log::info!("Successfully bootstrapped and secured {:?}", dir),
        }
    }

    // 4. Ensure the binaries exist and have executable permissions (755)
    // The launcher is root, so it keeps root ownership of these files.
    let binaries = vec![
        Path::new(AgentPath::MAIN_WORKER_EXE),
        Path::new(AgentPath::ADMIN_FETCHER_EXE),
    ];

    for bin in binaries {
        if bin.exists() {
            if let Err(e) = fs::set_permissions(bin, fs::Permissions::from_mode(0o755)) {
                log::warn!("Failed to set executable permissions on {:?}: {}", bin, e);
                // Not returning false here in case we are just bootstrapping an empty environment
            }
        } else {
            log::warn!("Binary not found at {:?}. It might be installed later.", bin);
        }
    }

    // 5. Ensure the config file itself is owned by the agent if it already exists
    if Path::new(AgentPath::CONFIG_FILE).exists() {
        let _ = Command::new("chown")
            .arg(&user_group)
            .arg(AgentPath::CONFIG_FILE)
            .output();
    }

    log::info!("All file permissions and ownerships successfully verified.");
    true
}