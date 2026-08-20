use tokio::net::windows::named_pipe::ServerOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::process::Command;

const PIPE_NAME: &str = r"\\.\pipe\gsoft_device_agent_pipe";

pub async fn start_ipc_server() {
    log::info!("Launcher listening on {}", PIPE_NAME);

    loop {
        // Windows pipes require a new instance for every connection
        let mut server = ServerOptions::new()
            .first_pipe_instance(true)
            .create(PIPE_NAME)
            .expect("Failed to create Named Pipe");

        if server.connect().await.is_ok() {
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                if let Ok(n) = server.read(&mut buf).await {
                    let key = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                    
                    let output = Command::new("admin_fetcher.exe")
                        .arg(&key)
                        .output();

                    let response = match output {
                        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).to_string(),
                        _ => "ERROR_FETCHING".to_string(),
                    };

                    let _ = server.write_all(response.as_bytes()).await;
                }
            });
        }
    }
}