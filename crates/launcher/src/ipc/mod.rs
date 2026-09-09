
//! # Launcher IPC server
//!
//! Provides the IPC server through which other agent binaries communicate with
//! the privileged `launcher` process.
//!
//! The transport is selected at compile time:
//!
//! - Linux -> Unix domain socket
//! - Windows -> Windows named pipe
//!
//! The platform-specific implementation is hidden behind [`start_ipc_server`],
//! which is re-exported here.
//!
//! ## Responsibilities
//!
//! The launcher IPC server handles requests that must cross the process
//! boundary, including:
//!
//! - requests for privileged system information, which are delegated to
//!   `admin_fetcher`
//! - binary update requests, which are forwarded to the watchdog
//!
//! ## Process relationship
//!
//! ```text
//!                    launcher (privileged)
//!                    /              \
//!                   /                \
//!             IPC server          watchdog
//!                /                    |
//!               /                     |
//!       admin_fetcher           main_worker
//! ```
//!
//! The IPC module does not perform the update itself. Update requests are
//! handed to the watchdog so that `main_worker` can be stopped before its
//! binary is replaced.

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::start_ipc_server;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::start_ipc_server;