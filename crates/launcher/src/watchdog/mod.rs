//!
//! Watch dog for Agent life time
//! 

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