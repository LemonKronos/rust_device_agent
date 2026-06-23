
use gsoft_device_agent::DeviceAgent;

fn main() {
    println!("=== Agent Init ===");

    let mut agent = DeviceAgent::new();

    if let Err(e) = agent.run() {
        eprintln!("Critial fatal error: Agent crashed!");
        eprintln!("Reason: {}", e);

        std::process::exit(1);
    }
}