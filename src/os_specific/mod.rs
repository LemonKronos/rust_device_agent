
pub mod interface;

use interface::OsSpecificInterface;

//_ Linux
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::OsSpecificBackend;

//_ Windows
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{OsSpecificBackend};

//_ MacOS
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{OsSpecificBackend};

//_ OS plexers
#[derive(Debug)]
pub struct OsSpecific {
    backend: OsSpecificBackend,
}

impl OsSpecific {
    pub fn new() -> Self {
        Self { backend: OsSpecificBackend::new() }
    }
}

impl OsSpecificInterface for OsSpecific {
    fn refresh(&mut self) {
        self.backend.refresh();
    }

    fn get_product_serial(&self) -> String {
        self.backend.get_product_serial()
    }

    fn get_architecture(&self) -> String {
        self.backend.get_architecture()
    }

    fn get_producer(&self) -> String {
        self.backend.get_producer()
    }

    fn get_system_model(&self) -> String {
        self.backend.get_system_model()
    }

    fn get_machine_type(&self) -> String {
        self.backend.get_machine_type()
    }

    fn get_os_name(&self) -> String {
        self.backend.get_os_name()
    }

    fn get_motherboard(&self) -> String {
        self.backend.get_motherboard()
    }

    fn get_motherboard_serial(&self) -> String {
        self.backend.get_motherboard_serial()
    }

    fn get_cpu_slot(&self) -> u32 {
        self.backend.get_cpu_slot()
    }

    fn get_ram_slot(&self) -> u32 {
        self.backend.get_ram_slot()
    }

    fn get_gpu_slot(&self) -> u32 {
        self.backend.get_gpu_slot()
    }

    fn get_tempe_mobo(&self) -> f32 {
        self.backend.get_tempe_mobo()
    }

    fn get_tempe_cpu(&self) -> f32 {
        self.backend.get_tempe_cpu()
    }

    fn get_percentage(&self) -> f32 {
        self.backend.get_percentage()
    }

    fn get_is_plugged_in(&self) -> bool {
        self.backend.get_is_plugged_in()
    }
}



//_ Helpers
fn parse_chassis_type(code: &str) -> String {
    match code {
        "1" => "Special: Other".to_string(),
        "2" => "Special: Unknown".to_string(),
        "3" => "Desktop: Desktop".to_string(),
        "4" => "Desktop: Low Profile Desktop".to_string(),
        "5" => "Desktop: Pizza Box".to_string(),
        "6" => "Desktop: Mini Tower".to_string(),
        "7" => "Desktop: Tower".to_string(),
        "8" => "Laptop: Portable".to_string(),
        "9" => "Laptop: Laptop".to_string(),
        "10" => "Laptop: Notebook".to_string(),
        "11" => "Mobile: Hand Held".to_string(),
        "12" => "Accessory: Docking Station".to_string(),
        "13" => "Desktop: All in One".to_string(),
        "14" => "Laptop: Sub Notebook".to_string(),
        "15" => "Desktop: Space-Saving".to_string(),
        "16" => "Portable: Lunch Box".to_string(),
        "17" => "Server: Main System Chassis".to_string(),
        "18" => "Accessory: Expansion Chassis".to_string(),
        "19" => "Component: SubChassis".to_string(),
        "20" => "Component: Bus Expansion Chassis".to_string(),
        "21" => "Accessory: Peripheral Chassis".to_string(),
        "22" => "Storage: RAID Chassis".to_string(),
        "23" => "Server: Rack Mount Chassis".to_string(),
        "24" => "Embedded: Sealed-Case PC".to_string(),
        "25" => "Server: Multi-system chassis".to_string(),
        "26" => "Embedded: Compact PCI".to_string(),
        "27" => "Embedded: Advanced TCA".to_string(),
        "28" => "Server: Blade".to_string(),
        "29" => "Server: Blade Enclosure".to_string(),
        "30" => "Mobile: Tablet".to_string(),
        "31" => "Mobile: Convertible".to_string(),
        "32" => "Mobile: Detachable".to_string(),
        "33" => "Embedded: IoT Gateway".to_string(),
        "34" => "Embedded: Embedded PC".to_string(),
        "35" => "Desktop: Mini PC".to_string(),
        "36" => "Desktop: Stick PC".to_string(),
        _ => format!("Unregistered (code {})", code),
    }
}