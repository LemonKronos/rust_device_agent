#![doc = include_str!("../../../README.md")]

//! # Main Worker
//!
//! `main_worker` is the unprivileged worker process responsible for the normal
//! operation of the Gsoft agent.
//!
//! The worker runs the agent's main execution loop. On each cycle it checks
//! the scheduler for tasks that are due, collects the information required by
//! those tasks, builds the payload, and sends the result to the Gsoft server.
//!
//! The high-level data flow is:
//!
//! ```text
//! Scheduler
//!     |
//!     | due tasks
//!     V
//! PayloadMaker
//!     |
//!     | payload
//!     V
//! Sender
//!     |
//!     V
//! Gsoft Server
//! ```
//!
//! `DeviceAgent` coordinates these components and also handles commands
//! received from the server, such as configuration changes, full scans, and
//! agent updates.
//!
//! The worker also communicates with the privileged `launcher` process through
//! the platform-specific IPC layer when an operation requires launcher
//! privileges, such as updating an agent binary.
//!
//! ## Main Responsibilities
//!
//! - Run the normal agent scheduling and collection loop.
//! - Collect system/device information required by scheduled tasks.
//! - Generate incremental payloads through [`PayloadMaker`].
//! - Send payloads and process server responses.
//! - Handle configuration updates and full-scan requests.
//! - Coordinate agent update requests with the privileged launcher.
//! - Perform a graceful shutdown when requested by the launcher or operating
//!   system.
//!
//! The worker is intentionally kept separate from the privileged `launcher`.
//! `main_worker` performs the normal agent workload without requiring elevated
//! privileges, while operations that cross the privilege boundary are handled
//! by the launcher through IPC.
//!
//! For the detailed behavior of individual subsystems, see the module
//! documentation for [`scheduler`], [`payload_maker`], [`ipc`], and
//! [`os_specific`].
//! 

use std::error::Error;
use tokio::signal::unix::{signal, SignalKind};
use tokio::io::{AsyncReadExt, stdin};

pub mod os_specific;
pub mod info_gatherer;
pub mod sender;
pub mod payload_maker;
pub mod scheduler;
pub mod config_handler;
pub mod ipc;

use crate::scheduler::Scheduler;
use crate::payload_maker::PayloadMaker;
use crate::sender::{Sender, ServerCmd};
use crate::ipc::ask_update;
use crate::config_handler::*;
use shared_libs::utils::*;

#[cfg(debug_assertions)]
use shared_libs::config::dev_config::*;

use shared_libs::config::{SCAN_SOFTWARE, INIT_FULL_SCAN};

/// `main_worker` binary version, read from `cargo.toml`
pub const MAIN_WORKER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct DeviceAgent {
    payload_maker: PayloadMaker,
    sender: Sender,
    scheduler: Scheduler,

    full_scan: bool,
}

