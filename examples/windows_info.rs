use serde::Deserialize;
use wmi::WMIConnection;

// ==========================================
// 1. WMI MAPPING STRUCTS
// ==========================================

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_ComputerSystemProduct", rename_all = "PascalCase")]
struct WmiCsProduct {
    identifying_number: Option<String>,
    vendor: Option<String>,
    name: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_ComputerSystem", rename_all = "PascalCase")]
struct WmiComputerSystem {
    system_type: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_BIOS", rename_all = "PascalCase")]
struct WmiBios {
    manufacturer: Option<String>,
    smbiosbiosversion: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_OperatingSystem", rename_all = "PascalCase")]
struct WmiOs {
    caption: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_BaseBoard", rename_all = "PascalCase")]
struct WmiBaseBoard {
    product: Option<String>,
    serial_number: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_Processor", rename_all = "PascalCase")]
struct WmiProcessor {
    socket_designation: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_SystemSlot", rename_all = "PascalCase")]
struct WmiSystemSlot {
    slot_designation: Option<String>,
    max_data_width: Option<u16>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_PhysicalMemoryArray", rename_all = "PascalCase")]
struct WmiMemoryArray {
    memory_devices: Option<u32>, // Max number of RAM slots
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_PhysicalMemory", rename_all = "PascalCase")]
struct WmiMemory {
    manufacturer: Option<String>,
    serial_number: Option<String>,
    speed: Option<u32>,
    capacity: Option<String>,
    bank_label: Option<String>,
    memory_type: Option<u16>,
    smbiosmemory_type: Option<u32>,
    form_factor: Option<u16>,
}

use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_DiskDrive", rename_all = "PascalCase")]
struct WmiDiskDrive {
    device_id: Option<String>,
    index: Option<u32>,
    model: Option<String>,
    serial_number: Option<String>,
    firmware_revision: Option<String>,
    size: Option<String>, // WMI returns u64 as String
    media_type: Option<String>,
    interface_type: Option<String>,
    status: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_DiskPartition", rename_all = "PascalCase")]
struct WmiDiskPartition {
    disk_index: Option<u32>,
    name: Option<String>,
    size: Option<String>, // WMI returns u64 as String
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_NetworkAdapter", rename_all = "PascalCase")]
struct WmiNetworkAdapter {
    net_connection_id: Option<String>, // The interface name (e.g., "Ethernet", "Wi-Fi")
    name: Option<String>,            // The hardware card name
    speed: Option<String>,           // Returned as a string of bits-per-second
    physical_adapter: Option<bool>,   // The magic filter boolean
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

// ==========================================
// 2. YOUR TARGET STRUCTS
// ==========================================
#[derive(Debug)]
pub struct Ram {
    pub name: String,
    pub serial: String,
    pub type_: String,
    pub speed: String,
    pub size: u64,
    pub bank: String,
    pub form_factor: String,
}

#[derive(Debug)]
pub struct PhysicalDisk {
    pub drive: String,
    pub index: u32,
    pub model: String,
    pub serial: String,
    pub firmware: String,
    pub size: u64, // CRITICAL FIX: Changed from u32 to prevent overflow
    pub media: String,
    pub interface: String,
    pub status: String,
    pub partition: Option<Vec<Partition>>,
}

#[derive(Debug)]
pub struct Partition {
    pub name: String,
    pub size: f64,
}

#[derive(Debug, Default, Clone)]
pub struct NetworkHardware {
    pub card: String,
    pub speed: u32,
}

#[derive(Debug, Default)]
pub struct DeviceAgentInfo {
    pub product_serial: Option<String>,
    pub architecture: Option<String>,
    pub producer: Option<String>,
    pub system_model: Option<String>,
    pub machine_type: Option<String>,
    pub bios_vendor: Option<String>,
    pub bios_version: Option<String>,
    pub is_secure_boot: Option<bool>,
    pub os_name: Option<String>,
    pub mobo_name: Option<String>,
    pub mobo_serial: Option<String>,
    pub cpu_socket: Option<String>, // Changed to String (e.g., "LGA1700")
    pub ram_socket: Option<u32>,
    pub gpu_socket: Option<u32>, // WMI struggles here, leaving None
    pub ram_list: Vec<Ram>,
    pub physical_disk_list: Vec<PhysicalDisk>,
    pub network_hardware: Option<Vec<NetworkHardware>>,
}

// ==========================================
// 3. THE HARDWARE POLLER
// ==========================================

pub fn gather_windows_telemetry() -> Result<DeviceAgentInfo, Box<dyn std::error::Error>> {
    let mut info = DeviceAgentInfo::default();

    // 1. Initialize COM and WMI Connection ONCE
    let wmi_con = WMIConnection::new()?;

    // 2. Batch Queries (Fire and map)
    
    if let Ok(cs_prod) = wmi_con.query::<WmiCsProduct>() {
        if let Some(data) = cs_prod.first() {
            info.product_serial = data.identifying_number.clone();
            info.producer = data.vendor.clone();
            info.system_model = data.name.clone();
        }
    }

    if let Ok(sys) = wmi_con.query::<WmiComputerSystem>() {
        if let Some(data) = sys.first() {
            info.architecture = data.system_type.clone();
        }
    }

    if let Ok(bios) = wmi_con.query::<WmiBios>() {
        if let Some(data) = bios.first() {
            info.bios_vendor = data.manufacturer.clone();
            info.bios_version = data.smbiosbiosversion.clone();
        }
    }

    if let Ok(os) = wmi_con.query::<WmiOs>() {
        if let Some(data) = os.first() {
            info.os_name = data.caption.clone();
        }
    }

    if let Ok(board) = wmi_con.query::<WmiBaseBoard>() {
        if let Some(data) = board.first() {
            info.mobo_name = data.product.clone();
            info.mobo_serial = data.serial_number.clone();
        }
    }

    // ! This is CPU socket design, not number
    if let Ok(cpu) = wmi_con.query::<WmiProcessor>() {
        if let Some(data) = cpu.first() {
            info.cpu_socket = data.socket_designation.clone();
        }
    }

    if let Ok(ram_array) = wmi_con.query::<WmiMemoryArray>() {
        if let Some(data) = ram_array.first() {
            info.ram_socket = data.memory_devices;
        }
    }

    // Extract GPU Sockets (Counting PCIe x16 motherboard slots)
    if let Ok(slots) = wmi_con.query::<WmiSystemSlot>() {
        let mut gpu_slots = 0;
        
        for slot in slots {
            // GPUs overwhelmingly use PCIe x16 slots (MaxDataWidth == 16)
            if let Some(width) = slot.max_data_width {
                if width == 16 {
                    gpu_slots += 1;
                }
            } else if let Some(designation) = &slot.slot_designation {
                // Fallback string check just in case the BIOS firmware is lazy
                if designation.to_uppercase().contains("X16") {
                    gpu_slots += 1;
                }
            }
        }
        
        // If we found slots, wrap it in Some(), otherwise leave it as None
        if gpu_slots > 0 {
            info.gpu_socket = Some(gpu_slots);
        }
    }

    // 3. Handle Lists (RAM and Disks)
    if let Ok(ram_modules) = wmi_con.query::<WmiMemory>() {
        for stick in ram_modules {
            let parsed_size = stick
                .capacity
                .unwrap_or_else(|| "0".to_string())
                .parse::<u64>()
                .unwrap_or(0);

            let parsed_speed = stick
                .speed
                .map(|s| format!("{} MHz", s))
                .unwrap_or_else(|| "Unknown".to_string());

            info.ram_list.push(Ram {
                name: stick.manufacturer.unwrap_or_else(|| "Unknown".to_string()).trim().to_string(),
                serial: stick.serial_number.unwrap_or_else(|| "Unknown".to_string()).trim().to_string(),
                type_: parse_ram_type(stick.memory_type, stick.smbiosmemory_type),
                speed: parsed_speed, // Fits your struct's String requirement
                size: parsed_size,
                bank: stick.bank_label.unwrap_or_else(|| "Unknown".to_string()),
                form_factor: parse_form_factor(stick.form_factor),
            });
        }
    }

    // 1. Pre-fetch partitions and group them by DiskIndex
    let mut partition_map: HashMap<u32, Vec<Partition>> = HashMap::new();
    
    if let Ok(partitions) = wmi_con.query::<WmiDiskPartition>() {
        for part in partitions {
            if let Some(disk_index) = part.disk_index {
                let size_val = part
                    .size
                    .unwrap_or_else(|| "0".to_string())
                    .parse::<f64>()
                    .unwrap_or(0.0);
                
                partition_map.entry(disk_index).or_insert_with(Vec::new).push(Partition {
                    name: part.name.unwrap_or_else(|| "Unknown".to_string()),
                    size: size_val, // Matches your f64 requirement
                });
            }
        }
    }

    // 2. Fetch Physical Disks and attach their partitions
    if let Ok(disks) = wmi_con.query::<WmiDiskDrive>() {
        for disk in disks {
            let disk_index = disk.index.unwrap_or(0);
            
            // Extract the vector of partitions for this specific disk.
            // .remove() returns an Option<Vec<Partition>>, which perfectly matches your struct!
            let attached_partitions = partition_map.remove(&disk_index);

            let disk_size = disk
                .size
                .unwrap_or_else(|| "0".to_string())
                .parse::<u64>()
                .unwrap_or(0);

            info.physical_disk_list.push(PhysicalDisk {
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
    }

    // Initialize the HashMap
    let mut network_map: HashMap<String, NetworkHardware> = HashMap::new();

    if let Ok(adapters) = wmi_con.query::<WmiNetworkAdapter>() {
        for adapter in adapters {
            // 1. Filter out loopbacks, virtual adapters, and WAN miniports
            if adapter.physical_adapter.unwrap_or(false) {
                
                // 2. We only care about adapters that have an active Interface Name in Windows
                if let Some(interface_name) = adapter.net_connection_id {
                    
                    // 3. Convert raw bits per second to Mbps for your u32 field
                    let speed_bps = adapter
                        .speed
                        .unwrap_or_else(|| "0".to_string())
                        .parse::<u64>()
                        .unwrap_or(0);
                    let speed_mbps = (speed_bps / 1_000_000) as u32;

                    // 4. Insert into the HashMap
                    network_map.insert(
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

    // If we found any physical adapters, wrap the map in Some()
        // if !network_map.is_empty() {
        //     info.network_hardware = Some(network_map);
        // }

    // 4. The Secure Boot Workaround (Registry)
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::HKEY_LOCAL_MACHINE;
        use winreg::RegKey;
        
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(subkey) = hklm.open_subkey("System\\CurrentControlSet\\Control\\SecureBoot\\State") {
            if let Ok(state) = subkey.get_value::<u32, _>("UEFISecureBootEnabled") {
                info.is_secure_boot = Some(state == 1);
            }
        }
    }

    // Note: GPU socket is left as None. Motherboards rarely expose exact PCIe lane mapping to WMI.

    Ok(info)
}

fn main() {
    match gather_windows_telemetry() {
        Ok(data) => println!("Hardware Profile Gathered: {:#?}", data),
        Err(e) => eprintln!("Agent Error: {}", e),
    }
}