//!
//! # Keep all type for json serialize formating
//! 
//! This is where we use `#[serde(rename = "_")]` to format each field
//! 

use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::scheduler::AgentValue;
use shared_libs::types;

/// Main body
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct InfoPayload {
    pub general: Option<GeneralPayload>,

    pub machine: Option<MachinePayload>,

    #[serde(rename = "MOTHER_BOARD")]
    pub motherboard: Option<MotherboardPayload>,

    pub bios: Option<BiosPayload>,

    pub os: Option<OsPayload>,

    pub cpu: Option<CpuPayload>,

    pub ram: Option<RamPayload>,

    pub swap: Option<SwapPayload>,

    #[serde(rename = "GPU_Lst")]
    pub gpu: Option<Vec<GpuPayload>>,

    pub disk: Option<DiskPayload>,

    #[serde(rename = "NETWORK_Lst")]
    pub network: Option<Vec<NetworkPayload>>,

    pub process_count: Option<AgentValue>,

    pub top_process: Option<Vec<ProcessPayload>>,

    // pub all_process: Option<Vec<ProcessPayload>>,
    pub battery: Option<BatteryPayload>,

    pub software: Option<Vec<types::Software>>, // Send all so this don't have a delicated type
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GeneralPayload {
    #[serde(rename = "FULL_COMPUTER_NAME")]
    pub host: Option<AgentValue>,

    #[serde(rename = "BOOT_TIME_EPOCH")]
    pub boot_time: Option<AgentValue>,

    pub run_time: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MachinePayload {
    #[serde(rename = "SERIAL_NUMBER")]
    pub serial: Option<AgentValue>,

    #[serde(rename = "SYSTEM_TYPE")]
    pub architecture: Option<AgentValue>,

    pub producer: Option<AgentValue>,

    #[serde(rename = "SYSTEM_MODEL")]
    pub model: Option<AgentValue>,

    #[serde(rename = "COMPUTER_TYPE")]
    pub r#type: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MotherboardPayload {
    pub name: Option<AgentValue>,

    #[serde(rename = "SERIAL_NUMBER")]
    pub serial: Option<AgentValue>,

    #[serde(rename = "TEMPERATURE")]
    pub tempe: Option<AgentValue>,

    pub cpu_slot: Option<AgentValue>,
    pub ram_slot: Option<AgentValue>,
    pub gpu_slot: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BiosPayload {
    pub version: Option<AgentValue>,
    pub vendor: Option<AgentValue>,
    pub secure_boot: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OsPayload {
    pub distro: Option<AgentValue>,
    pub name: Option<AgentValue>,
    pub version: Option<AgentValue>,
    pub kernel: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CpuPayload {
    #[serde(rename = "CPU_NAME")]
    pub name: Option<AgentValue>,

    #[serde(rename = "CORES")]
    pub core: Option<AgentValue>,

    #[serde(rename = "CPU_USAGE_RATE")]
    pub usage: Option<AgentValue>,

    #[serde(rename = "SPEED")]
    pub frequency: Option<AgentValue>,

    pub temperature: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RamPayload {
    pub total: Option<AgentValue>,
    pub usage: Option<AgentValue>,

    #[serde(rename = "DETAILS")]
    pub physical: Option<Vec<RamPhysicalPayload>>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RamPhysicalPayload {
    #[serde(rename = "MANUFACTURER")]
    pub name: Option<AgentValue>,
    
    #[serde(rename = "SERIAL_NUMBER")]
    pub serial: Option<AgentValue>,

    #[serde(rename = "RAM_TYPE")]
    pub r#type: Option<AgentValue>,

    #[serde(rename = "SPEED")]
    pub config_speed: Option<AgentValue>,

    #[serde(rename = "CAPACITY")]
    pub size: Option<AgentValue>,

    #[serde(rename = "BANK_LABEL")]
    pub bank: Option<AgentValue>,

    pub form_factor: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SwapPayload {
    pub total: Option<AgentValue>,
    pub usage: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GpuPayload {
    #[serde(rename = "GPU_NAME")]
    pub name: Option<AgentValue>,

    #[serde(rename = "DRIVER_VERSION")]
    pub driver: Option<AgentValue>,

    pub utilization: Option<AgentValue>,
    pub temperature: Option<AgentValue>,
    pub frequency: Option<AgentValue>,
    pub vram_total: Option<AgentValue>,
    pub vram_usage: Option<AgentValue>,

    #[serde(rename = "MAX_CLOCK_SPEED")]
    pub max_clock: Option<AgentValue>,

    pub serial: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct DiskPayload {
    #[serde(rename = "LOGICAL_DISK_Lst")]
    pub logical: Option<Vec<LogicalDiskPayload>>,

    #[serde(rename = "PHYSICAL_DISK_Lst")]
    pub physical: Option<Vec<PhysicalDiskPayload>>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LogicalDiskPayload {
    #[serde(rename = "LOGICAL_DISK_NAME")]
    pub name: Option<AgentValue>,

    #[serde(rename = "FILE_SYSTEM_TYPE")]
    pub file_system: Option<AgentValue>,

    pub mount_point: Option<AgentValue>,
    pub removable: Option<AgentValue>,
    pub total: Option<AgentValue>,

    #[serde(rename = "USAGE")]
    pub used: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PhysicalDiskPayload {
    #[serde(rename = "DISK_NAME")]
    pub drive: Option<AgentValue>,

    #[serde(rename = "PHYSICAL_DISK_INDEX")]
    pub index: Option<AgentValue>,

    pub model: Option<AgentValue>,

    #[serde(rename = "SERIAL_NUMBER")]
    pub serial: Option<AgentValue>,

    pub firmware: Option<AgentValue>,
    pub size: Option<AgentValue>,

    #[serde(rename = "PHYSICAL_DISK_STATUS")]
    pub status: Option<AgentValue>,

    #[serde(rename = "MEDIA_TYPE")]
    pub media: Option<AgentValue>,

    #[serde(rename = "INTERFACE_TYPE")]
    pub interface: Option<AgentValue>,

    #[serde(rename = "NUMBER_OF_PARTITION")]
    pub num_part: Option<AgentValue>,

    #[serde(rename = "PARTITION_Lst")]
    pub partition: Option<Vec<PartitionPayload>>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PartitionPayload {
    pub name: Option<AgentValue>,
    pub size: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct NetworkPayload {
    pub name: Option<AgentValue>,
    pub ipv4: Option<AgentValue>,
    pub ipv6: Option<AgentValue>,

    #[serde(rename = "MAC_ADDRESS")]
    pub mac: Option<AgentValue>,

    pub mtu: Option<AgentValue>,
    pub upload: Option<AgentValue>,
    pub download: Option<AgentValue>,

    #[serde(rename = "INET_CARD")]
    pub card: Option<AgentValue>,

    #[serde(rename = "INET_CARD_SPEED")]
    pub config_speed: Option<AgentValue>,

    pub ssid: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ProcessPayload {
    pub name: Option<AgentValue>,
    pub cpu: Option<AgentValue>,
    pub memory: Option<AgentValue>,
    pub run_time: Option<AgentValue>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BatteryPayload {
    pub percentage: Option<AgentValue>,
    pub power_plugged: Option<AgentValue>,
}

