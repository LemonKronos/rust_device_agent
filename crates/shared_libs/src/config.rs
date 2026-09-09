
//! Shared runtime constants used by all four binaries: dev-mode overrides, scan
//! behavior toggles, the update-verification public key, network timeouts, and
//! hardcoded server endpoints.

use std::time::Duration;

//: DEV CONFIG
/// In dev mode, will include the current in-dev feature
#[cfg(debug_assertions)]
pub mod dev_config {
    /// If `true`, overrides the machine's real serial number with `CUSTOM_SERIAL` below.
    /// Useful for running multiple dev instances that need to appear as distinct machines to the server.
    pub const USE_CUSTOM_SERIAL: bool = false;

    /// The fake serial reported when `USE_CUSTOM_SERIAL` is `true`.
    pub const CUSTOM_SERIAL: &str = "TEST_MACHINE_03";

    /// When `true`, `init_config()` seeds only the `general.*` tasks instead of all ~80, for faster dev iteration.
    pub const SIMPLE_CONFIG: bool = true; 
}

/// Whether software-inventory scanning runs at all. Disabled by default in debug to skip a slow scan during local dev; 
/// consider enabled in release.
pub const SCAN_SOFTWARE: bool = !cfg!(debug_assertions) || false;

/// If `true`, the Agent performs a full scan immediately on start-up rather than
/// waiting for its first scheduled cycle. See `DeviceAgent::new()`.
pub const INIT_FULL_SCAN: bool = true;

//: Versioning
/// Ed25519 public key used by `launcher` to verify binary-update signatures.
/// Matching private key lives only on the Central Server. Compiled in rather than
/// configurable, so a compromised on-disk config can't be used to accept forged updates.
pub const SERVER_PUB_KEY: [u8; 32] = [
    0xd2, 0x6c, 0x4e, 0x8e, 0x15, 0x33, 0x45, 0xff, 
    0x60, 0xf8, 0x6c, 0x03, 0xc6, 0x73, 0xb8, 0x4d, 
    0x15, 0x2b, 0x2d, 0x13, 0xee, 0x24, 0x13, 0x41, 
    0x0f, 0xb9, 0x12, 0xf1, 0x42, 0x67, 0xc7, 0xea, 
];

//: Timing
/// How long `launcher` waits for `main_worker` to exit gracefully (via dropped stdin)
/// before force-killing it during an update or shutdown.
pub const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(10);

//: URL
/// Central Server endpoint for full-scan payloads.
pub const SERVER_ENDPOINT_FULL_SCAN: &str = "https://172.20.1.61:44301/api/AssIT/ASS_IT_COMPUTER_Ins";

/// Central Server endpoint for delta (partial/changed-data) payloads.
pub const SERVER_ENDPOINT_DELTA: &str = "https://172.20.1.61:44301/api/AssIT/ASS_IT_COMPUTER_Delta";

/// Central Server endpoint for downloading signed update binaries.
pub const SERVER_ENDPOINT_DOWNLOAD: &str = "https://172.20.1.61:44301/api/Agent/Download?";

/// API key sent with every request to authenticate this Agent to the server.
pub const API_KEY: &str = "72895e95e7634de2a8f554abaf10ec0e7a46fff57b114b68b74efcd65a5d3ec5";