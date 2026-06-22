
use std::fs;

use super::interface::BatteryInterface;
use super::interface::MachineInterface;
use super::parse_chassis_type;

pub struct Battery {
    base_path: String,
}

impl Battery {
    pub fn new() -> Option<Self> {
        let path = if fs::metadata("/sys/class/power_supply/BAT0").is_ok() {
            "/sys/class/power_supply/BAT0".to_string()
        } else if fs::metadata("/sys/class/power_supply/BAT1").is_ok() {
            "/sys/class/power_supply/BAT1".to_string()
        } else {
            return None;
        };

        Some(Self {
            base_path: path.to_string()
        })
    }
}

impl BatteryInterface for Battery {
    fn get_percentage(&self) -> u32 {
        fs::read_to_string(format!("{}/capacity", self.base_path))
            .unwrap_or_default()
            .trim()
            .parse()
            .unwrap_or(0)
    }

    fn get_is_plugged_in(&self) -> bool {
        fs::read_to_string(format!("{}/status", self.base_path))
            .unwrap_or_default()
            .trim()
            .to_lowercase() != "discharging"
    }
}

pub struct Machine {
    architecture: String,
    os_name: String,
    producer: String,
    system_model: String,
    motherboard: String,
    machine_type: String,
}

impl Machine {
    pub fn new() -> Self {
        let (producer, system_model, motherboard, machine_type) = Self::fetch_hardware_data();

        Self {
            architecture: std::env::consts::ARCH.to_string(),
            os_name: std::env::consts::OS.to_string(),
            producer,
            system_model,
            motherboard,
            machine_type
        }
    }
}

impl MachineInterface for Machine {
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

impl Machine {
    fn fetch_hardware_data() -> (String, String, String, String) {
        let read_dmi = |file: &str| -> String {
            fs::read_to_string(format!("/sys/class/dmi/id/{}", file))
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| "Unknow".to_string())
        };

        let producer = read_dmi("sys_vendor");
        let model = read_dmi("product_name");
        let mobo = read_dmi("board_name");
        let chassis_code = read_dmi("chassis_type");

        (producer, model, mobo, parse_chassis_type(&chassis_code))
    }
}

