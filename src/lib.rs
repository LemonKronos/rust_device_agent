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
use crate::sender::Sender;
use crate::utils::*;
use crate::config_handler::*;

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

        loop {
            tokio::select! {
                _ = self.scheduler.go_sleep() => {
                    log::info!("Agent wake up")
                },
                _ = signal::ctrl_c() => {
                    log::info!("Normal shutdown");
                    break;
                }
            }

            let batch = self.scheduler.pop_due_batch();
            if batch.is_empty() {
                log::warn!("Batch empty, AHHHHHHHH");
                continue;
            }

            let json = self.payload_maker.process_batch_mock_up(&batch);
            
            //TODO Run sequential for now
            self.scheduler.reschedule_batch(batch);
            match self.sender.transmit(json) {
                Ok(_) => {

                },
                Err(e) => {
                    log::warn!("Sender warning: {}", e)
                },
            }

            // End of cycle, agent sleep here
        }


        Ok(())
    }
}

//TODO redo the test for new run
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

        let mut full_wheel: TimerWheel = init_config();
        let batch = full_wheel.pop_due_batch();
        let _ = agent.payload_maker.process_batch_mock_up(&batch);
        full_wheel.reschedule_batch(batch);

        assert!(start.elapsed().as_millis() < 500, "Fullscan took too long, over {} ms", start.elapsed().as_millis());
    }

    #[test]
    fn test_payload_has_no_empty_strings() {
        let mut agent = DeviceAgent::new();

        let mut full_wheel: TimerWheel = init_config();
        let batch = full_wheel.pop_due_batch();
        let payload = agent.payload_maker.process_batch_mock_up(&batch);
        assert_no_empty_strings(&payload);
    }
}