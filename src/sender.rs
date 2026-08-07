use std::time::Duration;
use serde_json::Value as Json;
use ureq::tls::TlsConfig;
use serde::Deserialize;

const SERVER_ENDPOINT: &str = "https://172.20.0.98:44301/api/AssIT/ASS_IT_COMPUTER_Delta";
const API_KEY: &str = "72895e95e7634de2a8f554abaf10ec0e7a46fff57b114b68b74efcd65a5d3ec5";

/// APB wrapper
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbpResponse {
    pub success: bool,
    pub result: Option<AbpResult>, 
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbpResult {
    pub result: Option<String>,
    pub error_desc: Option<String>,
    pub cmds: Option<Vec<ServerCmd>>, 
}

#[derive(Debug, Deserialize)]
#[serde(tag = "cmd", content = "payload", rename_all = "PascalCase")]
pub enum ServerCmd {
    AskFullScan,
    UpdateConfig(Json),
    UpdateAgent { version: String },

    #[serde(other)]
    Unknown,
}

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
            .timeout_global(Some(Duration::from_secs(30)))
            .tls_config(tls_config)
            .build().into();
        
        Self {
            server_endpoint: SERVER_ENDPOINT.to_string(),
            agent,
        }
    }

    pub fn transmit(&self, payload: Json) -> Vec<ServerCmd> {
        match self.agent.post(&self.server_endpoint)
            .header("X-Agent-Key", API_KEY) 
            .send_json(&payload) {
                Ok(response) => {
                    let reply_text = match response.into_body().read_to_string() {
                        Ok(text) => text,
                        Err(e) => {
                            log::warn!("Agent failed to read repspone body: {}", e);
                            return vec![];
                        }
                    };
                    
                    log::info!("Success! Edge server replied: {}", reply_text);

                    if reply_text.trim().is_empty() {
                        return vec![];
                    }

                    let parsed_rep: AbpResponse = serde_json::from_str(&reply_text).unwrap_or_else(|e| {
                        log::warn!("Json parse error from server: {}", e);
                        AbpResponse { success: false, result: None }
                    });

                    if let Some(rep) = &parsed_rep.result {
                        if let Some(err) = &rep.error_desc {
                            log::error!("Server return error description: {}", err);
                        }
                    }

                    parsed_rep.result.and_then(|rep| rep.cmds).unwrap_or_else(|| vec![])
                    
                },
                Err(e) => {
                    log::error!("Sender error: {}", e);
                    vec![]
                }
            }
    }
}