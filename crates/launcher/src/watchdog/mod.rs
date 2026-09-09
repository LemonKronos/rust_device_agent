
//! # Main worker watchdog
//!
//! The watchdog owns the lifecycle of the unprivileged `main_worker` process.
//!
//! `launcher` runs with elevated privileges, while `main_worker` performs the
//! normal agent workload with restricted privileges. The watchdog is the
//! boundary between those two processes.
//!
//! ## Responsibilities
//!
//! The platform-specific watchdog implementation:
//!
//! - starts `main_worker`
//! - runs it under the restricted `gsoft-agent` account where supported
//! - detects worker termination
//! - restarts the worker after an unexpected exit
//! - responds to launcher shutdown signals
//! - coordinates binary updates
//!
//! ## Shutdown mechanism
//!
//! `main_worker` receives a shutdown request by having its stdin pipe closed.
//! This allows the worker to perform its own cleanup before termination.
//!
//! [`graceful_shutdown`] waits for [`SHUTDOWN_TIMEOUT`] before forcefully
//! killing the process. This prevents a stuck worker from preventing launcher
//! shutdown indefinitely.
//!
//! ## Update lifecycle
//!
//! An update follows this general sequence:
//!
//! ```text
//! update request
//!       |
//!       v
//! watchdog
//!       |
//!       v
//! graceful_shutdown(main_worker)
//!       |
//!       v
//! versioning::handle_update()
//!       |
//!       v
//! spawn updated main_worker
//! ```
//!
//! The watchdog therefore acts as the process-lifecycle coordinator rather
//! than implementing update logic itself.

use shared_libs::config::SHUTDOWN_TIMEOUT;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::run_watchdog;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::run_watchdog;

/// Graceful shutdown for `main_worker` when "normal shutdown" and "update stop"
async fn graceful_shutdown(
    mut child: tokio::process::Child,
    stdin_pipe: tokio::process::ChildStdin,
    reason: &str
) {
    log::info!("Graceful shutdown cause by: {}", reason);

    // Tell main worker to shutdown via dropped pipe
    drop(stdin_pipe);

    match tokio::time::timeout(SHUTDOWN_TIMEOUT, child.wait()).await {
        Ok(Ok(status)) => log::info!("Worker exited cleanly: {}", status),
        Ok(Err(e)) => log::error!("Worker exited with OS error: {}", e),
        Err(_) => {
            // Main worker still stay, force kill
            log::warn!("Worker deadlock (wait exeeded {:?}), force kill", SHUTDOWN_TIMEOUT);
            let _ = child.kill().await;
        }
    }
    let _ = child.kill().await;
    return;
}