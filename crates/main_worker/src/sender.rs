//! 
//! # Network handling
//! 
//! Take the payload and send it to central server, wait for response.
//! 
//! The core Agent - Server logic is lazy and one-way, so only Agent can actively make HTTPS connection to server.
//! If server want to send any command to an Agent, it first have to wait for that Agent to be active, and send command(s) on the HTTPS response.
//! After Agent receive the response, it will immediately skip next sleep and working on those commands.
//! 
//! Currently, the server enpoint url is hardcoded, but that is subjected to change.
//! 
//! The payload sample:
//! ```json
//! {
//!   "result": {
//!     "result": "1",
//!     "errorDesc": null,
//!     "cmds": [
//!       {
//!         "cmd": "AskFullScan"
//!       },
//!       {
//!         "cmd": "UpdateConfig",
//!         "payload": {
//!             {
//!             "machine.model": [0, "test"],
//!             "machine.type": [0, null],
//!             "motherboard.tempe": [30, 5],
//!             "cpu.temperature": [30, 5],
//!             "ram.usage": [30, 1024],
//!             "battery.is_plugged_in": [1800, null],
//!             "disk.logical.used": [30, 1024],
//!             }
//!         }
//!       },
//!       {
//!         "cmd": "UpdateAgent",
//!         "payload": {
//!           "binary": "main_worker",
//!           "version": "1.0.0",
//!           "signature": "NjWz797ntYSLFwg7nKYYdn+On3cCMj4zKz059wadVVlBHxyxe4JrSZxgekwb9AYR5xFxuVE9dTnDSo+gCaW/CQ==",
//!         }
//!       },
//!       {
//!         "cmd": "LogLevel",
//!         "payload": {
//!           "level": "debug"
//!         }
//!       },
//!       {
//!         "cmd": "AskSendLog",
//!       },
//!     ]
//!   },
//!   "targetUrl": null,
//!   "success": true,
//!   "error": null,
//!   "unAuthorizedRequest": false,
//!   "__abp": true
//! }
//! ```

use std::time::Duration;
use serde_json::Value as Json;
use ureq::tls::TlsConfig;
use serde::Deserialize;

const SERVER_ENDPOINT_FULL_SCAN: &str = "https://172.20.0.98:44301/api/AssIT/ASS_IT_COMPUTER_Ins";
const SERVER_ENDPOINT_DELTA: &str = "https://172.20.0.98:44301/api/AssIT/ASS_IT_COMPUTER_Delta";
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

/// Enum list of acceptable server commands
#[derive(Debug, Deserialize)]
#[serde(tag = "cmd", content = "payload", rename_all = "PascalCase")]
pub enum ServerCmd {
    /// Force Agent to immediately send a full scan
    AskFullScan,

    /// Send in the "delta" of updated config ,which each entry as "module.component": [`<cycle_time>`, `Option<limit>`]
    UpdateConfig(Json),

    /// Tell Agent to download and update to specific version
    UpdateAgent { binary: String, version: String, signature: String },

    //TODO

    /// Ask Agent to send the it full local config file
    AskConfig,

    /// Ask Agent to send the latest log? TODO
    AskLog,

    /// Promote Agent to `Proxy Agent`
    Promote,

    /// Send addititional config for Proxy Agent
    ProxyConfig(Json),

    /// Fallback
    #[serde(other)]
    Unknown,
}

pub struct Sender {
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
            agent,
        }
    }

    /// Interuptably send the payload and wait for response
    pub async fn transmit(&self, payload: Json, full_scan: bool) -> Vec<ServerCmd> {
        let agent = self.agent.clone(); // it a Arc, so cheap clone

        let endpoint = if full_scan {
            log::info!("Agent send to FULL SCAN endpoint");
            SERVER_ENDPOINT_FULL_SCAN
        } else {
            log::info!("Agent send to DELTA endpoint");
            SERVER_ENDPOINT_DELTA
        };
        
        let join_result = tokio::task::spawn_blocking(move || {
            agent.post(endpoint)
                .header("X-Agent-Key", API_KEY) 
                .send_json(&payload)
        }).await;

        let http_result = match join_result {
            Ok(res) => res,
            Err(join_error) => {
                log::error!("HTTP background task cancelled: {}", join_error);
                return Vec::new();
            }
        };

        match http_result {
            Ok(response) => {
                let reply_text = match response.into_body().read_to_string() {
                    Ok(text) => text,
                    Err(e) => {
                        log::warn!("Agent failed to read repspone body: {}", e);
                        return Vec::new();
                    }
                };
                
                log::info!("Success! Edge server replied: {}", reply_text);

                if reply_text.trim().is_empty() {
                    return Vec::new();
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

                parsed_rep.result.and_then(|rep| rep.cmds).unwrap_or_else(|| Vec::new())
                
            },
            Err(e) => {
                log::error!("Sender error: {}", e);
                Vec::new()
            }
        }
    }
}