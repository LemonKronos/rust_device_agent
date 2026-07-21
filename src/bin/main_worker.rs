///
///  main worker: handle non-root task and network
/// 



use gsoft_device_agent::DeviceAgent;

fn main() {

    let mut agent = DeviceAgent::new();

    if let Err(e) = agent.run() {
        eprintln!("Critial fatal error: Agent crashed!");
        eprintln!("Reason: {}", e);

        std::process::exit(1);
    }
}