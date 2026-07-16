
use std::collections::HashMap;
use std::error::Error;
use std::process::Command;
use std::result::Result::Ok;
use std::time::SystemTime;
use sysinfo::Components;
use wmi::WMIConnection;
use windows_sys::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use winreg::RegKey;
use chrono::{NaiveDate, TimeZone, Utc};

use super::interface::OsSpecificInterface;
use super::parse_chassis_type;
use crate::types::*;
use crate::utils::FormatMem;

mod wmi_class_type;
use wmi_class_type::*;

#[derive(Debug, Default)]
struct CachedInfo {
    product_serial: Option<String>,
    architecture: Option<String>,
    producer: Option<String>,
    system_model: Option<String>,
    machine_type: Option<String>,
    bios_vendor: Option<String>,
    bios_version: Option<String>,
    is_secure_boot: Option<bool>,

    os_name: Option<String>,

    mobo_name: Option<String>,
    mobo_serial: Option<String>,
    cpu_socket: Option<u32>,
    ram_socket: Option<u32>,
    gpu_socket: Option<u32>,

    ram_list: Option<Vec<Ram>>,
    physical_disk_list: Option<Vec<PhysicalDisk>>,
    network_hardware: Option<HashMap<String,NetworkHardware>>,
}

#[derive(Debug, Default)]
pub struct OsSpecificBackend {
    components: Components,
    cached_info: Option<CachedInfo>,
}

impl OsSpecificBackend {
    pub fn new() -> Self {
        let cached_info = match Self::set_cached_info() {
            Ok(info) => Some(info),
            Err(e) => {
                eprintln!("Read cached info error: {}", e);
                None
            }
        };

        Self {
            components: Components::new_with_refreshed_list(),
            cached_info: cached_info,
        }
    }

