use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::time::Duration;

const SOCKET_PATH: &str = "/tmp/gsoft_agent.sock";

pub fn ask_launcher(key: &str) -> Option<String> {
    println!("[Worker IPC] 1. Attempting to connect to {}", SOCKET_PATH);
    
    let mut stream = match UnixStream::connect(SOCKET_PATH) {
        Ok(s) => {
            println!("[Worker IPC] -> Connected successfully!");
            s
        },
        Err(e) => {
            println!("[Worker IPC] -> FATAL: Failed to connect: {}", e);
            return None;
        }
    };
    
    let timeout = Duration::from_secs(2);
    let _ = stream.set_write_timeout(Some(timeout));
    let _ = stream.set_read_timeout(Some(timeout));
    
    println!("[Worker IPC] 2. Sending request for key: '{}'", key);
    if let Err(e) = stream.write_all(key.as_bytes()) {
        println!("[Worker IPC] -> FATAL: Failed to write to socket: {}", e);
        return None;
    }
    
    println!("[Worker IPC] 3. Waiting for response...");
    let mut buf = String::new();
    if let Err(e) = stream.read_to_string(&mut buf) {
        println!("[Worker IPC] -> FATAL: Failed to read response: {}", e);
        return None;
    }
    
    println!("[Worker IPC] 4. Received raw response: '{}'", buf);
    
    if buf == "ERROR_FETCHING" || buf.is_empty() {
        println!("[Worker IPC] -> Launcher failed to fetch the data.");
        None
    } else {
        Some(buf)
    }
}