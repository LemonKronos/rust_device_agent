//! 
//! # Fetcher logic for Linux
//! 


use std::io::ErrorKind::{PermissionDenied, NotFound, InvalidInput};
use smbioslib::{SMBiosMemoryDevice,SMBiosPhysicalMemoryArray, SMBiosProcessorInformation, table_load_from_device};
use shared_libs::types::Ram;

const HARDWARE_INFO_PATH: &str = "/sys/class/dmi/id";

pub fn get(key: &str) -> Option<String> {
    match key {
        "product_serial" => {
            parse_file(HARDWARE_INFO_PATH, "product_serial")
        },
        "board_serial" => {
            parse_file(HARDWARE_INFO_PATH, "board_serial")
        },
        "cpu_socket" => {
            get_cpu_socket()
        },
        "ram_socket" => {
            get_ram_socket()
        },
        "ram_list" => {
            get_ram_list()
        }
        _ => None,
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
        Err(e) if e.kind() == NotFound && std::path::Path::new(path).exists() => None,
        Err(e) if e.kind() != PermissionDenied => {
            log::error!("Linux file parser error for {}: {}", file, e.kind());
            None
        },
        Err(_) => None,
    }
}

fn get_cpu_socket() -> Option<String> {
    match table_load_from_device() {
        Ok(data) => {
            Some(data.defined_struct_iter::<SMBiosProcessorInformation>().count().to_string())
        }
        Err(e) if e.kind() == PermissionDenied => {
            log::warn!("Required Admin to read cpu socket");
            None
        }
        Err(e) => {
            log::error!("Failded to read cpu socket: {}", e);
            None
        }
    }
}

fn get_ram_socket() -> Option<String> {
    match table_load_from_device() {
        Ok(data) => {
            let mut ram_socket = 0;
            for mem_array in data.defined_struct_iter::<SMBiosPhysicalMemoryArray>() {
                if let Some(count) = mem_array.number_of_memory_devices() {
                    ram_socket += count;
                }
            }
            Some(ram_socket.to_string())
        }
        Err(e) if e.kind() == PermissionDenied => {
            log::warn!("Required Admin to read ram socket");
            None
        }
        Err(e) => {
            log::error!("Failded to read ram socket: {}", e);
            None
        }
    }
}

fn get_ram_list() -> Option<String> {
    let data = match table_load_from_device() {
        Ok(d) => d,
        Err(e) if e.kind() == PermissionDenied => {
            log::warn!("Required Admin to read ram list");
                return None;
        }
        Err(e) => {
            log::error!("Failded to read ram list: {}", e);
            return None;
        }
    };

    let mut ram_list = Vec::new();

    for mem_device in data.defined_struct_iter::<SMBiosMemoryDevice>() {
        let raw_size = format!("{:?}", mem_device.size());

        if raw_size.contains("0") || raw_size.contains("Unknown") || raw_size.contains("None") {
            continue;
        }

        let manufacturer = mem_device.manufacturer().to_string();
        let part_number = mem_device.part_number().to_string();
        let name = format!("{} {}", manufacturer, part_number).trim().to_string();

        let serial = mem_device.serial_number().to_string();

        let type_ = match mem_device.memory_type() {
            Some(mem_type_data) => format!("{:?}", mem_type_data.value).to_uppercase(),
            None => "Unknown".to_string(),
        };

        let bank = mem_device.bank_locator().to_string();

        let form_factor = match mem_device.form_factor() {
            Some(ff) => format!("{:?}", ff.value).to_uppercase(),
            None => "Unknown".to_string(),
        };

        let size =  if raw_size.contains("Gigabytes") {
            raw_size.replace("Some(Gigabytes(", "").replace("))", "")
        } else if raw_size.contains("Megabytes") {
            raw_size.replace("Some(Megabytes(", "").replace("))", "")
        } else if raw_size.contains("Some(") {
            raw_size.replace("Some(", "").replace(")", "")
        } else {
            raw_size
        }.trim().parse::<u64>().unwrap_or_default();

        let speed_debug = format!("{:?}", mem_device.configured_memory_speed().or_else(|| mem_device.speed()));
        let speed = speed_debug
            .replace("Some(MTs(", "").replace("))", "")
            .replace("MTs(", "").replace(")", "")
            .replace("Some(Unknown)", "Unknown");

        ram_list.push(Ram::new(name, serial, type_, speed, size, bank, form_factor));
    }
    
    let ram_json = match serde_json::to_string(&ram_list) {
        Ok(json) => Some(json),
        Err(e) => {
            log::error!("Error in Ram to json string conversion: {}", e);
            None
        }
    };

    ram_json
}