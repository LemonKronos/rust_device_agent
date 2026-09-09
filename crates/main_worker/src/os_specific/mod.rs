
//! # OS-specific hardware abstraction
//!
//! This module provides a single, platform-independent interface for collecting
//! device information while keeping OS-specific implementation details isolated.
//!
//! `main_worker` should interact with [`OsSpecific`] rather than directly
//! accessing Linux, Windows, or macOS APIs.
//!
//! ## Architecture
//!
//! [`OsSpecific`] is a thin wrapper around an [`OsSpecificBackend`]. The concrete
//! backend is selected at compile time using `cfg(target_os = ...)`:
//!
//! - Linux -> [`linux::OsSpecificBackend`]
//! - Windows -> [`windows::OsSpecificBackend`]
//! - macOS -> [`macos::OsSpecificBackend`]
//!
//! The platform backends implement [`OsSpecificInterface`], allowing the rest of
//! the agent to use the same API regardless of the host operating system.
//!
//! ## Why this module exists
//!
//! Hardware and system information cannot be collected uniformly across
//! operating systems. For example, Windows uses WMI for much of its hardware
//! inventory, while Linux and macOS use different OS facilities.
//!
//! Keeping those differences behind this module prevents platform-specific
//! code from leaking into the main worker's business logic.
//!
//! ## Adding support for another OS
//!
//! A new platform should provide an implementation of [`OsSpecificInterface`]
//! and be selected here with an appropriate `cfg(target_os = ...)` block.
//!
//! The public API exposed to the rest of `main_worker` should remain unchanged.

pub mod interface;

use interface::OsSpecificInterface;
use shared_libs::types::*;

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

/// Platform-independent system information provider.
/// This type delegates all operations to the OS-specific backend selected at compile time.
#[derive(Debug)]
pub struct OsSpecific {
    backend: OsSpecificBackend,
}

impl OsSpecific {
    pub fn new() -> Self {
        Self { backend: OsSpecificBackend::new() }
    }
}
/// Just wrapper for interface
impl OsSpecificInterface for OsSpecific {
    fn refresh(&mut self) {
        self.backend.refresh();
    }

    fn get_product_serial(&self) -> Option<&str> {
        self.backend.get_product_serial()
    }

    fn get_architecture(&self) -> &str {
        self.backend.get_architecture()
    }

    fn get_producer(&self) -> &str {
        self.backend.get_producer()
    }

    fn get_system_model(&self) -> &str {
        self.backend.get_system_model()
    }

    fn get_machine_type(&self) -> &str {
        self.backend.get_machine_type()
    }

    fn get_bios_version(&self) -> &str {
        self.backend.get_bios_version()    
    }

    fn get_bios_vendor(&self) -> &str {
        self.backend.get_bios_vendor()
    }

    fn get_is_secure_boot(&self) -> Option<bool> {
        self.backend.get_is_secure_boot()
    }

    fn get_os_name(&self) -> &str {
        self.backend.get_os_name()
    }

    fn get_motherboard(&self) -> &str {
        self.backend.get_motherboard()
    }

    fn get_motherboard_serial(&self) -> Option<&str> {
        self.backend.get_motherboard_serial()
    }

    fn get_cpu_socket(&self) -> Option<u64> {
        self.backend.get_cpu_socket()
    }

    fn get_ram_socket(&self) -> Option<u64> {
        self.backend.get_ram_socket()
    }

    fn get_gpu_socket(&self) -> Option<u64> {
        self.backend.get_gpu_socket()
    }

    fn get_ram_list(&self) -> Option<&Vec<Ram>> {
        self.backend.get_ram_list()
    }

    fn get_physical_disk_list(&self) -> Option<&Vec<PhysicalDisk>> {
        self.backend.get_physical_disk_list()    
    }

    fn get_tempe_mobo(&self) -> Option<f64> {
        self.backend.get_tempe_mobo()
    }

    fn get_tempe_cpu(&self) -> Option<f64> {
        self.backend.get_tempe_cpu()
    }

    fn get_battery_percentage(&self) -> Option<u64> {
        self.backend.get_battery_percentage()
    }

    fn get_is_plugged_in(&self) -> Option<bool> {
        self.backend.get_is_plugged_in()
    }

    fn fill_network_hardware(&self, network_list: &mut Vec<shared_libs::types::Network<'_>>) {
        self.backend.fill_network_hardware(network_list);
    }

    fn get_software_list(&self) -> Option<Vec<Software>> {
        self.backend.get_software_list()
    }
}



/// Decode the SMBIOS/DMI chassis type code into a human-readable category.
/// This is primarily used by platform implementations that obtain the numerical chassis type from firmware or WMI.
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