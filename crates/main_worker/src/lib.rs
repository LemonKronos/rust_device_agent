#![doc = include_str!("../../../README.md")]

//!
//! Logic wrapper for the app, contain the main loop and all module
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

// ! DEV CONFIG
#[cfg(debug_assertions)]
mod dev_config {
   /// In dev mode, will include the current in-dev feature
    pub const AGENT_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), ".", "separate_binary");
    pub const USE_CUSTOM_SERIAL: bool = false;
    pub const CUSTOM_SERIAL: &str = "TEST_MACHINE_03";

    /// Make the default dev config for only the "general" module
    pub const SIMPLE_CONFIG: bool = true; 
}

#[cfg(not(debug_assertions))]
const AGENT_VERSION: &str = env!("CARGO_PKG_VERSION");

use crate::scheduler::Scheduler;
use crate::payload_maker::PayloadMaker;
use crate::sender::{Sender, ServerCmd};
use crate::config_handler::*;
use shared_libs::utils::*;

#[cfg(debug_assertions)]
use crate::dev_config::*;

/// Allow software scanning or not
const SCAN_SOFTWARE: bool = !cfg!(debug_assertions) || false;

/// Init with full scan
const INIT_FULL_SCAN: bool = true;

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
                ServerCmd::UpdateAgent { version } => {
                    log::info!(
                        "Server ask to update to agent version {} over current version {}",
                        version, AGENT_VERSION)
                },
                ServerCmd::Unknown => {
                    log::warn!("Agent receive an Unknown Command")
                },
                _ => {
                    log::warn!("Server ask for TODO commands")
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