
use std::fs;
use std::thread;
use std::time::Duration;
use serde_json::json;

pub mod os_specific;
pub mod info_gatherer;
pub mod sender;
pub mod utils;

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
            })
        }).collect();

        let disks_json: Vec<_> = self.info.get_disk_info().iter().map(|disk| {
            json!({
                "name": disk.get_name(),
                "type": disk.get_type(),
                "removable": disk.get_removable(), // bool
                "total": disk.get_total(), // byte
                "used": disk.get_used(), // byte
            })
        }).collect();

        let networks_json: Vec<_> = self.info.get_networks().iter().map(|network| {
            json!({
                "name": network.get_name(),
                "ipv4": network.get_ipv4(),
                "ipv6": network.get_ipv6(),
                "mac": network.get_mac(),
                "mtu": network.get_mtu(), // byte
                "upload": network.get_upload(), // byte
                "download": network.get_download(), // byte
            })
        }).collect();

        let processes_json: Vec<_> = self.info.get_running().iter().map(|process| {
            json!({
                "name": process.get_name(),
                "cpu": process.get_cpu_usage(), // %
                "memory": process.get_memory(), // byte
                "runtime": process.get_runtime(), // second
            })
        }).collect();

        let payload = json!({
            "uuid": self.info.get_uuid().to_string(),
            "time_stamp": self.info.get_timestamp(), // second
            "machine": {   
                "architecture": self.info.get_architecture(),
                "os_name": self.info.get_os_name(),
                "producer": self.info.get_producer(),
                "model": self.info.get_system_model(),
                "motherboard": self.info.get_motherboard(),
                "machine_type": self.info.get_machine_type(),
            },
            "system": {
                "os": self.info.get_os(),
                "os_version": self.info.get_os_version(),
                "kernel": self.info.get_kernel(),
                "boot_time": self.info.get_boot_time(), // second
                "run_time": self.info.get_run_time(), // second
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
            },
            "SWAP": {
                "total": self.info.get_swap_total(), // byte
                "usage": &self.info.get_swap_usage(), // byte
            },
            "GPUs": gpus_json, // list
            "disks": disks_json, // list
            "networks": networks_json, // list
            "processes": processes_json, // list
            "battery": {
                "percentage": self.info.get_battery_percentage(),
                "is_plugged_in": self.info.get_battery_is_plugged_in(), // bool, note that "plugged in" is different from "charging"
            },
            "temperature": self.info.get_temp_mobo(), // ℃
        });

        let pretty_json = serde_json::to_string_pretty(&payload).unwrap();
        let md_content = format!("```json\n{}\n```", pretty_json);
        fs::write("json_sample.md", md_content).expect("Failed to write file");

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
    use std::{fs, thread::sleep, time::Duration};
    use crate::info_gatherer::Info;
    use serde_json::json;
    

    #[test]
    fn print_json() {
        let mut info = Info::new();

        sleep(Duration::from_secs(1));
        info.prepare();

        let gpus_json: Vec<_> = info.get_gpu_list().iter().map(|gpu| {
            json!({
                "name": gpu.get_name(),
                "driver": gpu.get_driver(),
                "utilization": gpu.get_util(), // %
                "temperature": gpu.get_temp(), // ℃
                "vram_total": gpu.get_vram_total(), // byte
                "vram_usage": gpu.get_vram_usage(), // byte
            })
        }).collect();

        let disks_json: Vec<_> = info.get_disk_info().iter().map(|disk| {
            json!({
                "name": disk.get_name(),
                "type": disk.get_type(),
                "removable": disk.get_removable(), // bool
                "total": disk.get_total(), // byte
                "used": disk.get_used(), // byte
            })
        }).collect();

        let networks_json: Vec<_> = info.get_networks().iter().map(|network| {
            json!({
                "name": network.get_name(),
                "ipv4": network.get_ipv4(),
                "ipv6": network.get_ipv6(),
                "mac": network.get_mac(),
                "mtu": network.get_mtu(), // byte
                "upload": network.get_upload(), // byte
                "download": network.get_download(), // byte
            })
        }).collect();

        let processes_json: Vec<_> = info.get_running().iter().map(|process| {
            json!({
                "name": process.get_name(),
                "cpu": process.get_cpu_usage(), // %
                "memory": process.get_memory(), // byte
                "runtime": process.get_runtime(), // second
            })
        }).collect();

        let payload = json!({
            "uuid": info.get_uuid(),
            "machine": {   
                "architecture": info.get_architecture(),
                "os_name": info.get_os_name(),
                "producer": info.get_producer(),
                "model": info.get_system_model(),
                "motherboard": info.get_motherboard(),
                "machine_type": info.get_machine_type(),
            },
            "system": {
                "os": info.get_os(),
                "os_version": info.get_os_version(),
                "kernel": info.get_kernel(),
                "boot_time": info.get_boot_time(), // second
                "run_time": info.get_run_time(), // second
            },
            "CPU": {
                "name": info.get_cpu_name(),
                "core": info.get_cpu_core(),
                "usage": info.get_cpu_usage(), // %
                "frequency": info.get_cpu_freq(), // MHz
                "temperature": info.get_cpu_temp(), // ℃
            },
            "RAM": {
                "total": info.get_ram_total(), // byte
                "usage": info.get_ram_usage(), // byte
            },
            "SWAP": {
                "total": info.get_swap_total(), // byte
                "usage": &info.get_swap_usage(), // byte
            },
            "GPUs": gpus_json, // list
            "disks": disks_json, // list
            "networks": networks_json, // list
            "processes": processes_json, // list
            "battery": {
                "percentage": info.get_battery_percentage(),
                "is_plugged_in": info.get_battery_is_plugged_in(), // bool, note that "plugged in" is different from "charging"
            },
            "temperature": info.get_temp_mobo(), // ℃
        });

        let pretty_json = serde_json::to_string_pretty(&payload).unwrap();
        
        println!("\n=== GENERATED HARDWARE PAYLOAD ===");
        println!("{}", pretty_json);
        println!("==================================\n");

        let md_content = format!("```json\n{}\n```", pretty_json);
        fs::write("json_sample.md", md_content).expect("Failed to write file");
    }
}