    /// Initialize WMI connection once and cache these info
    fn set_cached_info() -> Result<CachedInfo, Box<dyn Error>> {
        let mut info = CachedInfo::default();

        let wmi = WMIConnection::new()?;

        if let Ok(cs_prod) = wmi.query::<WmiCsProduct>() {
            if let Some(data) = cs_prod.first() {
                info.product_serial = data.identifying_number.clone();
                info.producer = data.vendor.clone();
                info.system_model = data.name.clone();
            }
        }

        if let Ok(sys) = wmi.query::<WmiComputerSystem>() {
            if let Some(data) = sys.first() {
                info.architecture = data.system_type.clone();
                info.cpu_socket = data.number_of_processors;
            }
        }

        if let Ok(chassis) = wmi.query::<WmiEnclosure>() {
            if let Some(data) = chassis.first()
                .and_then(|e| e.chassis_types.as_ref())
                .and_then(|types| types.first())
            {
                info.machine_type = Some(parse_chassis_type(&data.to_string()));
            }
        }

        if let Ok(bios) = wmi.query::<WmiBios>() {
            if let Some(data) = bios.first() {
                info.bios_vendor = data.manufacturer.clone();
                info.bios_version = data.smbiosbiosversion.clone();
            }
        }

        if let Ok(os) = wmi.query::<WmiOs>() {
            if let Some(data) = os.first() {
                info.os_name = data.caption.clone();
            }
        }

        if let Ok(mobo) = wmi.query::<WmiBaseBoard>() {
            if let Some(data) = mobo.first() {
                info.mobo_name = data.product.clone();
                info.mobo_serial = data.serial_number.clone();
            }
        }

        // ! This is CPU socket design, not number
        // if let Ok(cpu) = wmi.query::<WmiProcessor>() {
        //     if let Some(data) = cpu.first() {
        //         info.cpu_socket = data.socket_designation.clone();
        //     }
        // }

        if let Ok(ram_array) = wmi.query::<WmiMemoryArray>() {
            if let Some(data) = ram_array.first() {
                info.ram_socket = data.memory_devices;
            }
        }

        if let Ok(slots) = wmi.query::<WmiSystemSlot>() {
            let mut gpu_slots = 0;

            for slot in slots {
                if let Some(width) = slot.max_data_width {
                    if width == 16 { // GPU usually use PCIe x16
                        gpu_slots += 1;
                    }
                } else if let Some(designation) = &slot.slot_designation {
                    if designation.to_uppercase().contains("X16") { // fall back to string
                        gpu_slots += 1;
                    }
                }
            }
            if gpu_slots > 0 {
                info.gpu_socket = Some(gpu_slots);
            }
        }

        if let Ok(ram_modules) = wmi.query::<WmiMemory>() {
            let mut ram_list = Vec::new();
            for stick in ram_modules {
                let size = stick.capacity.unwrap_or_else(|| "0".to_string()).parse::<u64>().unwrap_or(0);

                let speed = stick.speed.map(|s| format!("{} MHz", s)).unwrap_or_else(|| "Unknown".to_string());

                ram_list.push(Ram {
                    name: stick.manufacturer.unwrap_or_else(|| "Unknown".to_string()).trim().to_string(),
                    serial: stick.serial_number.unwrap_or_else(|| "Unknown".to_string()).trim().to_string(),
                    type_: parse_ram_type(stick.memory_type, stick.smbiosmemory_type),
                    speed: speed,
                    size: size,
                    bank: stick.bank_label.unwrap_or_else(|| "Unknown".to_string()),
                    form_factor: parse_form_factor(stick.form_factor),
                });
            }
            info.ram_list = Some(ram_list);
        }

        // : Read physical disk
        let mut disk_list = Vec::new();
        let mut partition_map: HashMap<u32, Vec<Partition>> = HashMap::new();

        if let Ok(partitions) = wmi.query::<WmiDiskPartition>() {
            for part in partitions {
                if let Some(disk_index) = part.disk_index {
                    let size_val = part.size
                        .unwrap_or_else(|| "0".to_string())
                        .parse::<f64>().unwrap_or(0.0);

                    partition_map.entry(disk_index).or_insert_with(Vec::new).push(
                        Partition {
                            name: part.name.unwrap_or_else(|| "Unknown".to_string()),
                            size: size_val,
                        }
                    );
                }
            }
        }

        if let Ok(disks) = wmi.query::<WmiDiskDrive>() {
            for disk in disks {
                let disk_index = disk.index.unwrap_or(0);
                let attached_partitions = partition_map.remove(&disk_index);

                let disk_size = disk.size
                    .unwrap_or_else(||"0".to_string())
                    .parse::<u64>()
                    .unwrap_or(0);

                disk_list.push(PhysicalDisk {
                    drive: disk.device_id.unwrap_or_else(|| "Unknown".to_string()),
                    index: disk_index,
                    model: disk.model.unwrap_or_else(|| "Unknown".to_string()).trim().to_string(),
                    serial: disk.serial_number.unwrap_or_else(|| "Unknown".to_string()).trim().to_string(),
                    firmware: disk.firmware_revision.unwrap_or_else(|| "Unknown".to_string()).trim().to_string(),
                    size: disk_size, 
                    media: disk.media_type.unwrap_or_else(|| "Unknown".to_string()),
                    interface: disk.interface_type.unwrap_or_else(|| "Unknown".to_string()),
                    status: disk.status.unwrap_or_else(|| "Unknown".to_string()),
                    partition: attached_partitions, 
                });
            }
            info.physical_disk_list = Some(disk_list);
        }

        let mut net_map: HashMap<String, NetworkHardware> = HashMap::new();
        if let Ok(adapters) = wmi.query::<WmiNetworkAdapter>() {
            for adapter in adapters {
                if adapter.physical_adapter.unwrap_or(false) {
                    if let Some(interface_name) = adapter.net_connection_id {
                        let speed_bps = adapter.speed
                            .unwrap_or_else(|| "0".to_string())
                            .parse::<u64>().unwrap_or(0);
                        let speed_mbps = (speed_bps / 1_000_000) as u32;

                        net_map.insert(
                            interface_name,
                            NetworkHardware {
                                card: adapter.name.unwrap_or_else(|| "Unknown".to_string()),
                                speed: speed_mbps,
                            },
                        );
                    }
                }
            }
        }

        if !net_map.is_empty() {
            info.network_hardware = Some(net_map);
        }

        return Ok(info);
    }
}

