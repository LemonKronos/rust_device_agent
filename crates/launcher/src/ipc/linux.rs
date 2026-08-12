use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::process::Command;
use std::fs;
use std::os::unix::fs::PermissionsExt;

const SOCKET_PATH: &str = "/tmp/gsoft_agent.sock";

pub async fn start_ipc_server() {
    println!("[Launcher IPC] 1. Cleaning up old socket...");
    let _ = fs::remove_file(SOCKET_PATH);
    
    println!("[Launcher IPC] 2. Binding to socket...");
    let listener = UnixListener::bind(SOCKET_PATH).expect("Failed to bind Unix Socket");
    
    // WARNING: This is required if launcher is root and worker is non-root!
    // This gives rw access to the socket file so the worker isn't rejected.
    fs::set_permissions(SOCKET_PATH, fs::Permissions::from_mode(0o777))
        .expect("Failed to set socket permissions");
    
    println!("[Launcher IPC] 3. Listening on {} (Permissions set to 777)", SOCKET_PATH);

    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            println!("[Launcher IPC] -> New connection accepted!");
            
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                match stream.read(&mut buf).await {
                    Ok(n) if n > 0 => {
                        let key = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                        println!("[Launcher IPC] -> Received request for key: '{}'", key);
                        
                        // NOTE: Update this path if your target folder is somewhere else!
                        let fetcher_path = "./target/debug/admin_fetcher";
                        println!("[Launcher IPC] -> Spawning: {} {}", fetcher_path, key);
                        
                        let output = Command::new(fetcher_path).arg(&key).output();

                        let response = match output {
                            Ok(out) if out.status.success() => {
                                let val = String::from_utf8_lossy(&out.stdout).to_string();
                                println!("[Launcher IPC] -> admin_fetcher success. Sending back: '{}'", val);
                                val
                            },
                            Ok(out) => {
                                println!("[Launcher IPC] -> admin_fetcher failed with status. stderr: {}", String::from_utf8_lossy(&out.stderr));
                                "ERROR_FETCHING".to_string()
                            },
                            Err(e) => {
                                println!("[Launcher IPC] -> Failed to execute admin_fetcher: {}", e);
                                "ERROR_FETCHING".to_string()
                            }
                        };

                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                            println!("[Launcher IPC] -> Failed to send response back to worker: {}", e);
                        }
                    },
                    Ok(_) => println!("[Launcher IPC] -> Connection closed before sending data."),
                    Err(e) => println!("[Launcher IPC] -> Failed to read from socket: {}", e),
                }
            });
        }
    }
}