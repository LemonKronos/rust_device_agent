
use std::fs;
use sysinfo::Components;

use super::interface::OsSpecificInterface;
use super::parse_chassis_type;

const HARDWARE_INFO_PATH: &str = "/sys/class/dmi/id";
const BATTERY_0_PATH: &str = "/sys/class/power_supply/BAT0";
const BATTERY_1_PATH: &str = "/sys/class/power_supply/BAT1";

#[derive(Debug)]
pub struct OsSpecificBackend {
    components: Components,
    battery_path: Option<String>,
}

impl OsSpecificBackend {
    pub fn new() -> Self {
        let battery_path = if fs::metadata(BATTERY_0_PATH).is_ok() {
            Some(BATTERY_0_PATH.to_string())
        } else if fs::metadata(BATTERY_1_PATH).is_ok() {
            Some(BATTERY_1_PATH.to_string())
        } else {
            None
        };

        Self {
            components: Components::new_with_refreshed_list(),
            battery_path: battery_path,
        }
    }

    fn parse_file(path: &str, file: &str) -> String {
        match std::fs::read_to_string(format!("{}/{}", path, file)) {
            Ok(s) => s.trim().to_string(),
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => "Required Admin".to_string(),
            // Err(e) if e.kind() != std::io::ErrorKind::PermissionDenied => format!("Error {}", e.kind()), // for debug
            Err(_) => "Unknown".to_string(),
        }
    }
}

impl OsSpecificInterface for OsSpecificBackend {
    fn refresh(&mut self) {
        self.components.refresh(true);

        self.battery_path = if fs::metadata(BATTERY_0_PATH).is_ok() {
            Some(BATTERY_0_PATH.to_string())
        } else if fs::metadata(BATTERY_1_PATH).is_ok() {
            Some(BATTERY_1_PATH.to_string())
        } else {
            None
        };
    }

    fn get_product_serial(&self) -> String {
        Self::parse_file(HARDWARE_INFO_PATH, "product_serial")
    }

    fn get_architecture(&self) -> String {
        std::env::consts::ARCH.to_string()
    }

    fn get_producer(&self) -> String {
        Self::parse_file(HARDWARE_INFO_PATH, "sys_vendor")
    }

    fn get_system_model(&self) -> String {
        Self::parse_file(HARDWARE_INFO_PATH, "product_name")
    }

    fn get_machine_type(&self) -> String {
        let chassis_code = Self::parse_file(HARDWARE_INFO_PATH, "chassis_type");
        parse_chassis_type(&chassis_code)
    }

    fn get_os_name(&self) -> String {
        std::env::consts::OS.to_string()
    }

    fn get_motherboard(&self) -> String {
        Self::parse_file(HARDWARE_INFO_PATH, "board_name")
    }

    fn get_motherboard_serial(&self) -> String {
        Self::parse_file(HARDWARE_INFO_PATH, "board_serial")
    }

    fn get_cpu_slot(&self) -> u32 {
        todo!("Call Admin fetcher")
    }
    
    fn get_ram_slot(&self) -> u32 {
        todo!("Call Admin fetcher")
    }

    fn get_gpu_slot(&self) -> u32 {
        todo!("Do we look for GPU slot?")
    }

    fn get_tempe_mobo(&self) -> f32 {
        self.components
            .iter()
            .filter(|c| c.label().to_lowercase().contains("acpitz"))
            .map(|c| c.temperature().unwrap() as f32)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0)
    }

    fn get_tempe_cpu(&self) -> f32 {
        self.components
            .iter()
            .filter(|c| c.label().to_lowercase().contains("k10temp"))
            .map(|c| c.temperature().unwrap() as f32)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0)
    }

    fn get_percentage(&self) -> f32 {
        if let Some(path) = &self.battery_path {
            Self::parse_file(path, "capacity")
                .trim().parse().unwrap_or(0.0)
        } else {
            0.0
        }
    }

    fn get_is_plugged_in(&self) -> bool {
        if let Some(path) = &self.battery_path {
            Self::parse_file(path, "status")
                .trim().to_lowercase() != "discharging"
        } else {
            true
        }
    }
}
