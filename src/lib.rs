
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

        let rams_json = self.info.get_ram_list().map(|ram| {
            ram.iter().map(|ram| json!({
                "name": ram.get_name(),
                "serial": ram.get_serial(),
                "type": ram.get_type(),
                "speed": ram.get_speed(), // MT/s
                "size": ram.get_size(),
            })).collect::<Vec<_>>()
        });

        let gpus_json: Vec<_> = self.info.get_gpu_list().iter().map(|gpu| {
            json!({
                "name": gpu.get_name(),
                "driver": gpu.get_driver(),
                "utilization": gpu.get_util(), // %
                "temperature": gpu.get_temp(), // ℃
                "frequency": gpu.get_freq(), // MHz
                "vram_total": gpu.get_vram_total(), // byte
                "vram_usage": gpu.get_vram_usage(), // byte
                "max_clock": gpu.get_max_clock(), // MHz
                "serial": gpu.get_serial(),
                //TODO screen resolution + serial
            })
        }).collect();

        let logical_disks_json: Vec<_> = self.info.get_logical_disk_list().iter().map(|disk| {
            json!({
                "name": disk.get_name(),
                "type": disk.get_file_system(),
                "mount_point": disk.get_mount_point(),
                "removable": disk.get_removable(), // bool
                "total": disk.get_total(), // byte
                "used": disk.get_used(), // byte
            })
        }).collect();

        let hardware_disks_json = self.info.get_hardware_disk_list().map(|hd| {
            hd.iter().map(|hd| json!({
                "drive": hd.get_drive(),
                "model": hd.get_model(),
                "serial": hd.get_serial(),
                "firmware": hd.get_firmware(),
                "size": hd.get_size(), // GB
            })).collect::<Vec<_>>()
        });

        let networks_json: Vec<_> = self.info.get_network_list().iter().map(|network| {
            json!({
                "name": network.get_name(),
                "ipv4": network.get_ipv4(),
                "ipv6": network.get_ipv6(),
                "mac": network.get_mac(),
                "mtu": network.get_mtu(), // byte
                "upload": network.get_upload(), // byte
                "download": network.get_download(), // byte
                "card": network.get_card(),
                "config_speed": network.get_config_speed(), // Mbps
                "ssid": network.get_ssid(),
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

        // let all_processes_json: Vec<_> = self.info.get_full_process_list().iter().map(|process| {
        //     json!({
        //         "name": process.get_name(),
        //         "cpu": process.get_cpu_usage(),
        //         "memory": process.get_memory(),
        //         "runtime": process.get_runtime(),
        //     })
        // }).collect();

        //_ this have take(5) for debug
        let softwares_json = self.info.get_software_list().map(|app| {
            app.iter().take(5).map(|app| json!({
                "name": app.get_name(),
                "version": app.get_version(),
                "source": app.get_source(),
                "license": "todo",
                "expiration": "todo",
            })).collect::<Vec<_>>()
        });

        //TODO
        // let peripheral_json: Vec<_> = 

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
                "cpu_socket": self.info.get_cpu_socket(),
                "ram_socket": self.info.get_ram_socket(),
                "gpu_socket": self.info.get_gpu_socket(),
            },
            "bios": {
                "version": self.info.get_os_version(),
                "vendor": self.info.get_bios_vendor(),
                "secure_boot": self.info.get_is_secure_boot(),
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
                "hardware": rams_json, // list
            },
            "SWAP": {
                "total": self.info.get_swap_total(), // byte
                "usage": &self.info.get_swap_usage(), // byte
            },
            "GPUs": gpus_json, // list
            "disks": {
                "logical": logical_disks_json, // list
                "hardware": hardware_disks_json, // list
            },
            "networks": networks_json, // list
            "top_processes": top_processes_json, // list
            // "all_process": all_processes_json, // list
            "battery": {
                "percentage": self.info.get_battery_percentage(),
                "is_plugged_in": self.info.get_battery_is_plugged_in(), // bool, note that "plugged in" is different from "charging"
            },
            "softwares": softwares_json,
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