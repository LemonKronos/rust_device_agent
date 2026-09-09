
//! # Inter-process communication
//!
//! This module provides the client-side IPC interface used by `main_worker`
//! to communicate with the privileged `launcher` process.
//!
//! The transport is platform-specific:
//!
//! - Linux uses a Unix domain socket.
//! - Windows uses a Windows named pipe.
//!
//! The platform implementation is selected at compile time. The platform
//! details are re-exported here so callers do not need to know which transport
//! is being used.
//!
//! ## Main responsibilities
//!
//! The IPC client is primarily used for:
//!
//! - requesting privileged information from `admin_fetcher` through `launcher`
//! - requesting an agent binary update through `launcher`
//!
//! `main_worker` therefore communicates with `launcher` through this module
//! rather than accessing privileged OS resources directly.

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;