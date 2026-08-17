use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::process::Command;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use shared_libs::utils::*;

const SOCKET_PATH: &str = "/tmp/gsoft_agent.sock";

#[cfg(feature = "profiling")]
const ADMIN_FETCHER_PATH: &str = "./target/release/admin_fetcher";
#[cfg(not(feature = "profiling"))]
const ADMIN_FETCHER_PATH: &str = "/opt/gsoft/bin/admin_fetcher";

pub async fn start_ipc_server() {
    init_logger(env!("CARGO_PKG_NAME"));

    let _ = fs::remove_file(SOCKET_PATH);
    
    let listener = match UnixListener::bind(SOCKET_PATH) {
        Ok(l) => l,
        Err(e) => {
            log::error!("Failed to bin Unix socket: {}", e);
            return;
        }
    };
    
    if let Err(e) = fs::set_permissions(SOCKET_PATH, fs::Permissions::from_mode(0o666)) {
        log::error!("Failed to set socket permissions: {}", e);
        return;
    }
    
    log::info!("Listening on {} (Permissions set to 666)", SOCKET_PATH);

    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                match stream.read(&mut buf).await {
                    Ok(n) if n > 0 => {
                        let key = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                        log::info!("Received request for key: '{}'", key);
                        
                        let output = Command::new(ADMIN_FETCHER_PATH).arg(&key).output();

                        let response = match output {
                            Ok(out) if out.status.success() => {
                                let val = String::from_utf8_lossy(&out.stdout).to_string();
                                log::info!("admin_fetcher success. Sending back: '{}'", val);
                                val
                            },
                            Ok(out) => {
                                log::error!("admin_fetcher failed with status. stderr: {}", String::from_utf8_lossy(&out.stderr));
                                "ERROR_FETCHING".to_string()
                            },
                            Err(e) => {
                                log::error!("Failed to execute admin_fetcher: {}", e);
                                "ERROR_FETCHING".to_string()
                            }
                        };

                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                            log::error!("Failed to send response back to worker: {}", e);
                        }
                    },
                    Ok(_) => log::error!("Connection closed before sending data."),
                    Err(e) => log::error!("Failed to read from socket: {}", e),
                }
            });
        }
    }
}