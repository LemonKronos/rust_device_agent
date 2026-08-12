//!
//! # Real logic for reading these info on Linux
//! 
//! Some info can be read with `sysinfo::Components`, but still specific to Linux.
//! Some info can be read with `smbioslib`.
//! The rest need to be parsed dirrectly from the Linux file system.
//! 
//! **This was dev and tested only on Ubuntu 24.04.4 LTS x86_64**

use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::result::Result::Ok;
use std::io::{ErrorKind::{PermissionDenied, NotFound, InvalidInput},BufRead, BufReader};
use std::collections::HashMap;

use sysinfo::Components;

use super::interface::OsSpecificInterface;
use super::parse_chassis_type;
use crate::ipc;
use shared_libs::types::*;

const HARDWARE_INFO_PATH: &str = "/sys/class/dmi/id";
const HARD_DISK_PATH: &str = "/sys/block";
const BATTERY_0_PATH: &str = "/sys/class/power_supply/BAT0";
const BATTERY_1_PATH: &str = "/sys/class/power_supply/BAT1";
const SECURE_BOOT_PATH: &str = "/sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c";
const NETWORK_PATH: &str = "/sys/class/net";
const PCI_DEVICE_PATH: &str = "/sys/bus/pci/devices";
const DPKG_PATH: &str = "/var/lib/dpkg";

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
    cpu_socket: Option<u64>,
    ram_socket: Option<u64>,
    gpu_socket: Option<u64>,

    ram_list: Option<Vec<Ram>>,
    physical_disk_list: Option<Vec<PhysicalDisk>>,
    network_hardware: Option<HashMap<String,NetworkHardware>>,
}

/// Real Linux backend
#[derive(Debug)]
pub struct OsSpecificBackend {
    components: Components,
    cached_info: CachedInfo,
    battery_path: Option<String>,
}

impl OsSpecificBackend {
    pub fn new() -> Self {
        let cached_info = Self::set_cached_info();

        let battery_path = if fs::metadata(BATTERY_0_PATH).is_ok() {
            Some(BATTERY_0_PATH.to_string())
        } else if fs::metadata(BATTERY_1_PATH).is_ok() {
            Some(BATTERY_1_PATH.to_string())
        } else {
            None
        };

        Self {
            components: Components::new_with_refreshed_list(),
            cached_info: cached_info,
            battery_path: battery_path,
        }
    }

    fn parse_file(path: &str, file: &str) -> Option<String> {
        match std::fs::read_to_string(format!("{}/{}", path, file)) {
            Ok(s) => Some(s.trim().to_string()),
            Err(e) if e.kind() == PermissionDenied => {
                log::warn!("Required Admin to read {}", file);
                None
            },
            Err(e) if e.kind() == InvalidInput => None,
            Err(e) if e.kind() == NotFound && Path::new(path).exists() => None,
            Err(e) if e.kind() != PermissionDenied => {
                log::error!("Linux file parser error for {}: {}", file, e.kind());
                None
            },
            Err(_) => None,
        }
    }

    fn set_cached_info() -> CachedInfo {
        let mut info = CachedInfo::default();

        info.product_serial = ipc::ask_launcher("product_serial"); //: Admin priviledge

        info.architecture = Some(std::env::consts::ARCH.to_owned());

        info.producer = Self::parse_file(HARDWARE_INFO_PATH, "sys_vendor");

        info.system_model = Self::parse_file(HARDWARE_INFO_PATH, "product_name");

        info.machine_type = Self::parse_file(HARDWARE_INFO_PATH, "chassis_type")
            .map(|code| parse_chassis_type(&code));

        info.bios_version =  Self::parse_file(HARDWARE_INFO_PATH, "bios_version");

        info.bios_vendor = Self::parse_file(HARDWARE_INFO_PATH, "bios_vendor");

        info.is_secure_boot = match fs::read(SECURE_BOOT_PATH) {
            Ok(bytes) if bytes.len() >= 5 => Some(bytes[4] == 1),
            _ => None,
        };

        info.os_name = Some(std::env::consts::OS.to_string());

        info.mobo_name = Self::parse_file(HARDWARE_INFO_PATH, "board_name");

        info.mobo_serial = ipc::ask_launcher("board_serial");  //: Admin priviledge

        info.cpu_socket = ipc::ask_launcher("cpu_socket").and_then(|s| s.trim().parse::<u64>().ok());  //: Admin priviledge

        info.ram_socket = ipc::ask_launcher("ram_socket").and_then(|s| s.trim().parse::<u64>().ok());  //: Admin priviledge

        info.gpu_socket = Self::cache_gpu_socket();

        info.ram_list = ipc::ask_launcher("ram_list").and_then(|list| serde_json::from_str(&list).ok());  //: Admin priviledge

        info.physical_disk_list = Self::cache_physical_disk_list();

        info.network_hardware = Self::cache_network_hardware();

        info
    }

