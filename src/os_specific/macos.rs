
use std::fs;
use std::process::Output;
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

    fn get_temp_cpu(&self) -> u32 {
        self.components.iter()
            .filter(|c| {
                let name = c.label().to_lowercase();
                // Catches both Intel and Apple Silicon CPU sensors
                name.contains("tc0") || name.contains("pmu tdie") || name.contains("acc mtr")
            })
            .map(|c| c.temperature().unwrap() as u32)
            .max()
            .unwrap_or(0)
    }

    fn get_temp_mobo(&self) -> u32 {
        self.components.iter()
            .filter(|c| {
                let name = c.label().to_lowercase();
                name.contains("tm0p") || name.contains("soc mtr")
            })
            .map(|c| c.temperature().unwrap() as u32)
            .max()
            .unwrap_or(0)
    }
}

pub struct Battery {
    // Empty as intended
}

impl Battery {
    pub fn new() -> Option<Self> {
        if let Some(_) = Self::run_pmset() {
            Some(Self {})
        } else {None}
    }

    fn run_pmset() -> Option<Output> {
        Command::new("pmset").args(["-g", "batt"]).output().ok()
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
        let run_sysctl = |key: &str| -> String {
            if let Ok(output) = Command::new("sysctl").args(["-n", key]).output() {
                return  String::from_utf8_lossy(&output.stdout).trim().to_string();
            }
            "Unknown".to_string()
        };

        let producer = "Apple Inc.".to_string();
        let model = run_sysctl("hw.model");
        let mobo = run_sysctl("hw.board-id");

        // Mac don't use the code, so we just guess
        let machineid_type = if model.contains("MacBook") {
            "Laptop".to_string()
        } else {
            "Desktop".to_string()
        };

        Self {
            architecture: std::env::consts::ARCH.to_string(),
            os_name: std::env::consts::OS.to_string(),
            producer: producer,
            system_model: model,
            motherboard: mobo,
            machine_type: machineid_type
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
