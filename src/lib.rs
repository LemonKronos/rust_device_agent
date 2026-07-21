///
/// Contain all logic for Agent
/// 

use std::thread;
use std::time::Duration;

pub mod os_specific;
pub mod info_gatherer;
pub mod sender;
pub mod types;
pub mod utils;
pub mod payload_maker;
pub mod scheduler;
pub mod config_handler;

use crate::payload_maker::Payload;
use crate::sender::Sender;
use crate::utils::*;
use crate::config_handler::*;

pub struct DeviceAgent {
    payload: Payload,
    sender: Sender,
}

impl DeviceAgent {
    pub fn new() -> Self {
        Self {
            payload: Payload::new(),
            sender: Sender::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        init_logger();

        let _ = load_config();

        log::info!("Agent running");

        log::info!("Getting full scan v2...");
        let _ = self.payload.get_json_v2_fullscan();
        log::info!("Complete");

        log::info!("Getting full scan v3...");
        let _ = self.payload.get_json_v3_fullscan();
        log::info!("Complete");

        log::info!("Start loop sending telementry v2 demo ");
        loop {
            self.payload.prepare();

            let _ = self.payload.get_json_v3_fullscan();

            if let Err(e) = self.sender.transmit(self.payload.get_json_v2_telemetry()) {
                log::warn!("Sender warning: {}", e);
            } else {
                log::info!("Sended info at timestamp {}", self.payload.timestamp());
            }

            thread::sleep(Duration::from_secs(5));
        }
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
        let agent = DeviceAgent::new();
        let start = std::time::Instant::now();
        let _payload_v3_fullscan = agent.payload.get_json_v3_fullscan();
        assert!(start.elapsed().as_millis() < 500, "Fullscan took too long, over {} ms", start.elapsed().as_millis());
    }

    #[test]
    fn test_payload_has_no_empty_strings() {
        let agent = DeviceAgent::new();

        let payload_v3_fullscan = agent.payload.get_json_v3_fullscan();
        assert_no_empty_strings(&payload_v3_fullscan);

        let payload_v2_fullscan = agent.payload.get_json_v2_fullscan();
        assert_no_empty_strings(&payload_v2_fullscan);

        let payload_v2_telemetry = agent.payload.get_json_v2_telemetry();
        assert_no_empty_strings(&payload_v2_telemetry);
    }
}