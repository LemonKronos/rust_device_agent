
pub mod interface;

use interface::BatteryInterface;
use interface::MachineInterface;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{Battery as BatteryBackend, Machine as MachineBackend};


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
    fn get_percentage(&self) -> u32 {
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