    fn cache_gpu_socket() -> Option<u64> {
        let mut gpu_socket = 0;
        match  fs::read_dir(PCI_DEVICE_PATH) {
            Ok(entries) =>  {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let class_path = path.join("class");

                    if let Ok(class_str) = fs::read_to_string(&class_path) {
                        if class_str.trim().starts_with("0x03") {
                            gpu_socket += 1;
                        }
                    }
                }
                Some(gpu_socket)
            }
            Err(e) if e.kind() == PermissionDenied => {
                log::warn!("Required Admin to read gpu socket");
                None
            }
            Err(e) => {
                log::error!("Failded to read ram socket: {}", e);
                None
            }
        }
    }

    fn cache_physical_disk_list() -> Option<Vec<PhysicalDisk>> {
        let mut list = Vec::new();
        let block_dir = Path::new(HARD_DISK_PATH);
        match fs::read_dir(block_dir) {
            Ok(entries) => {
                let mut index: u64 = 0;

                for entry in entries.flatten() {
                    let drive = entry.file_name().to_string_lossy().into_owned();

                    if drive.starts_with("loop") || drive.starts_with("ram") || drive.starts_with("sr") {
                        continue;
                    }

                    let device_path = entry.path().join("device");

                    let model = match fs::read_to_string(device_path.join("model")) {
                        Ok(s) => s.trim().to_string(),
                        Err(_) => "Unknown".to_string(),
                    };

                    let serial = match fs::read_to_string(device_path.join("serial")) {
                        Ok(s) => s.trim().to_string(),
                        Err(_) => "Unknown".to_string(),
                    };

                    let firmware = fs::read_to_string(device_path.join("firmware_rev"))
                        .or_else(|_| fs::read_to_string(device_path.join("rev")))
                        .map(|s| s.trim().to_string())
                        .unwrap_or_else(|_| "Unknown".to_string());

                    let sector: u64 = fs::read_to_string(entry.path().join("size"))
                        .unwrap_or_else(|_| "0".to_string())
                        .trim().parse().unwrap_or(0);
                    let size = ((sector * 512) as f64 / 1_000_000_000.0) as u64;

                    let media = match fs::read_to_string(entry.path().join("queue/rotational")) {
                        Ok(val) if val.trim() == "0" => "SSD".to_string(),
                        Ok(val) if val.trim() == "1" => "HDD".to_string(),
                        _ => "Unknown".to_string(),
                    };

                    let interface = if drive.starts_with("nvme") {
                        "NVMe".to_string()
                    } else if drive.starts_with("sd") {
                        if let Ok(link) = fs::read_link(&device_path) {
                            if link.to_string_lossy().contains("usb") {
                                "USB".to_string()
                            } else {
                                "SATA/SCSI".to_string()
                            }
                        } else {
                            "SATA".to_string()
                        }
                    } else if drive.starts_with("mmcblk") {
                        "eMMC/SD".to_string()
                    } else {
                        "Unknown".to_string()
                    };

                    let status = match fs::read_to_string(device_path.join("state")) {
                        Ok(state) => state.trim().to_string(),
                        _ => "Unknown".to_string(),
                    };

                    let mut part_list = Vec::new();
                    let mut part_index = 0;
                    if let Ok(sub_entries) = fs::read_dir(entry.path()) {
                        for sub_entry in sub_entries.flatten() {
                            let sub_name = sub_entry.file_name().to_string_lossy().into_owned();
                            
                            if sub_name.starts_with(&drive) {
                                let p_size_path = sub_entry.path().join("size");
                                let p_sector: u64 = fs::read_to_string(p_size_path)
                                    .unwrap_or_else(|_| "0".to_string())
                                    .trim()
                                    .parse()
                                    .unwrap_or(0);
                                
                                let p_size_gb = (p_sector * 512) as f64 / 1_000_000_000.0;
                                let rounded_size = (p_size_gb * 100.0).round() / 100.0;

                                part_list.push(Partition {
                                    name: part_index.to_string(), // ! Should change this to real partition name
                                    size: rounded_size,
                                });
                                part_index += 1;
                            }
                        }
                    }
                    list.push(PhysicalDisk { drive, index, model, serial, firmware, size, media, interface, status, partition: Some(part_list) });

                    index += 1;
                }
                Some(list)
            }
            Err(e) if e.kind() == PermissionDenied => {
                log::warn!("Required Admin to read disk hardware");
                None
            }
            Err(e) => {
                log::error!("Failded to read disk hardware: {}", e);
                None
            }
        }
    }

    fn cache_network_hardware() -> Option<HashMap<String,NetworkHardware>> {
        let mut net_cache = HashMap::new();
    
        let entries = fs::read_dir(NETWORK_PATH).ok()?;

        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let path = entry.path();
            let path_str = path.to_str().unwrap_or("");

            let speed = Self::parse_file(path_str, "speed")
                .and_then(|s| s.trim().parse::<u64>().ok());

            let mut card = None;
            if let Ok(target) = fs::read_link(path.join("device")) {
                if let Some(pci) = target.file_name().and_then(|n| n.to_str()) {
                    if let Ok(output) = Command::new("lspci").arg("-s").arg(pci).output() {
                        let out_str = String::from_utf8_lossy(&output.stdout);
                        if let Some(desc) = out_str.split(": ").nth(1) {
                            card = Some(desc.trim().to_string());
                        }
                    }
                }
            }

            
            if let Some(card_name) = card {
                    net_cache.insert(name, NetworkHardware {
                    card: card_name,
                    speed 
                });
            }
        }

        Some(net_cache)
    }
}

