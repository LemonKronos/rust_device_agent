
//! # Filesystem permissions
//!
//! Platform-specific initialization and permission management for the agent's
//! runtime directories, configuration, logs, and binaries.
//!
//! The implementation is selected at compile time and re-exported through this
//! module so the launcher can call the same API on every supported platform.
//!
//! On Linux, permission setup also establishes the ownership boundary between
//! the privileged `launcher` and the restricted `gsoft-agent` user.
//!
//! This module is called during launcher startup, before the IPC server and
//! worker watchdog are started.

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;
