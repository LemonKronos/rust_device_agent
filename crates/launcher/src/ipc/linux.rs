//!
//! IPC implementation for Linux: 
//! Using UNIX socket with user group privilegde
//! 


use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;
use std::os::unix::fs::PermissionsExt;

const RUN_PATH: &str = "/run/gsoft-agent";
const SOCKET_PATH: &str = "/run/gsoft-agent/ipc.sock";

#[cfg(all(debug_assertions, feature = "local_workspace"))]
const ADMIN_FETCHER_PATH: &str = "./target/debug/admin_fetcher";
#[cfg(all(not(debug_assertions), feature = "local_workspace"))]
const ADMIN_FETCHER_PATH: &str = "./target/release/admin_fetcher";
#[cfg(all(not(debug_assertions), not(feature = "local_workspace")))]
const ADMIN_FETCHER_PATH: &str = "/opt/gsoft_device_agent/bin/admin_fetcher";

pub async fn start_ipc_server() {
    if let Err(e) = fs::create_dir_all(RUN_PATH) {
        log::error!("Failed to create run directory: {}", e);
        return;
    }

    let _ = fs::remove_file(SOCKET_PATH);
    
    let listener = match UnixListener::bind(SOCKET_PATH) {
        Ok(l) => l,
        Err(e) => {
            log::error!("Failed to bin Unix socket: {}", e);
            return;
        }
    };
    
    if let Err(e) = fs::set_permissions(SOCKET_PATH, fs::Permissions::from_mode(0o660)) {
        log::error!("Failed to set socket permissions: {}", e);
        return;
    }
    
    if let Err(e) = std::process::Command::new("chgrp").arg("gsoft-agent").arg(SOCKET_PATH).output() {
        log::error!("Failed to change socket group ownership: {}", e);
        return;
    }

    log::info!("Listening on {}", SOCKET_PATH);

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(handle_connection(stream));
        }
    }
}

async fn handle_connection(mut stream: UnixStream) {
    let mut buf = [0; 1024];

    let n = match stream.read(&mut buf).await {
        Ok(n) if n > 0 => n,
        Ok(_) => {
            log::error!("Connection close before sending data.");
            return;
        },
        Err(e) => {
            log::error!("Failed to read data from socket: {}", e);
            return;
        }
    };

    let key = String::from_utf8_lossy(&buf[..n]).trim().to_string();
    log::info!("Received request for key '{}'", key);

    let output = std::process::Command::new(ADMIN_FETCHER_PATH).arg(&key).output();

    let reponse = match output {
        Ok(out) if out.status.success() => {
            let val = String::from_utf8_lossy(&out.stdout).to_string();
            log::info!("Admin fetch info success, sending back '{}'", val);
            val
        },
        Ok(out) => {
            let err_msg = String::from_utf8_lossy(&out.stderr);
            log::error!("'admin_fetcher failed with status stderr:{}", err_msg);
            "".to_string()
        },
        Err(e) => {
            log::error!("Failed to execute 'admin_fetcher': {}", e);
            "".to_string()
        }
    };

    if let Err(e) = stream.write_all(reponse.as_bytes()).await {
        log::error!("Failed to send response back to worker: {}", e);
    }
}