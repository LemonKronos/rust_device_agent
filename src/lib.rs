///
/// Contain all logic for Agent
/// 

use std::error::Error;
use tokio::signal;

pub mod os_specific;
pub mod info_gatherer;
pub mod sender;
pub mod types;
pub mod utils;
pub mod payload_maker;
pub mod scheduler;
pub mod config_handler;

use crate::scheduler::TimerWheel;
use crate::payload_maker::PayloadMaker;
use crate::sender::{Sender, ServerCmd};
use crate::utils::*;
use crate::config_handler::*;

/// Allow software scanning or not
const SCAN_SOFTWARE: bool = !cfg!(debug_assertions) || false;

pub struct DeviceAgent {
    payload_maker: PayloadMaker,
    sender: Sender,
    scheduler: TimerWheel,
}

impl DeviceAgent {
    pub fn new() -> Self {
        Self {
            payload_maker: PayloadMaker::new(),
            sender: Sender::new(),
            scheduler: load_config(),
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        init_logger();

        log::info!("Agent loop start");

        let mut full_scan = true; // Init with full scan

        loop {
            let batch = self.scheduler.pop_due_batch();
            if batch.is_empty() {
                log::warn!("Agent wake up but batch is empty!");
                continue;
            }

            let json = self.payload_maker.process_batch(&batch, full_scan);
            full_scan = false;
            
            //TODO Run sequential for now
            match self.sender.transmit(json) {
                Ok(cmds) => {
                    for cmd in cmds {
                        match cmd {
                            ServerCmd::AskFullScan => {
                                log::info!("Server ask for Full Scan");
                                full_scan = true;
                                self.scheduler.skip_sleep();
                            },
                            ServerCmd::UpdateConfig(new_config) => {
                                todo!()
                            },
                            ServerCmd::UpdateAgent { version } => {
                                log::info!("Server aske to update to agent version {} over current version {}", version, "0.3.0.dev")
                            },
                            ServerCmd::Unknown => {
                                log::warn!("Agent receive an Unknown Command")
                            }
                        }
                    }
                },
                Err(e) => {
                    log::warn!("Sender warning: {}", e)
                },
            }

            self.scheduler.reschedule_batch(batch);

            tokio::select! {
                _ = self.scheduler.go_sleep() => {
                    // empty
                },
                _ = signal::ctrl_c() => {
                    self.scheduler.save_wheel();
                    log::info!("Normal shutdown completed, config saved");
                    break;
                }
            }
        }


        Ok(())
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
        full_wheel.reschedule_batch(batch);

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