
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Output};
use std::result::Result::Ok;
use std::io::{ErrorKind::PermissionDenied,BufRead, BufReader};
use sysinfo::Components;
use smbioslib::{SMBiosMemoryDevice,SMBiosPhysicalMemoryArray, SMBiosProcessorInformation, table_load_from_device};

use super::interface::OsSpecificInterface;
use super::parse_chassis_type;
use crate::types::*;

pub struct Temperature {
    components: Components,
}

impl Temperature {
    pub fn new() -> Self {
        Self { components: Components::new_with_refreshed_list() }
    }
}

impl TemperatureInterface for Temperature {
    fn ready_temp(&mut self) {
        self.components.refresh(true);
    }

    fn get_temp_cpu(&self) -> f32 {
        self.components.iter()
            .filter(|c| {
                let name = c.label().to_lowercase();
                name.contains("cpu") || name.contains("package") || name.contains("core")
            })
            .map(|c| c.temperature().unwrap() as f32)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0)
    }

    fn get_temp_mobo(&self) -> f32 {
        self.components.iter()
            .filter(|c| {
                let name = c.label().to_lowercase();
                name.contains("motherboard") || name.contains("system") || name.contains("thermal zone")
            })
            .map(|c| c.temperature().unwrap() as f32)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0) // ! Will likely return 0 if not run as Admin!
    }
}

pub struct Battery {
    // Empty as intended
}

impl Battery {
    pub fn new() -> Option<Self> {
        if let Some(_) = Self::run_wmic() {
            Some(Self {})
        } else { None }
    }

    fn run_wmic() -> Option<Output> {
        Command::new("wmic")
            .args(["path", "Win32_Battery", "get", "EstimatedChargeRemaining,BatteryStatus"])
            .output().ok()
    }
}

impl BatteryInterface for Battery {
    fn get_percentage(&self) -> f32 {
        if let Some(output) = Self::run_wmic() {
            let raw = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = raw.lines().filter(|l| !l.trim().is_empty()).collect();

            if lines.len() > 1 {
                let parts: Vec<&str> = lines[1].split_whitespace().collect();
                if parts.len() >= 2 {
                    return parts[1].parse::<f32>().unwrap_or(0.0);
                }
            }
        }
        0.0
    }

    fn get_is_plugged_in(&self) -> bool {
        if let Some(output) = Self::run_wmic() {
            let raw = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = raw.lines().filter(|l| !l.trim().is_empty()).collect();

            if lines.len() > 1 {
                let parts: Vec<&str> = lines[1].split_whitespace().collect();
                if parts.len() >= 2 {
                    return parts[0].parse::<u32>().unwrap_or(1) != 1;
                }
            }
        }
        false
    }
}

pub struct Machine {
    serial: String,
    architecture: String,
    os_name: String,
    producer: String,
    system_model: String,
    motherboard: String,
    machine_type: String,
}

impl Machine {
    pub fn new() -> Self {
        let run_wmic = |args: &[&str]| -> String {
            if let Ok(output) = Command::new("wmic").args(args).output() {
                let raw = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = raw.lines().filter(|l| !l.trim().is_empty()).collect();
                if lines.len() > 1 {
                    return lines[1].trim().to_string();
                }
            }
            "Unknown".to_string()
        };

        let serial = run_wmic(&["csproduct", "get", "identifyingnumber"]);
        let producer = run_wmic(&["csproduct", "get", "vendor"]);
        let model = run_wmic(&["csproduct", "get", "name"]);
        let mobo = run_wmic(&["baseboard", "get", "product"]);

        let raw_chassis = run_wmic(&["systemenclosure", "get", "chassistype"]);
        let chassis_code: String = raw_chassis.chars().filter(|c| c.is_ascii_digit()).collect();

        Self {
            serial: serial,
            architecture: std::env::consts::ARCH.to_string(),
            os_name: std::env::consts::OS.to_string(),
            producer: producer,
            system_model: model,
            motherboard: mobo,
            machine_type: parse_chassis_type(&chassis_code)
        }
    }
}

impl MachineInterface for Machine {
    fn get_serial(&self) -> &str {
        &self.serial
    }
    
    fn get_architecture(&self) -> &str {
        &self.architecture
    }

    fn get_os_name(&self) -> &str {
        &self.os_name
    }

    fn get_producer(&self) -> &str {
        &self.producer
    }

    fn get_system_model(&self) -> &str {
        &self.system_model
    }

    fn get_motherboard(&self) -> &str {
        &self.motherboard
    }

    fn get_machine_type(&self) -> &str {
        &self.machine_type
    }
}

#[derive(Debug)]
pub struct OsSpecificBackend {
    components: Components,

    product_serial: Option<String>,
    architecture: Option<String>,
    producer: Option<String>,
    system_model: Option<String>,
    machine_type: Option<String>,
    bios_vendor: Option<String>,
    bios_version: Option<String>,
    is_secure_boot: Option<bool>,

    os_name: Option<String>,

    mobo_name: Option<String>,
    mobo_serial: Option<String>,
    cpu_socket: Option<u32>,
    ram_socket: Option<u32>,
    gpu_socket: Option<u32>,


}

impl OsSpecificBackend {
    pub fn new() -> Self {
        Self {
            components: Components::new_with_refreshed_list(),
        }
    }
}

impl OsSpecificInterface for OsSpecificBackend {
    fn refresh(&mut self) {
        self.components.refresh(true);
    }

    fn get_product_serial(&self) -> String {
        
    }

    fn get_architecture(&self) -> String {
        
    }

    fn get_producer(&self) -> String {
        
    }

    fn get_system_model(&self) -> String {
        
    }

    fn get_machine_type(&self) -> String {
        
    }

    fn get_bios_version(&self) -> String {
        
    }

    fn get_bios_vendor(&self) -> String {
        
    }

    fn get_is_secure_boot(&self) -> Option<bool> {
        
    }

    fn get_os_name(&self) -> String {
        
    }

    fn get_motherboard(&self) -> String {
        
    }

    fn get_motherboard_serial(&self) -> String {
        
    }

    fn get_cpu_socket(&self) -> Option<u32> {
        
    }

    fn get_ram_socket(&self) -> Option<u32> {
        
    }

    fn get_gpu_socket(&self) -> Option<u32> {
        
    }

    fn get_ram_list(&self) -> Option<Vec<Ram>> {
        
    }

    fn get_physical_disk_list(&self) -> Option<Vec<PhysicalDisk>> {
        
    }

    fn get_tempe_mobo(&self) -> f32 {
        
    }

    fn get_tempe_cpu(&self) -> f32 {
        
    }

    fn get_battery_percentage(&self) -> f32 {
        
    }

    fn get_is_plugged_in(&self) -> bool {
        
    }

    fn fill_network_hardware(&self, network_list: &mut Vec<Network<'_>>) {
        
    }

    fn get_software_list(&self) -> Option<Vec<Software>> {
        
    }

}
