///
/// Keep all type for json serialize formating
/// 

use serde::Serialize;
use serde_with::skip_serializing_none;
use crate::scheduler::AgentValue;

/// Main body
#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct InfoPayload {
    pub agent_version: Option<AgentValue>,
    pub time_stamp: Option<AgentValue>,
    pub general: Option<GeneralPayload>,
    pub machine: Option<MachinePayload>,
    pub motherboard: Option<MotherboardPayload>,
    pub bios: Option<BiosPayload>,
    pub os: Option<OsPayload>,
    pub cpu: Option<CpuPayload>,
    pub ram: Option<RamPayload>,
    pub swap: Option<SwapPayload>,
    pub gpu: Option<Vec<GpuPayload>>,
    pub disk: Option<DiskPayload>,
    pub network: Option<Vec<NetworkPayload>>,
    pub process_count: Option<AgentValue>,
    pub top_process: Option<Vec<ProcessPayload>>,
    // pub all_process: Option<Vec<ProcessPayload>>,
    pub battery: Option<BatteryPayload>,
    pub software: Option<Vec<SoftwarePayload>>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct GeneralPayload {
    pub host: Option<AgentValue>,
    pub boot_time: Option<AgentValue>,
    pub run_time: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct MachinePayload {
    pub serial: Option<AgentValue>,
    pub architecture: Option<AgentValue>,
    pub producer: Option<AgentValue>,
    pub model: Option<AgentValue>,
    pub r#type: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct MotherboardPayload {
    pub name: Option<AgentValue>,
    pub serial: Option<AgentValue>,
    pub tempe: Option<AgentValue>,
    pub cpu_slot: Option<AgentValue>,
    pub ram_slot: Option<AgentValue>,
    pub gpu_slot: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct BiosPayload {
    pub version: Option<AgentValue>,
    pub vendor: Option<AgentValue>,
    pub secure_boot: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct OsPayload {
    pub distro: Option<AgentValue>,
    pub name: Option<AgentValue>,
    pub version: Option<AgentValue>,
    pub kernel: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct CpuPayload {
    pub name: Option<AgentValue>,
    pub core: Option<AgentValue>,
    pub usage: Option<AgentValue>,
    pub frequency: Option<AgentValue>,
    pub temperature: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct RamPayload {
    pub total: Option<AgentValue>,
    pub usage: Option<AgentValue>,
    pub physical: Option<Vec<RamPhysical>>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct RamPhysical {
    pub name: Option<AgentValue>,
    pub serial: Option<AgentValue>,
    pub r#type: Option<AgentValue>,
    pub config_speed: Option<AgentValue>,
    pub size: Option<AgentValue>,
    pub bank: Option<AgentValue>,
    pub form_factor: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct SwapPayload {
    pub total: Option<AgentValue>,
    pub usage: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct GpuPayload {
    pub name: Option<AgentValue>,
    pub driver: Option<AgentValue>,
    pub utilization: Option<AgentValue>,
    pub temperature: Option<AgentValue>,
    pub frequency: Option<AgentValue>,
    pub vram_total: Option<AgentValue>,
    pub vram_usage: Option<AgentValue>,
    pub max_clock: Option<AgentValue>,
    pub serial: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct DiskPayload {
    pub logical: Option<Vec<LogicalDiskPayload>>,
    pub physical: Option<Vec<PhysicalDiskPayload>>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct LogicalDiskPayload {
    pub name: Option<AgentValue>,
    pub file_system: Option<AgentValue>,
    pub mount_point: Option<AgentValue>,
    pub removable: Option<AgentValue>,
    pub total: Option<AgentValue>,
    pub used: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct PhysicalDiskPayload {
    pub drive: Option<AgentValue>,
    pub index: Option<AgentValue>,
    pub model: Option<AgentValue>,
    pub serial: Option<AgentValue>,
    pub firmware: Option<AgentValue>,
    pub size: Option<AgentValue>,
    pub status: Option<AgentValue>,
    pub media: Option<AgentValue>,
    pub interface: Option<AgentValue>,
    pub parted: Option<AgentValue>,
    pub partition: Option<Vec<Partition>>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct Partition {
    pub name: Option<AgentValue>,
    pub size: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct NetworkPayload {
    pub name: Option<AgentValue>,
    pub ipv4: Option<AgentValue>,
    pub ipv6: Option<AgentValue>,
    pub mac: Option<AgentValue>,
    pub mtu: Option<AgentValue>,
    pub upload: Option<AgentValue>,
    pub download: Option<AgentValue>,
    pub card: Option<AgentValue>,
    pub config_speed: Option<AgentValue>,
    pub ssid: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct ProcessPayload {
    pub name: Option<AgentValue>,
    pub cpu: Option<AgentValue>,
    pub memory: Option<AgentValue>,
    pub runtime: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct BatteryPayload {
    pub percentage: Option<AgentValue>,
    pub is_plugged_in: Option<AgentValue>,
}

#[derive(Debug, Default, Serialize)]
#[skip_serializing_none]
pub struct SoftwarePayload {
    pub name: Option<AgentValue>,
    pub version: Option<AgentValue>,
    pub source: Option<AgentValue>,
    pub size: Option<AgentValue>,
    pub install_date: Option<AgentValue>,
}