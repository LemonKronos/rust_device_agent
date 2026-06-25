
use std::fs;
use sysinfo::Components;

use super::interface::TemperatureInterface;
use super::interface::BatteryInterface;
use super::interface::MachineInterface;
use super::parse_chassis_type;

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
        self.components
            .iter()
            .filter(|c| c.label().to_lowercase().contains("k10temp"))
            .map(|c| c.temperature().unwrap() as f32)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0)
    }

    fn get_temp_mobo(&self) -> f32 {
        self.components
            .iter()
            .filter(|c| c.label().to_lowercase().contains("acpitz"))
            .map(|c| c.temperature().unwrap() as f32)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0)
    }
}

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
    fn get_percentage(&self) -> f32 {
        fs::read_to_string(format!("{}/capacity", self.base_path))
            .unwrap_or_default()
            .trim()
            .parse()
            .unwrap_or(0.0)
    }

    fn get_is_plugged_in(&self) -> bool {
        fs::read_to_string(format!("{}/status", self.base_path))
            .unwrap_or_default()
            .trim()
            .to_lowercase() != "discharging"
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
        let read_dmi = |file: &str| match std::fs::read_to_string(format!("/sys/class/dmi/id/{}", file)) {
            Ok(s) => s.trim().to_string(),
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => "Required Admin".to_string(),
            Err(_) => "Unknown".to_string(),
        };

        let serial = read_dmi("product_serial");
        let producer = read_dmi("sys_vendor");
        let model = read_dmi("product_name");
        let mobo = read_dmi("board_name");
        let chassis_code = read_dmi("chassis_type");

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

