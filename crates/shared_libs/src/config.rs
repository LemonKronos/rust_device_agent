//!
//! Contain config for all binary
//! 

use std::time::Duration;

//: DEV CONFIG
#[cfg(debug_assertions)]
pub mod dev_config {
    /// In dev mode, will include the current in-dev feature
    // pub const AGENT_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), ".", "versioning");
    pub const AGENT_VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const USE_CUSTOM_SERIAL: bool = false;
    pub const CUSTOM_SERIAL: &str = "TEST_MACHINE_03";

    /// Make the default dev config for only the "general" module
    pub const SIMPLE_CONFIG: bool = true; 
}

#[cfg(not(debug_assertions))]
pub const AGENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Allow software scanning or not
pub const SCAN_SOFTWARE: bool = !cfg!(debug_assertions) || false;

/// Init with full scan
pub const INIT_FULL_SCAN: bool = true;

//: Versioning
/// Hardcoded Public Key 
pub const SERVER_PUB_KEY: [u8; 32] = [
    0xd2, 0x6c, 0x4e, 0x8e, 0x15, 0x33, 0x45, 0xff, 
    0x60, 0xf8, 0x6c, 0x03, 0xc6, 0x73, 0xb8, 0x4d, 
    0x15, 0x2b, 0x2d, 0x13, 0xee, 0x24, 0x13, 0x41, 
    0x0f, 0xb9, 0x12, 0xf1, 0x42, 0x67, 0xc7, 0xea, 
];

//: Timing
/// Time out for `main_worker` shut down wait
pub const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(10);

//: URL
pub const SERVER_ENDPOINT_FULL_SCAN: &str = "https://172.20.0.98:44301/api/AssIT/ASS_IT_COMPUTER_Ins";
pub const SERVER_ENDPOINT_DELTA: &str = "https://172.20.0.98:44301/api/AssIT/ASS_IT_COMPUTER_Delta";
pub const SERVER_ENDPOINT_DOWNLOAD: &str = "https://172.20.0.98:44301/api/Agent/Download?";
pub const API_KEY: &str = "72895e95e7634de2a8f554abaf10ec0e7a46fff57b114b68b74efcd65a5d3ec5";