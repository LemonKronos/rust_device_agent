
use std::time::Duration;
use serde_json::Value;

const SERVER_ENDPOINT: &str = "http://172.20.2.154:8000/api/v1/scans";

pub struct Sender {
    server_endpoint: String,
    agent: ureq::Agent,
}

impl Sender {
    pub fn new() -> Self {
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(3)))
            .build().into();
        
        Self {
            server_endpoint: SERVER_ENDPOINT.to_string(),
            agent: agent
        }
    }

    pub fn transmit(&self, payload: Value) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.agent.post(&self.server_endpoint)
            .send_json(&payload)?;

        let reply_text = response.into_body().read_to_string()?;

        println!("Success! Edge server replied: {}", reply_text);
        
        Ok(())
    }
}