///
///  main worker: handle non-root task and network
/// 

use main_worker::DeviceAgent;

#[tokio::main]
async fn main() {

    let mut agent = DeviceAgent::new();

    if let Err(e) = agent.run().await {
        eprintln!("Critial fatal error: Agent crashed!");
        eprintln!("Reason: {}", e);

        std::process::exit(1);
    }
}