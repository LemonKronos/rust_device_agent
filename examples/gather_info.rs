use sysinfo::{Components, Disks, Networks, System};
use all_smi::{AllSmi, Result};
use std::net::{TcpStream, SocketAddr};
use std::time::{Duration, Instant};
use std::thread::{self, sleep};
use serde_json::json;
use ureq::Agent;

fn main() -> Result<()> {
    //_ General info via sysinfo
    // Initialize the system struct
    let mut sys = System::new_all();

    // Refresh all data to ensure we have the latest metrics
    sys.refresh_all();

    sleep(Duration::from_millis(500));

    sys.refresh_all();

    println!("=== Device Agent: System Info ===");

    // System/OS Info
    println!("Host name:       {:?}", System::host_name().unwrap_or_default());
    println!("System name:     {:?}", System::name().unwrap_or_default());
    println!("OS version:      {:?}", System::os_version().unwrap_or_default());
    println!("Kernel version:  {:?}", System::kernel_version().unwrap_or_default());

    // CPU Info
    let cpu_count = sys.cpus().len();
    println!("CPU Cores:       {}", cpu_count);
    if let Some(first_cpu) = sys.cpus().first() {
        println!("CPU Brand:       {}", first_cpu.brand());
    }

    println!("CPU usage:    {}", sys.global_cpu_usage());

    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!("Core {}: {} Mhz | Usage: {:.2}%", i, cpu.frequency(), cpu.cpu_usage())
    }

    // Memory Info (Converting bytes to Megabytes for readability)
    let total_mem_mb = sys.total_memory() / 1024 / 1024;
    let used_mem_mb = sys.used_memory() / 1024 / 1024;
    println!("Memory Total:    {} MB", total_mem_mb);
    println!("Memory Used:     {} MB", used_mem_mb);

    println!("=================================");

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display processes ID, name and disk usage:
    let mut procs: Vec<_> = sys.processes().values().collect();
    procs.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());
    for process in procs.into_iter().take(10) {
        println!("Name: {:?} | CPU: {:?} | RAM: {:?}", process.name(), process.cpu_usage(), process.memory());
    }

    // We display all disks' information:
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}");
    }

    // Network interfaces name, total data received and total data transmitted:
    let networks = Networks::new_with_refreshed_list();
    println!("=> networks:");
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} B (down) / {} B (up) / IP: {:#?} / MAC: {} / MTU: {}",
            data.total_received(),
            data.total_transmitted(),
            data.ip_networks(),
            data.mac_address(),
            data.mtu()
        );
        // If you want the amount of data received/transmitted since last call
        // to `Networks::refresh`, use `received`/`transmitted`.

        println!("==============================");

        println!("Network: {:#?}", data);
    }

    // Components temperature:
    let components = Components::new_with_refreshed_list();
    println!("=> components:");
    for component in &components {
        println!("{component:?}");
    }

    

    //_ GPU via all-smi
    println!("=== Testing all-smi ===");
    
    // all-smi dynamically hooks into NVML, ROCm, or Apple/Intel equivalents
    let smi = AllSmi::new()?; 
    let gpus = smi.get_gpu_info();
    
    println!("Found {} GPU(s) via all-smi", gpus.len());
    for (i, gpu) in gpus.iter().enumerate() {
        // Debug print dumps all the available metrics (thermals, VRAM, clock speeds)
        println!("GPU [{}]: {:#?}", i, gpu);
    }

    //_ Network via TCP handshake ping
    // Target Cloudflare's DNS server on the standard DNS port (53)
    let target: SocketAddr = "1.1.1.1:53".parse().expect("Failed to parse address");
    let timeout = Duration::from_secs(1);
    
    let total_pings = 5;
    let mut successful_pings = 0;
    let mut total_latency = Duration::new(0, 0);

    println!("=== Zero-Privilege Network Agent ===");
    println!("TCP Pinging {}...", target);

    for i in 1..=total_pings {
        let start = Instant::now();
        
        // Attempt to open a TCP connection to the target
        match TcpStream::connect_timeout(&target, timeout) {
            Ok(_) => {
                // If it connects, immediately measure the elapsed time
                let latency = start.elapsed();
                println!("Reply {}: time={}ms", i, latency.as_millis());
                successful_pings += 1;
                total_latency += latency;
            }
            Err(e) => {
                println!("Request {}: Failed! Reason: {}", i, e);
            }
        }
        
        // Sleep for a second before the next attempt
        thread::sleep(Duration::from_secs(1));
    }

    // Calculate quality metrics
    let loss = ((total_pings - successful_pings) as f32 / total_pings as f32) * 100.0;
    let avg_latency = if successful_pings > 0 {
        total_latency.as_millis() / successful_pings as u128
    } else {
        0
    };

    println!("====================================");
    println!("Packets: Sent = {}, Received = {}, Lost = {} ({}% loss)", 
             total_pings, successful_pings, total_pings - successful_pings, loss);
    println!("Average Latency: {}ms", avg_latency);

    //_ Send to server
    println!("=== ITAM Agent: Network Sync ===");

    // 1. Package the data
    let payload = json!({
        "agent_id": "uuid-gsoft-device-agent-test",
        "os": "Macaroni",
        "cpu_usage": 15.4,
        "ram_mb": 16384,
        "network_latency_ms": 32
    });

    // Replace this with your teammate's actual IPv4 address
    let server_url = "http://192.168.1.100:8080/api/telemetry";

    // 2. In v3, timeouts and error handling are configured via an Agent
    let agent: Agent = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .http_status_as_error(false) // This lets us read the body on 400/500 errors
        .build()
        .into();

    println!("Sending data to {}...", server_url);

    // 3. Send the POST request using the new v3 syntax
    let request = agent.post(server_url)
        .header("Content-Type", "application/json")
        .send_json(&payload);

    // 4. Handle the response
    match request {
        Ok(mut response) => {
            let status = response.status();
            
            // Check if it's a 2xx success code
            if status.is_success() {
                println!("Success! Server responded with status code: {}", status);
            } else {
                // Since we disabled http_status_as_error, 4xx/5xx codes end up here
                println!("Server rejected the payload. Code: {}", status);
                if let Ok(body) = response.body_mut().read_to_string() {
                    println!("Server message: {}", body);
                }
            }
        }
        Err(e) => {
            // Pure network failures (e.g., server offline, firewall block, timeout)
            println!("Network Error: Could not reach the server.");
            println!("Details: {}", e);
        }
    }

    Ok(())
}