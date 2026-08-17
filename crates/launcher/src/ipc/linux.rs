use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::process::Command;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use shared_libs::utils::*;

const SOCKET_PATH: &str = "/tmp/gsoft_agent.sock";

pub async fn start_ipc_server() {
    //TODO log correctly
    init_logger(env!("CARGO_PKG_NAME"));

    log::info!("[Launcher IPC] 1. Cleaning up old socket...");
    let _ = fs::remove_file(SOCKET_PATH);
    
    log::info!("[Launcher IPC] 2. Binding to socket...");
    let listener = match UnixListener::bind(SOCKET_PATH) {
        Ok(l) => l,
        Err(e) => {
            log::error!("Failed to bin Unix socket: {}", e);
            return;
        }
    };
    
    // WARNING: This is required if launcher is root and worker is non-root!
    // This gives rw access to the socket file so the worker isn't rejected.
    if let Err(e) = fs::set_permissions(SOCKET_PATH, fs::Permissions::from_mode(0o777)) {
        log::error!("Failed to set socket permissions: {}", e);
        return;
    }
    
    log::info!("[Launcher IPC] 3. Listening on {} (Permissions set to 777)", SOCKET_PATH);

    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            log::info!("[Launcher IPC] -> New connection accepted!");
            
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                match stream.read(&mut buf).await {
                    Ok(n) if n > 0 => {
                        let key = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                        log::info!("[Launcher IPC] -> Received request for key: '{}'", key);
                        
                        // NOTE: Update this path if your target folder is somewhere else!
                        let fetcher_path = "./target/debug/admin_fetcher";
                        log::info!("[Launcher IPC] -> Spawning: {} {}", fetcher_path, key);
                        
                        let output = Command::new(fetcher_path).arg(&key).output();

                        let response = match output {
                            Ok(out) if out.status.success() => {
                                let val = String::from_utf8_lossy(&out.stdout).to_string();
                                log::info!("[Launcher IPC] -> admin_fetcher success. Sending back: '{}'", val);
                                val
                            },
                            Ok(out) => {
                                log::error!("[Launcher IPC] -> admin_fetcher failed with status. stderr: {}", String::from_utf8_lossy(&out.stderr));
                                "ERROR_FETCHING".to_string()
                            },
                            Err(e) => {
                                log::error!("[Launcher IPC] -> Failed to execute admin_fetcher: {}", e);
                                "ERROR_FETCHING".to_string()
                            }
                        };

                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                            log::error!("[Launcher IPC] -> Failed to send response back to worker: {}", e);
                        }
                    },
                    Ok(_) => log::error!("[Launcher IPC] -> Connection closed before sending data."),
                    Err(e) => log::error!("[Launcher IPC] -> Failed to read from socket: {}", e),
                }
            });
        }
    }
}