impl DeviceAgent {
    pub fn new() -> Self {
        init_logger("main_worker");

        Self {
            payload_maker: PayloadMaker::new(),
            sender: Sender::new(),
            scheduler: load_config(),

            full_scan: INIT_FULL_SCAN,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {

        log::info!("Agent loop start");

        let mut sigterm = match signal(SignalKind::terminate()) {
            Ok(s) => s,
            Err(e) => {
                log::error!("FATAL: Failed to bind SIGTERM OS signal: {}, exit now.", e);
                return Err(Box::new(e));
            }
        };

        let mut sigint = match signal(SignalKind::interrupt()) {
            Ok(s) => s,
            Err(e) => {
                log::error!("FATAL: Failed to bind SIGINT OS signal: {}, exit now.", e);
                return Err(Box::new(e));
            }
        };

        let mut stdin = stdin();
        let mut buf = [0; 1];

        loop {
            tokio::select! {
                // Task 1: normal work cycle
                _ = self.process_cycle() => {},

                // Task 2: sigterm
                _ = sigterm.recv() => {
                    log::info!("Caught SIGTERM, start shutdown sequence");
                    self.shutdown();
                    break;
                },

                // Task 3: sigint
                _ = sigint.recv() => {
                    log::info!("Caught SIGINT, start shutdown sequence");
                    self.shutdown();
                    break;
                },

                // Task 4: dropped pipe
                res = stdin.read(&mut buf) => {
                    match res {
                        Ok(0) => log::info!("Launcher ask to shutdown, start shutdown sequence"),
                        Err(e) => log::error!("Stdin error: {}", e),
                        _ => {
                            log::warn!("Receive unexpected data on stdin, ignoring");
                            continue;
                        }
                    }
                    self.shutdown();
                    break;
                }
            }
        }

        Ok(())
    }

    /// A single iteration of the normal agent work cycle
    async fn process_cycle(&mut self) {
        let batch = self.scheduler.pop_due_batch();
        if batch.is_empty() && !self.full_scan {
            log::warn!("Agent wake up but batch is empty and not full scan!");
        }

        let json = self.payload_maker.process_batch(&batch, self.full_scan);
        
        self.scheduler.reschedule_batch(batch, self.full_scan);

        let server_commands = self.sender.transmit(json, self.full_scan).await;

        // //DEV mock server cmds
        // let server_commands = vec![
        //     ServerCmd::UpdateConfig(serde_json::json!({
        //         "machine.model": [0, "test"],
        //         "cpu.temperature": [30, 1]
        //     })),

        //     ServerCmd::UpdateAgent {
        //         binary: "main_worker".to_string(),
        //         version: "1.0.1".to_string(),
        //         signature: "WZTxRClivkG0ObKfKM4cALjembHovFrF+KhJLJZHSjgi7R0qLyxBwwVtwGfQqAxbyLX48LQoLKgQXgA/QCN/Cw==".to_string(),
        //     }
        // ];

        self.full_scan = false;

        for cmd in server_commands {
            match cmd {
                ServerCmd::AskFullScan => {
                    log::info!("Server ask for Full Scan");
                    self.full_scan = true;
                    self.scheduler.skip_sleep();
                },
                ServerCmd::UpdateConfig(new_config) => {
                    log::info!("Server ask to update config");
                    self.scheduler.update(new_config);
                },
                ServerCmd::UpdateAgent { binary, version, signature } => { //TODO handle multiple binary update in one server response
                    let current_version = mock_get_binary_version(&binary);

                    if version == current_version {
                        log::info!(
                            "Server ask to update to agent binary '{}' to version '{}' over current version '{}', ignored",
                            binary, version, current_version
                        );
                    } else {
                        log::info!(
                            "Server ask to update to agent binary '{}' to version '{}' over current version '{}'",
                            binary, version, current_version
                        );
                    }

                    // //DEV Temporarily pre-place the target binary in downloaded destination for easy demostration
                    // if !self.sender.download_file(&binary, &version) {
                    //     //TODO send distress to server to avoid spamming respawn of main_worker
                    //     log::error!("Can not download binary '{}'", binary);
                    //     continue;
                    // }

                    self.scheduler.save();
                    if !ask_update(&binary, &signature) {
                        log::error!("Can not update binary '{}'", binary);
                        //TODO send distress to server
                    }
                },
                _ => {
                    log::warn!("Agent receive an Unknown Command")
                }
            }
        }

        self.scheduler.go_sleep().await;
    }

    /// Save config to disk
    fn shutdown(&mut self) {
        self.scheduler.save();
        log::info!("Normal shutdown completed");
    }
}

//TODO handling individual binary version
/// Mock up launcher version
const LAUNCHER_VERSION: &str = "0.1.0";

/// Mock up admin_fetcher version
const ADMIN_FETCHER_VERSION: &str = "0.1.0";

/// Mock up to get binary version
fn mock_get_binary_version(binary_name: &str) -> &str {
    match binary_name {
        "main_worker" => MAIN_WORKER_VERSION,
        "launcher" => LAUNCHER_VERSION,
        "admin_fetcher" => ADMIN_FETCHER_VERSION,
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    /// Recursively traverses a JSON Value and asserts that no string is exactly `""`.
    fn assert_no_empty_strings(value: &Value) {
        match value {
            Value::String(s) => {
                assert!(
                    !s.is_empty(),
                    "Test failed: Found an empty string `\"\"` in the JSON payload!"
                );
            },
            Value::Array(arr) => {
                for item in arr {
                    assert_no_empty_strings(item);
                }
            },
            Value::Object(obj) => {
                for val in obj.values() {
                    assert_no_empty_strings(val);
                }
            },
            _ => {},
        }
    }

    #[test]
    fn test_limit_agent_runtime() {
        let mut agent = DeviceAgent::new();
        let start = std::time::Instant::now();

        let mut sched: Scheduler = load_config();
        let batch = sched.pop_due_batch();
        let _ = agent.payload_maker.process_batch(&batch, true);
        sched.reschedule_batch(batch, true);

        assert!(start.elapsed().as_millis() < 500, "Fullscan took too long, over {} ms", start.elapsed().as_millis());
    }

    #[test]
    fn test_payload_has_no_empty_strings() {
        let mut agent = DeviceAgent::new();

        let mut sched: Scheduler = load_config();
        let batch = sched.pop_due_batch();
        let payload = agent.payload_maker.process_batch(&batch, true);
        assert_no_empty_strings(&payload);
    }
}