impl OsSpecificInterface for OsSpecificBackend {
    fn refresh(&mut self) {
        self.components.refresh(true);

        self.battery_path = if fs::metadata(BATTERY_0_PATH).is_ok() {
            Some(BATTERY_0_PATH.to_string())
        } else if fs::metadata(BATTERY_1_PATH).is_ok() {
            Some(BATTERY_1_PATH.to_string())
        } else {
            None
        };
        
        // todo!("What to refresh?")
    }

    fn get_product_serial(&self) -> Option<&str> {
        self.cached_info.product_serial.as_deref()
    }

    fn get_architecture(&self) -> &str {
        match &self.cached_info.architecture {
            Some(info) => info.as_str(),
            None => "Unknown",
        }
    }

    fn get_producer(&self) -> &str {
        match &self.cached_info.producer {
            Some(info) => info.as_str(),
            None => "Unknown",
        }
    }

    fn get_system_model(&self) -> &str {
        match &self.cached_info.system_model {
            Some(info) => info.as_str(),
            None => "Unknown",
        }
    }

    fn get_machine_type(&self) -> &str {
        match &self.cached_info.machine_type {
            Some(info) => info.as_str(),
            None => "Unknown",
        }
    }

    fn get_bios_version(&self) -> &str {
        match &self.cached_info.bios_version {
            Some(info) => info.as_str(),
            None => "Unknown",
        }
    }

    fn get_bios_vendor(&self) -> &str {
        match &self.cached_info.bios_vendor {
            Some(info) => info.as_str(),
            None => "Unknown",
        }
    }

    fn get_is_secure_boot(&self) -> Option<bool> {
        self.cached_info.is_secure_boot
    }

    fn get_os_name(&self) -> &str {
        match &self.cached_info.os_name {
            Some(info) => info.as_str(),
            None => "Unknown",
        }
    }

