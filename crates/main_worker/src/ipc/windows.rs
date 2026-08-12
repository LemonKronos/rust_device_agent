use std::fs::OpenOptions;
use std::io::{Read, Write};

const PIPE_NAME: &str = r"\\.\pipe\gsoft_agent_pipe";

pub fn ask_launcher(key: &str) -> Option<String> {
    // 1. Open the pipe blocking
    let mut client = OpenOptions::new()
        .read(true)
        .write(true)
        .open(PIPE_NAME)
        .ok()?;
        
    // 2. Write and Read
    client.write_all(key.as_bytes()).ok()?;
    
    let mut buf = String::new();
    client.read_to_string(&mut buf).ok()?;
    
    if buf == "ERROR_FETCHING" || buf.is_empty() {
        None
    } else {
        Some(buf)
    }
}