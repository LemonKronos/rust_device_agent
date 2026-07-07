use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Default)]
pub struct NetworkCard {
    interface: String,
    model_name: String,
    speed_mbps: u32,
    ssid: String,
}

fn get_network_cards() -> Vec<NetworkCard> {
    let mut cards = Vec::new();
    let net_dir = Path::new("/sys/class/net");

    if let Ok(entries) = fs::read_dir(net_dir) {
        for entry in entries.flatten() {
            let iface = entry.file_name().to_string_lossy().into_owned();

            // Filter out virtual interfaces (localhost, docker, bridges)
            if iface == "lo" || iface.starts_with("docker") || iface.starts_with("br-") || iface.starts_with("veth") {
                continue;
            }

            let mut card = NetworkCard {
                interface: iface.clone(),
                model_name: "Unknown Network Card".to_string(),
                ssid: "null".to_string(),
                speed_mbps: 0,
            };

            // 1. Get Speed (Native sysfs read)
            // If the ethernet cable is unplugged, this file throws an OS error. We safely default to 0.
            let speed_path = net_dir.join(&iface).join("speed");
            if let Ok(speed_str) = fs::read_to_string(speed_path) {
                card.speed_mbps = speed_str.trim().parse().unwrap_or(0);
            }

            // 2. Get Model Name (Extract PCI address & query lspci)
            let device_link = net_dir.join(&iface).join("device");
            if let Ok(target) = fs::read_link(&device_link) {
                // The target looks like "../../../0000:00:1f.6"
                if let Some(pci_addr) = target.file_name().and_then(|n| n.to_str()) {
                    if let Ok(output) = Command::new("lspci").arg("-s").arg(pci_addr).output() {
                        let out_str = String::from_utf8_lossy(&output.stdout);
                        // lspci output format: "00:1f.6 Ethernet controller: Intel Corporation..."
                        if let Some(desc) = out_str.split(": ").nth(1) {
                            card.model_name = desc.trim().to_string();
                        }
                    }
                }
            }

            // 3. Get SSID (Query iwgetid)
            // -r prints just the raw SSID. It gracefully fails empty if not connected or not Wi-Fi.
            if let Ok(output) = Command::new("iwgetid").arg("-r").arg(&iface).output() {
                let out_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !out_str.is_empty() {
                    card.ssid = out_str;
                }
            }

            cards.push(card);
        }
    }
    cards
}

fn main() {
    println!("--- Scanning Network Cards ---\n");
    let cards = get_network_cards();
    for card in cards {
        println!("Interface:  {}", card.interface);
        println!("Card Model: {}", card.model_name);
        println!("Speed:      {} Mbps", card.speed_mbps);
        println!("SSID:       {}", card.ssid);
        println!("--------------------------------------");
    }
}