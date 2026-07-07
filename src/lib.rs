
use std::fs;
use std::thread;
use std::time::Duration;
use serde_json::json;

pub mod os_specific;
pub mod info_gatherer;
pub mod sender;
pub mod types;

use crate::info_gatherer::Info;
use crate::sender::Sender;

pub struct DeviceAgent {
    info: Info,
    sender: Sender,
}

impl DeviceAgent {
    pub fn new() -> Self {
        Self {
            info: Info::new(),
            sender: Sender::new(),
        }
    }

    fn get_json(&self) -> serde_json::Value {
        let gpus_json: Vec<_> = self.info.get_gpu_list().iter().map(|gpu| {
            json!({
                "name": gpu.get_name(),
                "driver": gpu.get_driver(),
                "utilization": gpu.get_util(), // %
                "temperature": gpu.get_temp(), // ℃
                "vram_total": gpu.get_vram_total(), // byte
                "vram_usage": gpu.get_vram_usage(), // byte
                //TODO
            })
        }).collect();

        let disks_json: Vec<_> = self.info.get_disk_list().iter().map(|disk| {
            json!({
                "name": disk.get_name(),
                "type": disk.get_type(),
                "removable": disk.get_removable(), // bool
                "total": disk.get_total(), // byte
                "used": disk.get_used(), // byte
                //TODO
            })
        }).collect();

        let networks_json: Vec<_> = self.info.get_network_list().iter().map(|network| {
            json!({
                "name": network.get_name(),
                "ipv4": network.get_ipv4(),
                "ipv6": network.get_ipv6(),
                "mac": network.get_mac(),
                "mtu": network.get_mtu(), // byte
                "upload": network.get_upload(), // byte
                "download": network.get_download(), // byte
                //TODO
            })
        }).collect();

        let top_processes_json: Vec<_> = self.info.get_top_process_list().iter().map(|process| {
            json!({
                "name": process.get_name(),
                "cpu": process.get_cpu_usage(), // %
                "memory": process.get_memory(), // byte
                "runtime": process.get_runtime(), // second
            })
        }).collect();

        let all_processes_json: Vec<_> = self.info.get_full_process_list().iter().map(|process| {
            json!({
                "name": process.get_name(),
                "cpu": process.get_cpu_usage(),
                "memory": process.get_memory(),
                "runtime": process.get_runtime(),
            })
        }).collect();

        //TODO
        // let softwares_json: Vec<_> = self.

        let payload = json!({
            "time_stamp": self.info.get_timestamp(), // second
            "general": {
                "boot_time": self.info.get_boot_time(), // second
                "run_time": self.info.get_run_time(), // second
            },
            "machine": {   
                "serial": self.info.get_product_serial(),
                "architecture": self.info.get_architecture(),
                "producer": self.info.get_producer(),
                "model": self.info.get_system_model(),
                "machine_type": self.info.get_machine_type(),
            },
            "motherboard": {
                "name": self.info.get_motherboard(),
                "serial": self.info.get_motherboard_serial(),
                "tempe": self.info.get_temp_mobo(),
                //TODO more on hardware
            },
            "bios": {
                //TODO
            },
            "peripheral": {
                //TODO
            },

            "os": {
                "os": self.info.get_os(),
                "os_name": self.info.get_os_name(),
                "os_version": self.info.get_os_version(),
                "kernel": self.info.get_kernel(),
            },
            "CPU": {
                "name": self.info.get_cpu_name(),
                "core": self.info.get_cpu_core(),
                "usage": self.info.get_cpu_usage(), // %
                "frequency": self.info.get_cpu_freq(), // MHz
                "temperature": self.info.get_cpu_temp(), // ℃
            },
            "RAM": {
                "total": self.info.get_ram_total(), // byte
                "usage": self.info.get_ram_usage(), // byte
                //TODO more on hardware
            },
            "SWAP": {
                "total": self.info.get_swap_total(), // byte
                "usage": &self.info.get_swap_usage(), // byte
            },
            "GPUs": gpus_json, // list
            "disks": disks_json, // list
            "networks": networks_json, // list
            "top_processes": top_processes_json, // list
            // "all_process": all_processes_json, // list
            "battery": {
                "percentage": self.info.get_battery_percentage(),
                "is_plugged_in": self.info.get_battery_is_plugged_in(), // bool, note that "plugged in" is different from "charging"
            },
            "softwares": {
                //TODO
            }
        });

        let pretty_json = serde_json::to_string_pretty(&payload).unwrap();
        let md_content = format!("```json\n{}\n```", pretty_json);
        fs::write("examples/others/json_sample.md", md_content).expect("Failed to write file");

        return payload;
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Agent running");

        loop {
            self.info.prepare();

            if let Err(e) = self.sender.transmit(self.get_json()) {
                eprintln!("Sender warning: {}", e);
            } else {
                println!("Sended info at timestamp {}", self.info.get_timestamp());
            }

            thread::sleep(Duration::from_secs(5));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};
    use super::DeviceAgent;
    

    #[test]
    fn test_json() {
        let mut agent = DeviceAgent::new();

        sleep(Duration::from_secs(1));

        agent.info.prepare();

        let payload = agent.get_json();

        let pretty_json = serde_json::to_string_pretty(&payload).unwrap();
        
        println!("\n=== GENERATED HARDWARE PAYLOAD ===");
        println!("{}", pretty_json);
        println!("==================================\n");
    }
}