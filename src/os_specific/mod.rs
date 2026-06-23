
pub mod interface;

use interface::TemperatureInterface;
use interface::BatteryInterface;
use interface::MachineInterface;

//_ Linux
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{
    Temperature as TemperatureBackend,
    Battery as BatteryBackend,
    Machine as MachineBackend
};

//_ Windows
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{
    Temperature as TemperatureBackend,
    Battery as BatteryBackend,
    Machine as MachineBackend
};

//_ MacOS
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{
    Temperature as TemperatureBackend,
    Battery as BatteryBackend,
    Machine as MachineBackend
};

//_ OS plexers
pub struct Temperature {
    backend: TemperatureBackend,
}

impl Temperature {
    pub fn new() -> Self {
        Self { backend: TemperatureBackend::new() }
    }
}

impl TemperatureInterface for Temperature {
    fn ready_temp(&mut self) {
        self.backend.ready_temp()
    }

    fn get_temp_cpu(&self) -> f32 {
        self.backend.get_temp_cpu()
    }

    fn get_temp_mobo(&self) -> f32 {
        self.backend.get_temp_mobo()
    }
}

pub struct Battery {
    backend: BatteryBackend,
}

impl Battery {   
    pub fn new() -> Option<Self> {
        let backend = BatteryBackend::new()?;
        Some(Self { backend })
    }
}

impl BatteryInterface for Battery {
    fn get_percentage(&self) -> f32 {
        self.backend.get_percentage()
    }

    fn get_is_plugged_in(&self) -> bool {
        self.backend.get_is_plugged_in()
    }
}

pub struct Machine {
    backend: MachineBackend,
}

impl Machine {
    pub fn new() -> Self {
        Self { backend: MachineBackend::new() }
    }
}

impl MachineInterface for Machine {
    fn get_architecture(&self) -> &str {
        self.backend.get_architecture()
    }

    fn get_os_name(&self) -> &str {
        self.backend.get_os_name()
    }

    fn get_producer(&self) -> &str {
        self.backend.get_producer()
    }

    fn get_system_model(&self) -> &str {
        self.backend.get_system_model()
    }

    fn get_motherboard(&self) -> &str {
        self.backend.get_motherboard()
    }

    fn get_machine_type(&self) -> &str {
        self.backend.get_machine_type()
    }
}

//_ Helpers
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