impl OsSpecificInterface for OsSpecificBackend {
    fn refresh(&mut self) {
        self.components.refresh(true);
    }

    fn get_product_serial(&self) -> &str {
        self.cached_info.as_ref()
            .and_then(|i| i.product_serial.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("Unknown")
    }

    fn get_architecture(&self) -> &str {
        self.cached_info.as_ref()
            .and_then(|i| i.architecture.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("Unknown")
    }

    fn get_producer(&self) -> &str {
        self.cached_info.as_ref()
            .and_then(|i| i.producer.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("Unknown")
    }

    fn get_system_model(&self) -> &str {
        self.cached_info.as_ref()
            .and_then(|i| i.system_model.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("Unknown")
    }

    fn get_machine_type(&self) -> &str {
        self.cached_info.as_ref()
            .and_then(|i| i.machine_type.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("Unknown")
    }

    fn get_bios_version(&self) -> &str {
        self.cached_info.as_ref()
            .and_then(|i| i.bios_version.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("Unknown")
    }

    fn get_bios_vendor(&self) -> &str {
        self.cached_info.as_ref()
            .and_then(|i| i.bios_vendor.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("Unknown")
    }

    fn get_is_secure_boot(&self) -> Option<bool> {
        self.cached_info.as_ref().and_then(|i| i.is_secure_boot)
    }

    fn get_os_name(&self) -> &str {
        self.cached_info.as_ref()
            .and_then(|i| i.os_name.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("Unknown")
    }

    fn get_motherboard(&self) -> &str {
        self.cached_info.as_ref()
            .and_then(|i| i.mobo_name.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("Unknown")
    }

    fn get_motherboard_serial(&self) -> &str {
        self.cached_info.as_ref()
            .and_then(|i| i.mobo_serial.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("Unknown")
    }

    fn get_cpu_socket(&self) -> Option<u32> {
        self.cached_info.as_ref().and_then(|i| i.cpu_socket)
    }

    fn get_ram_socket(&self) -> Option<u32> {
        self.cached_info.as_ref().and_then(|i| i.ram_socket)
    }

    fn get_gpu_socket(&self) -> Option<u32> {
        self.cached_info.as_ref().and_then(|i| i.gpu_socket)
    }

    fn get_ram_list(&self) -> Option<&Vec<Ram>> {
        self.cached_info.as_ref().and_then(|i| i.ram_list.as_ref())
    }

    fn get_physical_disk_list(&self) -> Option<&Vec<PhysicalDisk>> {
        self.cached_info.as_ref().and_then(|i| i.physical_disk_list.as_ref())
    }

    fn get_tempe_mobo(&self) -> Option<f32> {
        let mut tempes = Vec::new();

        for c in self.components.iter().filter(|c| {
            let name = c.label().to_lowercase();
            name.contains("motherboard") || name.contains("system") || name.contains("thermal zone")
        }) {
            match c.temperature() {
                Some(tempe) => tempes.push(tempe as f32),
                None => println!("Unable to read {}", c.label()),
            }
        }

        tempes.into_iter().max_by(|a, b| a.total_cmp(b))
    }

    fn get_tempe_cpu(&self) -> Option<f32> {
        let mut tempes = Vec::new();

        for c in self.components.iter().filter(|c| {
            let name = c.label().to_lowercase();
            name.contains("cpu") || name.contains("package") || name.contains("core")
        }) {
            match c.temperature() {
                Some(tempe) => tempes.push(tempe as f32),
                None => println!("Unable to read {}", c.label()),
            }
        }

        tempes.into_iter().max_by(|a, b| a.total_cmp(b))
    }

    fn get_battery_percentage(&self) -> Option<f32> {
        unsafe {
            let mut status: SYSTEM_POWER_STATUS = std::mem::zeroed();
            
            if GetSystemPowerStatus(&mut status) != 0 {
                // 255 is the Windows kernel code for "No Battery / Desktop PC"
                if status.BatteryLifePercent != 255 {
                    return Some(status.BatteryLifePercent as f32);
                }
            }
        }
        None
    }

    fn get_is_plugged_in(&self) -> Option<bool> {
        unsafe {
            let mut status: SYSTEM_POWER_STATUS = std::mem::zeroed();
            
            if GetSystemPowerStatus(&mut status) != 0 {
                if status.ACLineStatus == 0 {
                    return Some(false);
                } else if status.ACLineStatus == 1 {
                    return Some(true);
                }
            }
        }
        None
    }

    fn fill_network_hardware(&self, network_list: &mut Vec<Network<'_>>) {
        if let Some(hardware_map) = &self.cached_info.as_ref().and_then(|i| i.network_hardware.as_ref()) {
            for net in network_list.iter_mut() {
                if let Some(hw) = hardware_map.get(net.get_name().as_str()) {
                    net.hardware = hw.clone();
                }

                let name = &net.get_name();
                if name.contains("wlan") || name.contains("wi-fi") || name.contains("wireless") {
                    net.ssid = get_ssid_of(name);
                }
            }
        }
    }

    fn get_software_list(&self) -> Option<Vec<Software>> {
        let mut software_list = Vec::new();
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        // Both 64-bit apps and 32-bit apps (WOW6432Node)
        let registry_paths = [
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        ];

        for path in registry_paths {
            if let Ok(uninstall_key) = hklm.open_subkey_with_flags(path, KEY_READ) {
                
                for subkey_name in uninstall_key.enum_keys().filter_map(Result::ok) {
                    if let Ok(app_key) = uninstall_key.open_subkey_with_flags(&subkey_name, KEY_READ) {
                        if let Ok(name) = app_key.get_value::<String, _>("DisplayName") {
                            
                            let version = app_key
                                .get_value::<String, _>("DisplayVersion")
                                .unwrap_or_else(|_| "Unknown".to_string());
                                
                            let source = app_key
                                .get_value::<String, _>("Publisher")
                                .unwrap_or_else(|_| "Unknown".to_string());

                            // EstimatedSize is stored in Kilobytes as a u32.
                            let raw_size = app_key.get_value::<u32, _>("EstimatedSize").unwrap_or(0);
                            let size = (raw_size as u64).kb_to_mb();

                            let install_date = app_key
                                .get_value::<String, _>("InstallDate")
                                .ok()
                                .and_then(|d| parse_install_date(&d));

                            software_list.push(Software {
                                name,
                                version,
                                source,
                                size: size,
                                install_date,
                            });
                        }
                    }
                }
            }
        }

        Some(software_list)
    }

}

//: Helpers
fn parse_ram_type(mem_type: Option<u16>, smbios_type: Option<u32>) -> String {
    // Check modern SMBIOS type first
    if let Some(st) = smbios_type {
        match st {
            26 => return "DDR4".to_string(),
            34 => return "DDR5".to_string(),
            _ => {}
        }
    }
    // Fallback to legacy MemoryType
    match mem_type.unwrap_or(0) {
        20 => "DDR".to_string(),
        21 => "DDR2".to_string(),
        24 => "DDR3".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn parse_form_factor(code: Option<u16>) -> String {
    match code.unwrap_or(0) {
        8 => "DIMM".to_string(),
        12 => "SODIMM".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn get_ssid_of(interface_name: &str) -> String {
    let target_name = format!("name={}", interface_name);
    let output = Command::new("netsh")
        .args(&["wlan", "show", "interfaces", &target_name])
        .output();

    if let Ok(out) = output {
        let raw = String::from_utf8_lossy(&out.stdout);
        
        for line in raw.lines() {
            if line.trim_start().starts_with("SSID") {
                if let Some((_, ssid_val)) = line.split_once(':') {
                    let clean_ssid = ssid_val.trim();
                    if !clean_ssid.is_empty() {
                        return clean_ssid.to_string();
                    }
                }
            }
        }
    }
    
    "Unknown".to_string()
}

fn parse_install_date(date_str: &str) -> Option<SystemTime> {
    if date_str.len() != 8 {
        return None;
    }
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y%m%d").ok()?;
    let datetime = naive_date.and_hms_opt(0, 0, 0)?;
    Some(Utc.from_utc_datetime(&datetime).into())
}


