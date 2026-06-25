
use std::process::Output;
use sysinfo::Components;
use std::process::Command;

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


