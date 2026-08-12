#![doc = include_str!("../../../README.md")]

//!
//! Logic wrapper for the app, contain the main loop and all module
//! 

use std::error::Error;
use tokio::signal;

pub mod os_specific;
pub mod info_gatherer;
pub mod sender;
pub mod payload_maker;
pub mod scheduler;
pub mod config_handler;

use crate::scheduler::TimerWheel;
use crate::payload_maker::PayloadMaker;
use crate::sender::{Sender, ServerCmd};
use shared_libs::utils::*;
use crate::config_handler::*;

/// Allow software scanning or not
const SCAN_SOFTWARE: bool = !cfg!(debug_assertions) || false;

/// Init with full scan
const INIT_FULL_SCAN: bool = true;


// ! DEV CONFIG
#[cfg(debug_assertions)]

/// In dev mode, will include the current in-dev feature
const AGENT_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), ".", "separate_binary");
const USE_CUSTOM_SERIAL: bool = false;
const CUSTOM_SERIAL: &str = "TEST_MACHINE_03";

/// Make the default dev config for only the "general" module
const SIMPLE_CONFIG: bool = true;

#[cfg(not(debug_assertions))]
const AGENT_VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct DeviceAgent {
    payload_maker: PayloadMaker,
    sender: Sender,
    scheduler: TimerWheel,
}

impl DeviceAgent {
    pub fn new() -> Self {
        init_logger();

        Self {
            payload_maker: PayloadMaker::new(),
            sender: Sender::new(),
            scheduler: load_config(),
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {

        log::info!("Agent loop start");

        let mut full_scan = INIT_FULL_SCAN;

        loop {
            let batch = self.scheduler.pop_due_batch();
            if batch.is_empty() {
                log::warn!("Agent wake up but batch is empty! Maybe full scan?");
            }

            let json = self.payload_maker.process_batch(&batch, full_scan);
            
            self.scheduler.reschedule_batch(batch, full_scan);

            let server_commands = tokio::select! {
                cmds = self.sender.transmit(json, full_scan) => {
                    cmds
                },
                _ = signal::ctrl_c() => {
                    self.shutdown();
                    break;
                }
            };

            full_scan = false;

            for cmd in server_commands {
                match cmd {
                    ServerCmd::AskFullScan => {
                        log::info!("Server ask for Full Scan");
                        full_scan = true;
                        self.scheduler.skip_sleep();
                    },
                    ServerCmd::UpdateConfig(new_config) => {
                        log::info!("Server ask to update config");
                        self.scheduler.update_wheel(new_config);
                    },
                    ServerCmd::UpdateAgent { version } => {
                        log::info!("Server ask to update to agent version {} over current version {}", version, AGENT_VERSION)
                    },
                    ServerCmd::Unknown => {
                        log::warn!("Agent receive an Unknown Command")
                    },
                    _ => {
                        log::warn!("Server ask for TODO commands")
                    }
                }
            }

            tokio::select! {
                _ = self.scheduler.go_sleep() => {
                    // empty
                },
                _ = signal::ctrl_c() => {
                    self.shutdown();
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.scheduler.save_wheel();
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

        let mut full_wheel: TimerWheel = load_config();
        let batch = full_wheel.pop_due_batch();
        let _ = agent.payload_maker.process_batch(&batch, true);
        full_wheel.reschedule_batch(batch, true);

        assert!(start.elapsed().as_millis() < 500, "Fullscan took too long, over {} ms", start.elapsed().as_millis());
    }

    #[test]
    fn test_payload_has_no_empty_strings() {
        let mut agent = DeviceAgent::new();

        let mut full_wheel: TimerWheel = load_config();
        let batch = full_wheel.pop_due_batch();
        let payload = agent.payload_maker.process_batch(&batch, true);
        assert_no_empty_strings(&payload);
    }
}