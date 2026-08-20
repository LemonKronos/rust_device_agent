//!
//! Watch dog implementation for Linux
//! 

use std::process::Command;
use tokio::signal::unix::{signal, SignalKind};

#[cfg(all(debug_assertions, feature = "local_workspace"))]
const MAIN_WORKER_PATH: &str = "./target/debug/main_worker";
#[cfg(all(not(debug_assertions), feature = "local_workspace"))]
const MAIN_WORKER_PATH: &str = "./target/release/main_worker";
#[cfg(all(not(debug_assertions), not(feature = "local_workspace")))]
const MAIN_WORKER_PATH: &str = "/opt/gsoft-agent/main_worker";

/// Calling 'main_worker' and watching it
pub async fn run_watchdog() {
    log::info!("Starting Watchdog...");

    if !is_root() {
        log::error!("FATAL: Launcher do not have admin priviledge, exit now.");
        println!("FATAL: Launcher do not have admin priviledge, exit now.");
        return;
    }

    let (uid, gid) = match ensure_agent_user() {
        Some(ids) => ids,
        None => {
            log::error!("FATAL: Unable to resolve group 'gsoft-agent', exit now.");
            println!("FATAL: Unable to resolve group 'gsoft-agent', exit now.");
            return;
        }
    };
    
    let mut sigterm = match signal(SignalKind::terminate()) {
        Ok(s) => s,
        Err(e) => {
            log::error!("FATAL: Failed to bind SIGTERM OS signal: {}, exit now.", e);
            println!("FATAL: Failed to bind SIGTERM OS signal: {}, exit now.", e);
            return;
        }
    };

    let mut sigint = match signal(SignalKind::interrupt()) {
        Ok(s) => s,
        Err(e) => {
            log::error!("FATAL: Failed to bind SIGINT OS signal: {}, exit now.", e);
            println!("FATAL: Failed to bind SIGINT OS signal: {}, exit now.", e);
            return;
        }
    };

    loop {
        log::info!("Spawning main_worker (UID: {}, GID: {})", uid, gid);
        println!("Spawning main_worker (UID: {}, GID: {})", uid, gid);

        let mut child_process = match tokio::process::Command::new(MAIN_WORKER_PATH)
            .uid(uid)
            .gid(gid)
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to spawn main_worker: {}", e);
                println!("Failed to spawn main_worker: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                continue;
            }
        };

        tokio::select! {
            // The child process exited (crash or normal exit)
            status = child_process.wait() => {
                match status {
                    Ok(s) => log::warn!("main_worker exited with status: {}", s),
                    Err(e) => log::error!("main_worker process error: {}", e),
                }
                log::info!("Restarting main_worker in 5 seconds...");
                println!("Restarting main_worker in 5 seconds...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
            
            // Systemctl sent a SIGTERM
            _ = sigterm.recv() => {
                log::info!("Caught SIGTERM. Terminating main_worker and exiting Watchdog.");
                println!("Caught SIGTERM. Terminating main_worker and exiting Watchdog.");
                let _ = child_process.kill().await;
                return; 
            }
            
            // User hit Ctrl+C in terminal
            _ = sigint.recv() => {
                log::info!("Caught SIGINT. Terminating main_worker and exiting Watchdog.");
                println!("Caught SIGINT. Terminating main_worker and exiting Watchdog.");
                let _ = child_process.kill().await;
                return;
            }
        }
    }
}

/// Helper to check if the current process is running as root
fn is_root() -> bool {
    match Command::new("id").arg("-u").output() {
        Ok(output) => {
            let uid = String::from_utf8_lossy(&output.stdout).trim().to_string();
            uid == "0"
        },
        Err(_) => false,
    }
}

/// Helper to fetch the numerical UID and GID of the restricted user
fn get_agent_ids() -> Option<(u32, u32)> {
    let uid_out = Command::new("id").arg("-u").arg("gsoft-agent").output().ok()?;
    let gid_out = Command::new("id").arg("-g").arg("gsoft-agent").output().ok()?;

    let uid = String::from_utf8_lossy(&uid_out.stdout).trim().parse::<u32>().ok()?;
    let gid = String::from_utf8_lossy(&gid_out.stdout).trim().parse::<u32>().ok()?;

    Some((uid, gid))
}

/// Helper to fetch IDs, and create the user/group if they don't exist
fn ensure_agent_user() -> Option<(u32, u32)> {
    if let Some(ids) = get_agent_ids() {
        return Some(ids); // User already exist
    }

    log::warn!("User group 'gsoft-agent' does not exist, attemp to create one.");

    let output = Command::new("useradd")
        .arg("--system")
        .arg("--no-create-home")
        .arg("--shell")
        .arg("/user/bin/nologin")
        .arg("gsoft-agent")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            log::info!("Successfully create user group 'gsoft-agent'");
            get_agent_ids()
        },
        Ok(out) => {
            log::error!("Failed to create group, OS responded: {}", String::from_utf8_lossy(&out.stderr));
            None
        },
        Err(e) => {
            log::error!("Failed to run 'useradd command: {}", e);
            None
        }
    }
}