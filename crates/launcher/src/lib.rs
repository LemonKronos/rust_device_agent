#![doc = include_str!("../README.md")]

//! 
//! Launcher manage binary, version, watch dog, intergrity.
//! Run as background task, called at start-up by sub-system.
//! 

use tokio::sync::mpsc;

use shared_libs::utils::*;

pub mod ipc;
pub mod versioning;
pub mod watchdog;
pub mod permission;

pub const LAUNCHER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Launcher {
    // Empty
}

impl Launcher {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self) {
        init_logger("launcher");

        log::info!("Rust Launcher starting up.");

        if !permission::ensure_file_permission() {
            log::error!("FATAL: Failed to ensure file permisson, exit now.");
            return;
        }

        let (tx, rx) = mpsc::channel::<String>(5);

        tokio::select! {
            _ = ipc::start_ipc_server(tx) => {
                log::error!("IPC server crashed unexpectedly.");
            }
            _ = watchdog::run_watchdog(rx) => {
                log::error!("Watchdog loop exited unexpectedly.");
            }
        }
        
        log::info!("Launcher shutdown complete.");
    }
}