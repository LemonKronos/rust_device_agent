
use std::fs;
use std::process::Command;

#[derive(Debug)]
pub struct Battery {
    pub percentage: u32,
    pub is_plugged_in: bool,
}

#[derive(Debug)]
pub struct OsHandler {
    pub architecture: String,
    pub os_name: String,
    pub producer: String,
    pub system_model: String,
    pub motherboard: String,
    pub machine_type: String,
}

impl OsHandler {
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

    pub fn get_battery(&self) -> Option<Battery> {
        self.read_battery_backend()
    }

    //_ Linux
    #[cfg(target_os = "linux")]
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

    #[cfg(target_os = "linux")]
    fn read_battery_backend(&self) -> Option<Battery> {
        let base_path = if fs::metadata("/sys/class/power_supply/BAT0").is_ok() {
            "/sys/class/power_supply/BAT0"
        } else if fs::metadata("/sys/class/power_supply/BAT1").is_ok() {
            "/sys/class/power_supply/BAT1"
        } else {
            return None;
        };

        let capacity_str = fs::read_to_string(format!("{}/capacity", base_path)).ok()?;
        let status_str = fs::read_to_string(format!("{}/status", base_path)).ok()?;

        let percentage = capacity_str.trim().parse::<u32>().unwrap_or(0);
        // If plugged-in == battery NOT discharging
        let is_plugged_in = status_str.trim().to_lowercase() != "discharging";

        Some(Battery { percentage, is_plugged_in })
    }

    //_ Window
    #[cfg(target_os = "windows")]
    fn fetch_hardware_data() -> (String, String, String, String) {
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

        let producer = run_wmic(&["csproduct", "get", "vendor"]);
        let model = run_wmic(&["csproduct", "get", "name"]);
        let mobo = run_wmic(&["baseboard", "get", "product"]);

        let raw_chassis = run_wmic(&["systemenclosure", "get", "chassistype"]);
        let chassis_code: String = raw_chassis.chars().filter(|c| c.is_ascii_digit()).collect();

        (producer, model, mobo, parse_chassis_type(&chassis_code))
    }

    #[cfg(target_os = "windows")]
    fn read_battery_backend(&self) -> Option<Battery> {
        let output = Command::new("wmic")
            .args(["path", "Win32_Battery", "get", "EstimatedChargeRemaining,BatteryStatus"])
            .output().ok()?;

        let raw = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = raw.lines().filter(|l| !l.trim().is_empty()).collect();

        if lines.len() > 1 {
            let parts: Vec<&str> = lines[1].split_whitespace().collect();
            if parts.len() >= 2 {
                let status_code = parts[0].parse::<u32>().unwrap_or(1);
                let percentage = parts[1].parse::<u32>().unwrap_or(0);

                let is_plugged_in = status_code != 1;

                return Some(Battery { percentage, is_plugged_in });
            }
        }
        None
    }

    //_ Mac
    #[cfg(target_os = "macos")]
    fn fetch_hardware_data() -> (String, String, String, String) {
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

        (producer,  model, mobo, machineid_type)
    }

    #[cfg(target_os = "macos")]
    fn read_battery_backend(&self) -> Option<Battery> {
        let output = Command::new("pmset").args(["-g", "batt"]).output().ok()?;
        let raw = String::from_utf8_lossy(&output.stdout);

        if !raw.contains('%') {return None;}

        // Mac show clearly it power supply
        let is_plugged_in = raw.contains("AC Power");

        let pct_line = raw.lines().find(|l| l.contains('%'))?;
        let parts: Vec<&str> = pct_line.split('\t').collect();
        let data_part = parts.get(1)?;
        let pct_str = data_part.split(';').next()?.replace('%', "").trim().to_string();

        let percentage = pct_str.parse::<u32>().unwrap_or(0);

        Some(Battery { percentage, is_plugged_in })
    }
}

//#TAG: Helper
fn parse_chassis_type(code: &str) -> String {
    match code {
        "3" | "4" | "6" | "7"   => "Desktop".to_string(),
        "8" | "9" | "10" | "14" => "Laptop / Notebook".to_string(),
        "11"                    => "Hand Held".to_string(),
        "17" | "23"             => "Server".to_string(),
        "30"                    => "Tablet".to_string(),
        "31" | "32"             => "Convertible".to_string(),
        _                       => format!("Unknow (Code {})", code)
    }
}


//#TAG: Test
#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_full_hardware_read() {
        let handler = OsHandler::new();
        println!("Hardware scan complete:");
        println!("{:#?}", handler);

        assert!(!handler.producer.is_empty());
        assert!(!handler.system_model.is_empty());
    }

    #[test]
    fn test_battery_read() {
        let handler = OsHandler::new();
        if let Some(batt) = handler.get_battery() {
            println!("Battery: {}%, plugged In: {}", batt.percentage, batt.is_plugged_in);
            assert!(batt.percentage <= 100);
        } else {
            println!("No battery detected");
        }
    }
}
