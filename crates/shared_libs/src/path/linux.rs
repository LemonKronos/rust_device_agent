//!
//! Path configure for Linux
//! Note: Changes on these paths need to be reflected to the 'crates/launcher/debian' scripts.
//! 

pub struct AgentPath;

impl AgentPath {
    pub const AGENT_USER_GROUP: &str = "gsoft-agent";

    pub const RUN_PATH: &str = "/run/gsoft-agent";
    pub const SOCKET_FILE: &str = "/run/gsoft-agent/ipc.sock";
    pub const TEMP_DOWNLOADED_PATH: &str = "/tmp/gsoft-agent";

    #[cfg(debug_assertions)]
    pub const SAMPLE_PATH: &str = "./doc/sample";
}

#[cfg(feature = "local_workspace")] 
impl AgentPath {
    pub const CONFIG_FILE: &str = "./doc/config.json";
    pub const LOG_PATH: &str = "./doc/logs";

    #[cfg(debug_assertions)]
    pub const ADMIN_FETCHER_EXE: &str = "./target/debug/admin_fetcher";
    #[cfg(not(debug_assertions))]
    pub const ADMIN_FETCHER_EXE: &str = "./target/release/admin_fetcher";

    #[cfg(debug_assertions)]
    pub const MAIN_WORKER_EXE: &str = "./target/debug/main_worker";
    #[cfg(not(debug_assertions))]
    pub const MAIN_WORKER_EXE: &str = "./target/release/main_worker";

    #[cfg(debug_assertions)]
    pub const PROXY_SCANNER: &str = "./target/debug/proxy_scanner";
    #[cfg(not(debug_assertions))]
    pub const PROXY_SCANNER: &str = "./target/release/proxy_scanner";

    #[cfg(debug_assertions)]
    pub const LAUNCHER: &str = "./target/debug/launcher";
    #[cfg(not(debug_assertions))]
    pub const LAUNCHER: &str = "./target/release/launcher";
}

#[cfg(not(feature = "local_workspace"))]
impl AgentPath {
    pub const CONFIG_FILE: &str     = "/var/lib/gsoft-agent/config";
    pub const LOG_PATH: &str        = "/var/log/gsoft_agent";
    pub const ADMIN_FETCHER_EXE: &str   = "/opt/gsoft_agent/bin/admin_fetcher";
    pub const MAIN_WORKER_EXE: &str     = "/opt/gsoft-agent/bin/main_worker";
    pub const PROXY_SCANNER: &str       = "/opt/gsoft-agent/bin/proxy_scanner";
    pub const LAUNCHER: &str            = "/opt/gsoft-agent/bin/launcher";
}