    fn get_motherboard(&self) -> &str {
        match &self.cached_info.mobo_name {
            Some(info) => info.as_str(),
            None => "Unknown",
        }
    }

    fn get_motherboard_serial(&self) -> Option<&str> {
        self.cached_info.mobo_serial.as_deref()
    }

    fn get_cpu_socket(&self) -> Option<u64> {
        self.cached_info.cpu_socket
    }

    fn get_gpu_socket(&self) -> Option<u64> {
        self.cached_info.gpu_socket
    }
    
    fn get_ram_socket(&self) -> Option<u64> {
        self.cached_info.ram_socket
    }

    fn get_ram_list(&self) -> Option<&Vec<Ram>> {
        self.cached_info.ram_list.as_ref()
    }
    
    fn get_physical_disk_list(&self) -> Option<&Vec<PhysicalDisk>> {
        self.cached_info.physical_disk_list.as_ref()
    }

    fn get_tempe_mobo(&self) -> Option<f64> {
        self.components
            .iter()
            .filter(|c| c.label().to_lowercase().contains("acpitz"))
            .map(|c| c.temperature().unwrap_or(0.0) as f64)
            .max_by(|a, b| a.total_cmp(b))
    }

    fn get_tempe_cpu(&self) -> Option<f64> {
        self.components
            .iter()
            .filter(|c| c.label().to_lowercase().contains("k10temp"))
            .map(|c| c.temperature().unwrap_or(0.0) as f64)
            .max_by(|a, b| a.total_cmp(b))
    }

    fn get_battery_percentage(&self) -> Option<u64> {
        self.battery_path.as_ref()
            .and_then(|p| Self::parse_file(p, "capacity"))
            .and_then(|i| i.trim().parse().ok())
    }

    fn get_is_plugged_in(&self) -> Option<bool> {
        self.battery_path.as_ref()
            .and_then(|p| Self::parse_file(p, "status"))
            .and_then(|i| Some(i.trim().to_lowercase() != "discharging"))
    }

    fn fill_network_hardware(&self, network_list: &mut Vec<shared_libs::types::Network<'_>>) {
        for interface in network_list {
            if let Some(hw_map) = &self.cached_info.network_hardware {
                interface.hardware = hw_map.get(interface.get_name().as_str()).cloned();
            }

            // Read SSID with iwgetid command decisively
            if interface.get_name() == "wlp5s0" {
                if let Ok(output) = Command::new("iwgetid").arg("-r").arg(interface.get_name()).output() {
                    let out_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !out_str.is_empty() {
                        interface.ssid = Some(out_str);
                    }
                }
            }
        }
    }

    fn get_software_list(&self) -> Option<Vec<Software>> {
        let mut list = Vec::new();

        let status_path = Path::new(DPKG_PATH).join("status");
        let file = match File::open(status_path) {
            Ok(f) => f,
            Err(e) if e.kind() == PermissionDenied => {
                log::warn!("Required Admin to read software list");
                return None;
            }
            Err(e) => {
                log::error!("Failded to read software list: {}", e);
                return None;
            }
        };

        let reader = BufReader::new(file);
        let mut current = Software::new();

        for line in reader.lines().flatten() {
            if line.trim().is_empty() {
                // End of a pkg block
                if !current.name.is_empty() {
                    let info_path = format!("{}/info/{}.list", DPKG_PATH, current.name);
                    current.install_date = fs::metadata(&info_path).and_then(|meta| meta.modified()).ok();

                    list.push(current);

                    #[cfg(debug_assertions)]
                    //: Early exit for in dev mode, only show 5 software
                    if list.len() >= 5 {
                        break;
                    }

                    current = Software::new();
                }
                continue;
            }

            if let Some((key, value)) = line.split_once(": ") {
                match key {
                    "Package" => current.name = value.to_string(),
                    "Version" => current.version = Some(value.to_string()),
                    "Maintainer" => current.source = Some(value.split('<').next().unwrap_or(value).trim().to_string()),
                    "Installed-Size" => current.size = value.parse::<f64>().unwrap_or_default(),
                    _ => {},
                }
            }
        }
        Some(list)
    }

}

