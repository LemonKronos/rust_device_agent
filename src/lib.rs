

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
        println!("Agent running");

        println!("Getting full scan v2...");
        let _ = self.payload.get_json_v2_fullscan();
        println!("Complete");

        println!("Getting full scan v3...");
        let _ = self.payload.get_json_v3_fullscan();
        println!("Complete");

        println!("Start loop sending telementry v2 demo ");
        loop {
            self.payload.prepare();

            if let Err(e) = self.sender.transmit(self.payload.get_json_v2_telemetry()) {
                eprintln!("Sender warning: {}", e);
            } else {
                println!("Sended info at timestamp {}", self.payload.timestamp());
            }

            thread::sleep(Duration::from_secs(5));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};
    use super::DeviceAgent;
    

    #[test]
    fn test_json() {
        let mut agent = DeviceAgent::new();

        sleep(Duration::from_secs(1));

        agent.payload.prepare();

        let payload = agent.payload.get_json_v3_fullscan();

        let pretty_json = serde_json::to_string_pretty(&payload).unwrap();
        
        println!("\n=== GENERATED HARDWARE PAYLOAD ===");
        println!("{}", pretty_json);
        println!("==================================\n");
    }
}