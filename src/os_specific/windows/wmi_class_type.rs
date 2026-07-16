
use serde::Deserialize;

//: WMI struct mapping
#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_ComputerSystemProduct", rename_all = "PascalCase")]
pub struct WmiCsProduct {
    pub identifying_number: Option<String>,
    pub vendor: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_ComputerSystem", rename_all = "PascalCase")]
pub struct WmiComputerSystem {
    pub system_type: Option<String>,
    pub number_of_processors: Option<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_SystemEnclosure", rename_all = "PascalCase")]
pub struct WmiEnclosure {
    pub chassis_types: Option<Vec<u16>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_BIOS", rename_all = "PascalCase")]
pub struct WmiBios {
    pub manufacturer: Option<String>,
    pub smbiosbiosversion: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_OperatingSystem", rename_all = "PascalCase")]
pub struct WmiOs {
    pub caption: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_BaseBoard", rename_all = "PascalCase")]
pub struct WmiBaseBoard {
    pub product: Option<String>,
    pub serial_number: Option<String>,
}

// #[derive(Deserialize, Debug)]
// #[serde(rename = "Win32_Processor", rename_all = "PascalCase")]
// pub struct WmiProcessor {
//     pub socket_designation: Option<String>,
// }

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_SystemSlot", rename_all = "PascalCase")]
pub struct WmiSystemSlot {
    pub slot_designation: Option<String>,
    pub max_data_width: Option<u16>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_PhysicalMemoryArray", rename_all = "PascalCase")]
pub struct WmiMemoryArray {
    pub memory_devices: Option<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_PhysicalMemory", rename_all = "PascalCase")]
pub struct WmiMemory {
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub speed: Option<u32>,
    pub capacity: Option<String>,
    pub bank_label: Option<String>,
    pub memory_type: Option<u16>,
    pub smbiosmemory_type: Option<u32>,
    pub form_factor: Option<u16>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_DiskDrive", rename_all = "PascalCase")]
pub struct WmiDiskDrive {
    pub device_id: Option<String>,
    pub index: Option<u32>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub firmware_revision: Option<String>,
    pub size: Option<String>,
    pub media_type: Option<String>,
    pub interface_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_DiskPartition", rename_all = "PascalCase")]
pub struct WmiDiskPartition {
    pub disk_index: Option<u32>,
    pub name: Option<String>,
    pub size: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_NetworkAdapter", rename_all = "PascalCase")]
pub struct WmiNetworkAdapter {
    pub net_connection_id: Option<String>,
    pub name: Option<String>,
    pub speed: Option<String>,
    pub physical_adapter: Option<bool>,
}
