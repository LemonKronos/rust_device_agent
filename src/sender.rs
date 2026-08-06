use std::time::Duration;
use serde_json::Value;
use ureq::tls::TlsConfig;

const SERVER_ENDPOINT: &str = "https://172.20.0.98:44301/api/AssIT/ASS_IT_COMPUTER_Delta";
const API_KEY: &str = "72895e95e7634de2a8f554abaf10ec0e7a46fff57b114b68b74efcd65a5d3ec5";

pub struct Sender {
    server_endpoint: String,
    agent: ureq::Agent,
}

impl Sender {
    pub fn new() -> Self {
        // Bypass certificate verification for the self-signed edge server cert
        let tls_config = TlsConfig::builder()
            .disable_verification(true)
            .build();

        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(3)))
            .tls_config(tls_config)
            .build().into();
        
        Self {
            server_endpoint: SERVER_ENDPOINT.to_string(),
            agent,
        }
    }

    pub fn transmit(&self, payload: Value) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.agent.post(&self.server_endpoint)
            .header("X-Agent-Key", API_KEY) 
            .send_json(&payload)?;

        let reply_text = response.into_body().read_to_string()?;

        log::info!("Success! Edge server replied: {}", reply_text);
        
        Ok(())
    }
}