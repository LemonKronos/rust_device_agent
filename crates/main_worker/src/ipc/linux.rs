use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::time::Duration;

const SOCKET_PATH: &str = "/run/gsoft-agent/ipc.sock";

pub fn ask_admin(key: &str) -> Option<String> {
    log::info!("Asking Admin Fetcher for '{}'", key);
    
    let mut stream = match UnixStream::connect(SOCKET_PATH) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to connect IPC to Launcher: {}", e);
            return None;
        }
    };
    
    let timeout = Duration::from_secs(1);
    let _ = stream.set_write_timeout(Some(timeout));
    let _ = stream.set_read_timeout(Some(timeout));
    
    if let Err(e) = stream.write_all(key.as_bytes()) {
        log::error!("Failed to write to socket: {}", e);
        return None;
    }
    
    let mut buf = String::new();
    if let Err(e) = stream.read_to_string(&mut buf) {
        log::error!("Failed to read response: {}", e);
        return None;
    }
    
    if buf.trim().is_empty() {
        log::error!("Launcher failed to fetch the data.");
        None
    } else {
        Some(buf)
    }
}

pub fn ask_update(binary: &str, signature: &str) -> bool {
    log::info!("Asking Laucher to update binary '{}'", binary);

    let mut stream = match UnixStream::connect(SOCKET_PATH) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to connect IPC to Launcher: {}", e);
            return false;
        }
    };

    let timeout = Duration::from_secs(1);
    let _ = stream.set_write_timeout(Some(timeout));
    let _ = stream.set_read_timeout(Some(timeout));

    let payload = format!("UPDATE|{}|{}", binary, signature);
    if let Err(e) = stream.write_all(payload.as_bytes()) {
        log::error!("Failed to write to socket: {}", e);
        return false;
    }

    true
}