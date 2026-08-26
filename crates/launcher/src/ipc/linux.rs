//!
//! IPC implementation for Linux: 
//! Using UNIX socket with user group privilegde
//! 


use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::Sender;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use shared_libs::path::AgentPath;

pub async fn start_ipc_server(tx: Sender<String>) {
    if let Err(e) = fs::create_dir_all(AgentPath::RUN_PATH) {
        log::error!("Failed to create run directory: {}", e);
        return;
    }

    let _ = fs::remove_file(AgentPath::SOCKET_FILE);
    
    let listener = match UnixListener::bind(AgentPath::SOCKET_FILE) {
        Ok(l) => l,
        Err(e) => {
            log::error!("Failed to bin Unix socket: {}", e);
            return;
        }
    };
    
    if let Err(e) = fs::set_permissions(AgentPath::SOCKET_FILE, fs::Permissions::from_mode(0o660)) {
        log::error!("Failed to set socket permissions: {}", e);
        return;
    }
    
    if let Err(e) = std::process::Command::new("chgrp").arg("gsoft-agent").arg(AgentPath::SOCKET_FILE).output() {
        log::error!("Failed to change socket group ownership: {}", e);
        return;
    }

    log::info!("Listening on {}", AgentPath::SOCKET_FILE);

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(handle_connection(stream, tx.clone()));
        }
    }
}

async fn handle_connection(mut stream: UnixStream, tx: Sender<String>) {
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

    // Asked for update
    if key.starts_with("UPDATE|") {
        log::info!("Received request for update, handoff to watchdog");
        if let Err(e) = tx.send(key).await {
            log::error!("Failed to handoff to watchdog: {}", e);
        }
        return;
    }

    // Asked for admin_fetcher
    log::info!("Received request for key '{}'", key);

    let output = std::process::Command::new(AgentPath::ADMIN_FETCHER_EXE).arg(&key).output();

    let reponse = match output {
        Ok(out) if out.status.success() => {
            let val = String::from_utf8_lossy(&out.stdout).to_string();
            // log::info!("Admin fetch info success, sending back '{}'", val);
            log::info!("Resolve request for key '{}'", key);
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